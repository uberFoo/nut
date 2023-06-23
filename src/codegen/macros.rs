//! Macros to make code generation nicer. 😉
//!

/// Emit a String with a line break added
///
/// This macro makes it a tad bit easier to write to a [Context][c].
///
/// [c]: crate::codegen::Context
#[macro_export]
macro_rules! emitln {
    ($context:expr, $string:expr, $($args:expr),* ,) => {
        emitln!($context, $string, $($args),*)
    };
    ($context:expr, $string:expr, $($args:expr),*) => {
        $context.writeln(format!($string, $($args),*))
    };
    ($context:expr, $string:expr) => {
        $context.writeln($string)
    };
}

/// Emit a String _without_ a line break added
///
/// This macro makes it a tad bit easier to write to a [Context][c].
///
/// [c]: crate::codegen::Context
#[macro_export]
macro_rules! emit {
    ($context:expr, $string:expr, $($args:expr),* ,) => {
        emit!($context, $string, $($args),*)
    };
    ($context:expr, $string:expr, $($args:expr),*) => {
        $context.write(format!($string, $($args),*))
    };
    ($context:expr, $string:expr) => {
        $context.write($string)
    };
}

/// Emit a begin critical section block
///
/// This is a context method that is used to remove and add tagged sections
/// of code.
#[macro_export]
macro_rules! begin_crit {
    ($context:expr, $string:expr, $($args:expr),* ,) => {
        begin_crit!($context, $string, $($args),*)
    };
    ($context:expr, $string:expr, $($args:expr),*) => {
        $context.begin_critical_block(format!($string, $($args),*))
    };
    ($context:expr, $string:expr) => {
        $context.begin_critical_block($string)
    }
}

/// Emit an end critical section block
///
/// This is a context method that is used to remove and add tagged sections
/// of code.
#[macro_export]
macro_rules! end_crit {
    ($context:expr, $string:expr, $($args:expr),* ,) => {
        end_crit!($context, $string, $($args),*)
    };
    ($context:expr, $string:expr, $($args:expr),*) => {
        $context.end_critical_block(format!($string, $($args),*))
    };
    ($context:expr, $string:expr) => {
        $context.end_critical_block($string)
    }
}

/// This macro prints the name of the function in which it's called.
///
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

/// This macro get's the name of the function in which it's called
///
/// It's exactly the same as function!, but it only output's the last value
/// in the module path
#[macro_export]
macro_rules! func_name {
    () => {
        function!().split("::").last().unwrap()
    };
}
