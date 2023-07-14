pub struct PowerOfTwo(u128);

impl PowerOfTwo {
    pub fn parse(number: u128) -> Result<Self, String> {
        if number < 1 || number & (number - 1) != 0 {
            return Err(format!("{} is not a valid power of two", number));
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
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<u16> for PowerOfTwo {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<u32> for PowerOfTwo {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<u64> for PowerOfTwo {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<u128> for PowerOfTwo {
    type Error = String;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<usize> for PowerOfTwo {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::parse(value as u128)
    }
}

impl TryFrom<i8> for PowerOfTwo {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(format!("{} is not a valid power of two", value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<i16> for PowerOfTwo {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(format!("{} is not a valid power of two", value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<i32> for PowerOfTwo {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(format!("{} is not a valid power of two", value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<i64> for PowerOfTwo {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(format!("{} is not a valid power of two", value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<i128> for PowerOfTwo {
    type Error = String;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(format!("{} is not a valid power of two", value));
        }

        Self::parse(value as u128)
    }
}

impl TryFrom<isize> for PowerOfTwo {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(format!("{} is not a valid power of two", value));
        }

        Self::parse(value as u128)
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
}
