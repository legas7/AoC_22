#[cfg(windows)]
pub const NEWLINE: &str = "\r\n";

#[cfg(unix)]
pub const NEWLINE: &str = "\n";