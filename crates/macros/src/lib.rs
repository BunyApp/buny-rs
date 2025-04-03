#![no_std]

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        buny::std::format(core::format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        buny::std::print("");
    }};
    ($($arg:tt)*) => {{
        let string = buny::std::format(core::format_args!($($arg)*));
        buny::std::print(&(string));
    }};
}
