#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        let message = colored::Colorize::bright_black(format!("[INFO] {}", format!($($arg)*)).as_str());
        println!("{}", message);
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        let message = colored::Colorize::yellow(format!("[WARN] {}", format!($($arg)*)).as_str());
        println!("{}", message);
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        let message = colored::Colorize::red(format!("[ERRO] {}", format!($($arg)*)).as_str());
        println!("{}", message);
    })
}
