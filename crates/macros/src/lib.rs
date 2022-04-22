#![no_std]
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        aidoku::std::format(core::format_args!($($arg)*))
    }}
}