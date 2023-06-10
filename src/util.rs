#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => {
        {
            eprint!("{}", "[ERROR]: ");
            eprintln!($($arg)*);
            std::process::exit(1)
        }
    };
}