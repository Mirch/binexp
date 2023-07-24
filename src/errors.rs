use thiserror::Error;

/// Errors that can occur when parsing a number into a power of two.
#[derive(Debug, Error)]
pub enum PowerOfTwoError {
    // ======================================================
    // Errors for parsing a number into a power of two
    // ======================================================
    #[error("{0} is not a valid power of two")]
    InvalidPowerOfTwo(u128),
    // ======================================================
    // Errors for conversion from integer types to PowerOfTwo
    // ======================================================
    #[error(
        "{0} is not a valid input for conversion. Provide an integer greater than or equal to 1."
    )]
    InvalidInputi8(i8),
    #[error(
        "{0} is not a valid input for conversion. Provide an integer greater than or equal to 1."
    )]
    InvalidInputi16(i16),
    #[error(
        "{0} is not a valid input for conversion. Provide an integer greater than or equal to 1."
    )]
    InvalidInputi32(i32),
    #[error(
        "{0} is not a valid input for conversion. Provide an integer greater than or equal to 1."
    )]
    InvalidInputi64(i64),
    #[error(
        "{0} is not a valid input for conversion. Provide an integer greater than or equal to 1."
    )]
    InvalidInputi128(i128),
    #[error(
        "{0} is not a valid input for conversion. Provide an integer greater than or equal to 1."
    )]
    InvalidInputisize(isize),
}
