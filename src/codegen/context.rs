//! Context for Extrusion
//!
//! This deserves some love, but not now. It's used in code generation. It should be
//! renamed StringContext, and all the code gen stuff should be represented in terms
//! of Extrude. I think anyway.
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    ops::AddAssign,
    ops::Deref,
    path::{Path, PathBuf},
    process,
};

use diff;
use log::{debug, error, trace, warn};
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::codegen::{
    BadnessSnafu, CodeGenError, ContextNoPathSnafu, ContextPathBeingStubbornSnafu, Error,
    FileCreateSnafu, FileWriteSnafu, Result, SerdeJsonBombedSnafu, SpawnRustfmtSnafu, VERSION,
};

const MAGIC: &str = "";

#[derive(Deserialize, Serialize)]
struct ContextHeader {
    magic: String,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ContextDirective {
    magic: String,
    kind: DirectiveKind,
}

#[derive(Debug, Deserialize, Serialize)]
enum DirectiveKind {
    IgnoreBlockBegin(IgnoreBlockBegin),
    IgnoreBlockEnd,
    CriticalBlockBegin(CriticalBlockBegin),
    CriticalBlockEnd { tag: String },
}

#[derive(Debug, Deserialize, Serialize)]
struct CriticalBlockBegin {
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_uber: Option<bool>,
}

impl CriticalBlockBegin {
    fn is_uber(&self) -> bool {
        if let Some(b) = self.is_uber {
            match b {
                true => true,
                false => false,
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct IgnoreBlockBegin {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_uber: Option<bool>,
}

impl IgnoreBlockBegin {
    fn is_uber(&self) -> bool {
        if let Some(b) = self.is_uber {
            match b {
                true => true,
                false => false,
            }
        } else {
            false
        }
    }
}

/// A Context for writing output
/// The general idea is that you pass this around and write to it. When you are
/// done it commits itself to a the file at the passed in path.
#[derive(Debug)]
pub struct Context {
    path: Option<PathBuf>,
    context: CachingContext,
    our_file: bool,
    have_magic: bool,
    magic_string: String,
    ignore_ignore: bool,
}

impl Context {
    pub fn new<P: AsRef<Path>>(path: P, ignore_ignore: bool) -> Result<Self> {
        let mut new = Self {
            path: Some(path.as_ref().into()),
            context: CachingContext::new(),
            our_file: false,
            have_magic: false,
            magic_string: serde_json::to_string(&ContextHeader {
                magic: MAGIC.to_owned(),
                version: VERSION.to_owned(),
            })
            .context(SerdeJsonBombedSnafu)?,
            ignore_ignore,
        };

        new.validate_output_file()?;

        Ok(new)
    }

    fn validate_output_file(&mut self) -> Result<()> {
        ensure!(self.path.is_some(), ContextNoPathSnafu);
        let path = self.path.clone().unwrap();

        debug!("✅ validating output path: {}", path.display());

        // Check if the file exists.
        if path.exists() {
            // If so, did we generate it?
            let file = File::open(&path).context(ContextPathBeingStubbornSnafu { path: &path })?;
            if file.metadata().context(BadnessSnafu)?.len() > 0 {
                let mut line = String::new();
                let mut reader = BufReader::new(file);

                // Having the header as the first line in the file makes the
                // documentation ugly. So I'm going to move it to the first
                // regular comment line.
                //
                // So, first we throw away all the lines that start with "//!".
                reader.read_line(&mut line).context(BadnessSnafu)?;
                while line.starts_with("//!") {
                    line.clear();
                    reader.read_line(&mut line).context(BadnessSnafu)?;
                }

                // Then we continue as before, except that we only strip off the
                // first two bytes.
                //
                reader.read_line(&mut line).context(BadnessSnafu)?;
                if loop {
                    if line.starts_with("//") {
                        // Strip off the first three bytes, as they should be "//!".
                        line.replace_range(..2, "");
                        match serde_json::from_str::<ContextHeader>(&line) {
                            Ok(header) => {
                                // We may want to consider doing something with the version.
                                // It would be nice to replace the header, rather than comment
                                // out the old line. Not sure how that would work exactly.
                                if header.magic != MAGIC {
                                    error!("😱 file not generated by sarzak: {}", path.display());
                                    break false;
                                } else {
                                    // Indicate that writing is safe.
                                    self.our_file = true;
                                    break true;
                                }
                            }
                            Err(_e) => {
                                line.clear();
                                reader.read_line(&mut line).context(BadnessSnafu)?;
                            }
                        }
                    } else {
                        break false;
                    }
                } {
                    self.our_file = true;
                } else {
                    error!("😱 file not generated by sarzak: {}", path.display());
                }
            }
        } else {
            self.our_file = true;
        }

        Ok(())
    }

    pub fn inter_symbol(&mut self, id: Uuid, symbol: Symbol) {
        self.context.inter_symbol(id, symbol)
    }

    pub fn exhume_symbol(&self, id: &Uuid) -> Option<&Symbol> {
        self.context.exhume_symbol(id)
    }

    pub fn increase_indent(&mut self) {
        self.context.increase_indent()
    }

    pub fn decrease_indent(&mut self) {
        self.context.decrease_indent()
    }

    pub fn begin_ignore_block(&mut self) -> Result<()> {
        self.context.begin_ignore_block()
    }

    pub fn end_ignore_block(&mut self) -> Result<()> {
        self.context.end_ignore_block()
    }

    pub fn begin_critical_block<S: AsRef<str>>(&mut self, tag: S) -> Result<()> {
        self.context.begin_critical_block(tag)
    }

    pub fn end_critical_block<S: AsRef<str>>(&mut self, tag: S) -> Result<()> {
        self.context.end_critical_block(tag)
    }

    pub fn writeln<S: AsRef<str>>(&mut self, data: S) {
        if !self.have_magic {
            if !data.as_ref().starts_with("//!") && !data.as_ref().starts_with("//") {
                self.context.writeln(format!("// {}", self.magic_string));
                self.have_magic = true;
            }
        }
        self.context.writeln(data);
    }

    pub fn write<S: AsRef<str>>(&mut self, data: S) {
        self.context.write(data);
    }

    pub fn format(&self, buffer: &String) -> Result<String> {
        trace!("formatting output buffer");
        let mut file = NamedTempFile::new().context(BadnessSnafu)?;
        file.write_all(buffer.as_bytes()).context(BadnessSnafu)?;

        let child = process::Command::new("rustfmt")
            // .arg(&path.to_str().expect("this is a pain in the dick"))
            .args(["--emit", "stdout", &file.path().to_string_lossy()])
            .stdout(process::Stdio::piped())
            .spawn()
            .context(SpawnRustfmtSnafu)?;

        let output = child.wait_with_output().context(BadnessSnafu)?;

        if output.status.success() {
            let buffer = String::from_utf8_lossy(&output.stdout);

            // Some junk get's appended to the top of stdout.
            let mut iter = buffer.splitn(3, '\n');
            iter.next();
            iter.next();

            Ok(iter.next().unwrap().into())
        } else {
            eprintln!("Failure parsing input file, reproduced below...");
            eprintln!("{}", buffer);

            Err(Error(CodeGenError::RustFmt {
                exit_code: output.status.code(),
            }))
        }
    }

    /// My god, this is a big, ugly son-of-a-bitch.
    ///
    pub fn commit(mut self) -> Result<()> {
        if self.our_file {
            match self.path {
                Some(ref path) => {
                    // It looks like we are going to write this thing. Ensure that
                    // we have added magic.
                    if !self.have_magic {
                        self.context.writeln(format!("// {}", self.magic_string));
                        self.have_magic = true;
                    }

                    if path.exists() {
                        // Get the text from the file.
                        let mut orig = String::new();
                        File::open(&path)
                            .context(ContextPathBeingStubbornSnafu { path: &path })?
                            .read_to_string(&mut orig)
                            .context(BadnessSnafu)?;

                        // Format our buffer
                        let formatted = self.format(&self.context.buffer)?;
                        let (diffed, dirty) =
                            Context::merge_sources(&orig, &formatted, self.ignore_ignore);

                        if dirty {
                            // Format the new buffer, because there is going to be junk left over (
                            // spaces, and blank lines, and whatnot).
                            let formatted = self.format(&diffed)?;

                            File::create(path)
                                .context(FileCreateSnafu { path })?
                                .write_all(formatted.as_bytes())
                                .context(FileWriteSnafu { path })?;

                            debug!("writing diff");
                        } else {
                            debug!("no changes");
                        }

                        Ok(())
                    } else {
                        // Format our buffer
                        let formatted = self.format(&self.context.buffer)?;

                        File::create(path)
                            .context(FileCreateSnafu { path })?
                            .write_all(formatted.as_bytes())
                            .context(FileWriteSnafu { path })?;

                        debug!("writing output");
                        Ok(())
                    }
                }
                None => Err(Error(CodeGenError::ContextNoPath)),
            }
        } else {
            warn!("not writing output due to unknown contents of destination file");
            Ok(())
        }
    }

    fn merge_sources(orig: &String, formatted: &String, ignore_ignore: bool) -> (String, bool) {
        let mut dirty = false;
        let mut ignoring = false;
        let mut delete_ignore = false;
        let mut crit = false;
        let mut ignore_crit = false;
        let mut delete_crit = false;
        let mut delete_last_crit = false; // There's surely a better way to do this.
        let mut delete_crit_tag = String::new();
        let mut diffed = String::new();
        // Produce the diff. Lines on the left (from the file) are inserted behind
        // a comment. Otherwise, we just write the line.
        // Unless, there is an ignore directive, then we ... ignore.
        for diff in diff::lines(orig.as_str(), formatted.as_str()) {
            match diff {
                diff::Result::Left(orig) => {
                    // Pass through blank lines
                    if orig == "" {
                        diffed = diffed + orig + "\n"
                    } else {
                        // I'm not sure where to put this. It seems like the right
                        // place is outside of other directives. So I'll try that
                        // first.
                        if let Some(directive) = parse_directive(orig) {
                            trace!("found directive in orig: {:?}", directive);

                            match directive.kind {
                                DirectiveKind::CriticalBlockBegin(cb) => {
                                    // If it's uber, leave it be.
                                    if cb.is_uber() {
                                        ignore_crit = true;
                                    } else {
                                        // We need to be on the lookout for the closing
                                        // magic. Let's create what that looks like here.
                                        if let Ok(crit_end_magic) =
                                            serde_json::to_string(&ContextDirective {
                                                magic: MAGIC.to_owned(),
                                                kind: DirectiveKind::CriticalBlockEnd {
                                                    tag: cb.tag.clone(),
                                                },
                                            })
                                        {
                                            if let None = formatted.find(crit_end_magic.as_str()) {
                                                // This critical section isn't being written,
                                                // so remove it from the source file.
                                                delete_crit = true;
                                                delete_last_crit = false;
                                                delete_crit_tag = cb.tag;
                                                dirty = true;
                                            }
                                        }
                                    }
                                }
                                DirectiveKind::CriticalBlockEnd { tag } => {
                                    if delete_crit && tag == delete_crit_tag {
                                        delete_last_crit = true;
                                        ignore_crit = false;
                                    }
                                }
                                _ => {}
                            }
                        }
                        // We specifically want to ignore any changes in this section.
                        // This is primarily used to ignore changes to generated code,
                        // e.g., generated tests.
                        if ignoring {
                            diffed = diffed + orig + "\n"
                        } else if delete_ignore || delete_crit {
                            dirty = true;

                            if delete_last_crit {
                                delete_crit = false;
                            }
                        } else if crit {
                            // If this is a critical section, comment out the offending
                            // lines. Otherwise, pass them through.
                            dirty = true;
                            // Already commented out -- don't cons comments
                            if orig.contains("⚡️") {
                                diffed = diffed + orig + "\n"
                            } else {
                                // Comment out the line.
                                diffed = diffed + "// " + orig + " //⚡️" + "\n"
                            }
                        } else {
                            // Allow the end user to add code outside of critical sections.
                            diffed = diffed + orig + "\n"
                        }
                    }
                }
                diff::Result::Both(both, _) => {
                    // Look for embedded processing directives.
                    //
                    // Processing directives come from the original file. We only
                    // consider them here because they are only "live" when they
                    // are also in the input buffer. If they aren't in the buffer,
                    // then what's in the original file is outdated and will be
                    // commented out.
                    if let Some(directive) = parse_directive(both) {
                        trace!("found directive in both: {:?}", directive);

                        match directive.kind {
                            DirectiveKind::IgnoreBlockBegin(i) => {
                                ignoring = {
                                    if i.is_uber() {
                                        delete_ignore = false;
                                        true
                                    } else {
                                        if ignore_ignore {
                                            debug!("ignore_ignore");
                                            delete_ignore = true;
                                            false
                                        } else {
                                            true
                                        }
                                    }
                                };
                            }
                            DirectiveKind::IgnoreBlockEnd => {
                                delete_ignore = false;
                                ignoring = false;
                            }
                            DirectiveKind::CriticalBlockBegin(_) => {
                                crit = true;
                            }
                            DirectiveKind::CriticalBlockEnd { tag: _ } => {
                                crit = false;
                                ignore_crit = false;
                            }
                        }
                    }

                    // Output the line that occurs in both buffers
                    diffed = diffed + both + "\n";
                }
                diff::Result::Right(new) => {
                    if !ignoring && !ignore_crit {
                        dirty = true;

                        // Output the new line.
                        diffed = diffed + new + "\n";
                    }
                }
            }
        }

        (diffed, dirty)
    }

    pub fn to_string(self) -> String {
        self.context.buffer
    }
}

impl AddAssign for Context {
    fn add_assign(&mut self, rhs: Self) {
        // We copy the buffer's contents. We do so using our write methods.
        // Doing so ensures indentation is maintained. It's not without it's
        // problems though.
        let rhs = rhs.to_string();
        // We want to avoid adding an extra newline at the end of iteration.
        let mut iter = rhs.split('\n').peekable();
        loop {
            if let Some(l) = iter.next() {
                if iter.peek() == None {
                    // Somehow this happens. Sometimes. Not happy about it.
                    if l.len() > 0 {
                        self.writeln(l);
                    }
                } else {
                    self.writeln(l);
                }
            } else {
                break;
            }
        }

        // Now we need to copy the symbol table
    }
}

impl From<CachingContext> for Context {
    fn from(value: CachingContext) -> Self {
        Context {
            path: None,
            context: value,
            our_file: false,
            have_magic: false,
            magic_string: "no_magic 😭".to_owned(),
            ignore_ignore: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub value: String,
    pub value_type: String,
    pub is_reference: bool,
}

#[derive(Debug)]
pub struct CachingContext {
    buffer: String,
    symbol_table: HashMap<Uuid, Symbol>,
    indent: u8,
    in_critical_block: bool,
    in_ignored_block: bool,
}

/// A Context that looks like a String
///
/// This context does not write itself to a file. It can write itself to a String.
/// Otherwise it's like a regular context, with writing and indenting.
impl CachingContext {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            symbol_table: HashMap::new(),
            indent: 0,
            in_critical_block: false,
            in_ignored_block: false,
        }
    }

    pub fn inter_symbol(&mut self, id: Uuid, symbol: Symbol) {
        trace!("interring symbol {} as {}", symbol.value, id);
        self.symbol_table.insert(id, symbol);
    }

    pub fn exhume_symbol(&self, id: &Uuid) -> Option<&Symbol> {
        trace!("exhuming symbol at {}", id);
        self.symbol_table.get(id)
    }

    pub fn increase_indent(&mut self) {
        self.indent += 1;
    }

    pub fn decrease_indent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }

    pub fn begin_ignore_block(&mut self) -> Result<()> {
        if self.in_ignored_block {
            panic!("Started new ignore block, with existing block still active.");
        }

        self.in_ignored_block = true;
        self.writeln(format!(
            "// {}",
            serde_json::to_string(&ContextDirective {
                magic: MAGIC.to_owned(),
                kind: DirectiveKind::IgnoreBlockBegin(IgnoreBlockBegin { is_uber: None }),
            })
            .context(SerdeJsonBombedSnafu)?
        ));

        Ok(())
    }

    pub fn end_ignore_block(&mut self) -> Result<()> {
        if !self.in_ignored_block {
            panic!("Not inside of an ignore block.");
        }

        self.in_ignored_block = false;
        self.writeln(format!(
            "// {}",
            serde_json::to_string(&ContextDirective {
                magic: MAGIC.to_owned(),
                kind: DirectiveKind::IgnoreBlockEnd,
            })
            .context(SerdeJsonBombedSnafu)?
        ));

        Ok(())
    }

    pub fn begin_critical_block<S: AsRef<str>>(&mut self, tag: S) -> Result<()> {
        if self.in_critical_block {
            panic!("Started new critical block, with existing block still active.");
        }

        self.in_critical_block = true;
        self.writeln(format!(
            "// {}",
            serde_json::to_string(&ContextDirective {
                magic: MAGIC.to_owned(),
                kind: DirectiveKind::CriticalBlockBegin(CriticalBlockBegin {
                    tag: tag.as_ref().to_owned(),
                    is_uber: None
                }),
            })
            .context(SerdeJsonBombedSnafu)?
        ));

        Ok(())
    }

    pub fn end_critical_block<S: AsRef<str>>(&mut self, tag: S) -> Result<()> {
        if !self.in_critical_block {
            panic!("Not inside of a critical block.");
        }

        self.in_critical_block = false;
        self.writeln(format!(
            "// {}",
            serde_json::to_string(&ContextDirective {
                magic: MAGIC.to_owned(),
                kind: DirectiveKind::CriticalBlockEnd {
                    tag: tag.as_ref().to_owned()
                },
            })
            .context(SerdeJsonBombedSnafu)?
        ));

        Ok(())
    }

    pub fn writeln<S: AsRef<str>>(&mut self, data: S) {
        self.write(data);
        // I figured out what this is about. It's to get rid of spaces at the
        // end of comment lines.  I need a new solution.
        // Chop off trailing whitespace -- this seems really wasteful.
        // self.buffer = self.buffer.trim_end().to_owned();
        self.buffer += "\n";
    }

    pub fn write<S: AsRef<str>>(&mut self, data: S) {
        let mut indent = 0;
        while indent < self.indent {
            self.buffer += "    ";
            indent += 1;
        }

        self.buffer += data.as_ref();
    }

    pub fn insert_prefix<S: AsRef<str>>(&mut self, prefix: S) {
        let rhs = self.buffer.clone();
        self.buffer = String::new();

        let mut iter = rhs.split('\n').peekable();
        loop {
            if let Some(l) = iter.next() {
                if iter.peek() == None {
                    // Somehow this happens. Sometimes. Not happy about it.
                    // Looking at the code coverage, this actually never happens
                    // any longer. I think I fixed it elsewhere.
                    // I just tried to remove it, and I'd have to spend too much
                    // time digging into it to be worthwhile a this moment.
                    if l.len() > 0 {
                        let s = String::from(prefix.as_ref());
                        self.writeln(s + l);
                    }
                } else {
                    let s = String::from(prefix.as_ref());
                    self.writeln(s + l);
                }
            } else {
                break;
            }
        }
    }

    pub fn commit(self) -> String {
        self.buffer
    }
}

impl Deref for CachingContext {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl AddAssign for CachingContext {
    fn add_assign(&mut self, rhs: Self) {
        // First we need to copy over the symbol table.
        for (k, v) in &rhs.symbol_table {
            self.symbol_table.insert(*k, v.clone());
        }

        // Next we copy the buffer's contents. We do so using our write methods.
        // Doing so ensures indentation is maintained. It's not without it's
        // problems though.
        let rhs = rhs.commit();
        // We want to avoid adding an extra newline at the end of iteration.
        let mut iter = rhs.split('\n').peekable();
        loop {
            if let Some(l) = iter.next() {
                if iter.peek() == None {
                    // Somehow this happens. Sometimes. Not happy about it.
                    if l.len() > 0 {
                        self.writeln(l);
                    }
                } else {
                    self.writeln(l);
                }
            } else {
                break;
            }
        }
    }
}

fn parse_directive(line: &str) -> Option<ContextDirective> {
    let mut test = String::from(line);
    test = test.trim_start().to_owned();
    if test.starts_with("//") {
        test.replace_range(..2, "");
        if let Ok(directive) = serde_json::from_str::<ContextDirective>(test.as_str()) {
            Some(directive)
        } else {
            None
        }
    } else {
        None
    }
}
