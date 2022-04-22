#![no_std]

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        aidoku::std::format(core::format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        aidoku::std::print("");
    }};
    ($($arg:tt)*) => {{
        let string = aidoku::std::format(core::format_args!($($arg)*));
        aidoku::std::print(&(string));
    }};
}
