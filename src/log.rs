#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        {
            let message = format!($($arg)*);
            let thread_name = std::thread::current().name().unwrap_or_default().to_string();
            let extended_message = format!("[{}] {}", thread_name, message);
            println!("{}", extended_message);
        }
    );
}
