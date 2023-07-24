use crate::errors::PowerOfTwoError;

/// A type that represents a power of two. It holds a u128 internally, but can be constructed from any integer type.
/// # Examples
/// ```
/// use binexp::PowerOfTwo;
/// assert!(PowerOfTwo::try_from(1u8).is_ok());
/// assert!(PowerOfTwo::try_from(2u16).is_ok());
/// assert!(PowerOfTwo::try_from(3u32).is_err());
/// assert!(PowerOfTwo::try_from(4u64).is_ok());
/// assert!(PowerOfTwo::try_from(5u128).is_err());
/// assert!(PowerOfTwo::try_from(6usize).is_err());
/// assert!(PowerOfTwo::try_from(7i8).is_err());
/// assert!(PowerOfTwo::try_from(8i16).is_ok());
/// assert!(PowerOfTwo::try_from(9i32).is_err());
/// assert!(PowerOfTwo::try_from(10i64).is_err());
/// assert!(PowerOfTwo::try_from(11i128).is_err());
/// assert!(PowerOfTwo::try_from(12isize).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowerOfTwo(u128);

impl PowerOfTwo {
    /// Parses a number into a power of two.
    /// # Examples
    /// ```
    /// use binexp::PowerOfTwo;
    /// assert!(PowerOfTwo::parse(1).is_ok());
    /// assert!(PowerOfTwo::parse(2).is_ok());
    /// assert!(PowerOfTwo::parse(3).is_err());
    /// ```
    pub fn parse(number: u128) -> Result<Self, PowerOfTwoError> {
        if number < 1 || number & (number - 1) != 0 {
            return Err(PowerOfTwoError::InvalidPowerOfTwo(number));
        }

        Ok(Self(number))
    }
}

impl std::fmt::Display for PowerOfTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut number = self.0;
        let mut power = 0;

        while number > 1 {
            number >>= 1;
            power += 1;
        }

        write!(f, "2^{} ({})", power, self.0)
    }
}

impl TryFrom<u8> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<u16> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<u32> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<u64> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<u128> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<usize> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<i8> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(PowerOfTwoError::InvalidInputi8(value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<i16> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(PowerOfTwoError::InvalidInputi16(value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<i32> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(PowerOfTwoError::InvalidInputi32(value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<i64> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(PowerOfTwoError::InvalidInputi64(value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<i128> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(PowerOfTwoError::InvalidInputi128(value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<isize> for PowerOfTwo {
    type Error = PowerOfTwoError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(PowerOfTwoError::InvalidInputisize(value));
        }

        Self::parse(value as u128)
    }
}

impl From<PowerOfTwo> for u128 {
    fn from(value: PowerOfTwo) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn zero_is_not_a_power_of_two() {
        assert!(crate::PowerOfTwo::parse(0).is_err());
    }

    #[test]
    fn one_is_a_power_of_two() {
        assert!(crate::PowerOfTwo::parse(1).is_ok());
    }

    #[test]
    fn two_is_a_power_of_two() {
        assert!(crate::PowerOfTwo::parse(2).is_ok());
    }

    #[test]
    fn powers_of_two_are_parsed_correctly() {
        for i in 0..128 {
            let number = 1 << i;
            assert!(crate::PowerOfTwo::parse(number).is_ok());
        }
    }

    #[test]
    fn non_powers_of_two_are_not_parsed() {
        for i in 1..128 {
            let number = (1 << i) + 1;
            assert!(crate::PowerOfTwo::parse(number).is_err());
        }
    }

    #[test]
    fn integer_types_can_be_converted_to_power_of_two() {
        assert!(crate::PowerOfTwo::try_from(1u8).is_ok());
        assert!(crate::PowerOfTwo::try_from(1u16).is_ok());
        assert!(crate::PowerOfTwo::try_from(1u32).is_ok());
        assert!(crate::PowerOfTwo::try_from(1u64).is_ok());
        assert!(crate::PowerOfTwo::try_from(1u128).is_ok());
        assert!(crate::PowerOfTwo::try_from(1usize).is_ok());

        assert!(crate::PowerOfTwo::try_from(1i8).is_ok());
        assert!(crate::PowerOfTwo::try_from(1i16).is_ok());
        assert!(crate::PowerOfTwo::try_from(1i32).is_ok());
        assert!(crate::PowerOfTwo::try_from(1i64).is_ok());
        assert!(crate::PowerOfTwo::try_from(1i128).is_ok());
        assert!(crate::PowerOfTwo::try_from(1isize).is_ok());
    }

    #[test]
    fn power_of_two_can_be_converted_into_u128() {
        let power_of_two = crate::PowerOfTwo::try_from(1u8).unwrap();
        assert_eq!(u128::from(power_of_two), 1u128);
    }
}
