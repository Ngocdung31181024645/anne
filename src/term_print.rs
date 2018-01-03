#[macro_export]
macro_rules! log {
	// stdout
	(o => $pre:expr) => (println!("[{}] ", $pre));
	(o => $pre:expr, $fmt:expr) => (println!("[{}] {}", $pre, $fmt));
	(o => $pre:expr, $fmt:expr, $($arg:tt)*) => (println!(concat!("[{}] ", $fmt), $pre, $($arg)*));
	// stderr
	(e => $pre:expr) => (eprintln!("[{}]", $pre));
	(e => $pre:expr, $fmt:expr) => (eprintln!("[{}] {}", $pre, $fmt));
	(e => $pre:expr, $fmt:expr, $($arg:tt)*) => (eprintln!(concat!("[{}] ", $fmt), $pre, $($arg)*));
}

#[macro_export]
macro_rules! info {
	($fmt:expr) => (log!(o => "info", $fmt));
	($fmt:expr, $($arg:tt)*) => (log!(o => "info", $fmt, $($arg)*));
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
	($fmt:expr) => (log!(o => "debug", $fmt));
	($fmt:expr, $($arg:tt)*) => (log!(o => "debug", $fmt, $($arg)*));
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
	($fmt:expr) => ();
	($fmt:expr, $($arg:tt)*) => ();
}

#[macro_export]
macro_rules! warn {
	($fmt:expr) => (log!(e => "warn", $fmt));
	($fmt:expr, $($arg:tt)*) => (log!(e => "warn", $fmt, $($arg)*));
}

#[macro_export]
macro_rules! error {
	($fmt:expr) => (log!(e => "error", $fmt));
	($fmt:expr, $($arg:tt)*) => (log!(e => "error", $fmt, $($arg)*));
}
