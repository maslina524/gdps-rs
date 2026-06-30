pub const RED: &str = "\x1b[1;31m";
pub const BLUE: &str = "\x1b[1;34m";
pub const WHITE: &str = "\x1b[1;40m";
pub const ESC: &str = "\x1b[0m";

#[macro_export]
macro_rules! info {
    ($($tt:tt)*) => {
        std::print!("{}| {}INFO{} |{} ", $crate::log::WHITE, $crate::log::BLUE, $crate::log::ESC, $crate::log::WHITE);
        std::println!($($tt)*);
    };
}

#[macro_export]
macro_rules! fail {
    ($($tt:tt)*) => {
        std::print!("{}| {}FAIL{} |{} [{}:{}:{}] ", $crate::log::WHITE, $crate::log::RED, $crate::log::WHITE, $crate::log::ESC, std::file!(), std::line!(), std::column!());
        std::println!($($tt)*);
    };
}