#[cfg(windows)]
pub const NEWLINE: &str = "\r\n";

#[cfg(linux)]
pub const NEWLINE: &str = "\n";
