pub const RED: &str = "\x1b[1;31m";
pub const GREEN: &str = "\x1b[1;32m";
pub const YELLOW: &str = "\x1b[1;33m";
pub const BLUE: &str = "\x1b[1;34m";
pub const ESC: &str = "\x1b[0m";

#[macro_export]
macro_rules! raw_log {
    ($typ:literal, $color:expr, $($tt:tt)*) => {
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        std::print!("{}{}{} [{}] {}:{}     ", $color, $typ, $crate::log::ESC, now, std::file!(), std::line!());
        std::println!($($tt)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($tt:tt)*) => {
        raw_log!("INFO", $crate::log::BLUE, $($tt)*)
    };
}

#[macro_export]
macro_rules! fail {
    ($($tt:tt)*) => {
        raw_log!("FAIL", $crate::log::RED, $($tt)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($tt:tt)*) => {
        raw_log!("WARN", $crate::log::YELLOW, $($tt)*)
    };
}

#[macro_export]
macro_rules! okay {
    ($($tt:tt)*) => {
        raw_log!("OKAY", $crate::log::GREEN, $($tt)*)
    };
}