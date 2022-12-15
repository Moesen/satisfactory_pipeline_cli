use std::fmt::Error;
pub trait InfoDisplays {
    /// Named format
    fn nfmt(&self) -> Result<String, Error>;
    /// Detailed format
    fn dfmt(&self) -> Result<String, Error>;
}
