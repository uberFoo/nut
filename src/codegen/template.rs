use std::env;
use uuid::Uuid;

use crate::codegen::{emitln, CachingContext};

pub mod macros;
pub mod types;

pub(crate) fn emit_generated_code_comments() -> CachingContext {
    let mut context = CachingContext::new();

    emitln!(context, "//! # Generated Code -- edit _with care_.");
    emitln!(context, "//!");
    emitln!(
        context,
        r#"//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`"#
    );
    emitln!(
        context,
        r#"//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free"#
    );
    emitln!(context, "//! to go wild. Happy hacking!");
    emitln!(context, "//!");
    emitln!(context, "//! Use the following invocation to reproduce:");
    let _ = context.begin_ignore_block();
    emitln!(context, "//! ```shell");
    emitln!(
        context,
        "//! {}",
        env::args().fold(String::new(), |mut s, z| {
            s += " ";
            s += &z;
            s
        })
    );
    emitln!(context, "//! ```");
    let _ = context.end_ignore_block();

    context
}

/// This little function is tits
///
/// This thing is so awesome, and its purely cosmetic. Just say no to trailing
/// commas in your generated code.
pub(crate) fn create_arg_string(args: &Vec<Uuid>, context: &CachingContext) -> String {
    let mut iter = args.iter().peekable();
    let mut result = String::new();
    let mut first_time = true;
    loop {
        if let Some(id) = iter.next() {
            if first_time {
                result += ", ";
                first_time = false;
            }

            let symbol = context.exhume_symbol(&id).unwrap();
            if iter.peek() == None {
                match symbol.is_reference {
                    true => result = result + "&" + symbol.value.as_str(),
                    false => result = result + symbol.value.as_str(),
                }
            } else {
                match symbol.is_reference {
                    true => result = result + "&" + symbol.value.as_str() + ", ",
                    false => result = result + symbol.value.as_str() + ", ",
                }
            }
        } else {
            break;
        }
    }

    result
}
