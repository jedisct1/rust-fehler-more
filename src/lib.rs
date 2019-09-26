pub use fehler::*;

/// Exits a function early with an `Exception`.
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        throw!(Exception::new_adhoc($e));
    };
    ($fmt:expr, $($arg:tt)+) => {
        throw!(error!($fmt, $($arg)+));
    };
}

/// Exits a function early with an `Exception` if the condition is not satisfied.
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)+) => {
        if !($cond) {
            bail!($fmt, $($arg)+);
        }
    };
}
