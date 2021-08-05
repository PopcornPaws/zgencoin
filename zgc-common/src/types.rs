use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hash<const N: usize>([u8; N]);

impl<const N: usize> Hash<N> {
    pub fn try_from_str(string: &str) -> Result<Self, String> {
        let trimmed_string = string.trim_start_matches("0x");

        if trimmed_string.len() != 2 * N {
            return Err(format!(
                "input length was {}, expected {}",
                trimmed_string.len(),
                2 * N
            ));
        }

        let mut array = [0_u8; N];

        array
            .iter_mut()
            .enumerate()
            .try_for_each(|(i, byte)| -> Result<(), String> {
                let str_window = &trimmed_string[2 * i..2 * i + 2];
                let parsed_byte = u8::from_str_radix(str_window, 16)
                    .map_err(|e| format!("cannot parse into hexadecimal: {}", e))?;

                *byte = parsed_byte;
                Ok(())
            })?;

        Ok(Self(array))
    }

    pub fn to_string(&self) -> String {
        self.0.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    pub fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    pub fn zero() -> Self {
        Self([0_u8; N])
    }
}

impl<const N: usize> Default for Hash<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> TryFrom<&[u8]> for Hash<N> {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let result: [u8; N] = bytes
            .try_into()
            .map_err(|_| format!("input (&[u8]) has {}, expected {}", bytes.len(), N))?;

        Ok(Self(result))
    }
}

#[cfg(test)]
mod test {
    use super::Hash;

    #[test]
    fn address() {
        let address_str = "0123456789abcdeffedcba9876543210aabbccdd";
        let address = Hash::<20>::try_from_str(address_str).expect("failed to parse string");

        assert_eq!(address.to_string(), address_str);

        let address_string_0x = String::from("0x") + address_str;
        let address = Hash::<20>::try_from_str(&address_string_0x).expect("failed to parse string");
        assert_eq!(address.to_string(), address_str);
    }

    #[test]
    fn from_invalid_string() {
        let invalid_len_string = "563fdea";
        let result = Hash::<4>::try_from_str(invalid_len_string);
        assert_eq!(result, Err("input length was 7, expected 8".to_string()));

        let invalid_hex_string = "563fgdea";
        let result = Hash::<4>::try_from_str(invalid_hex_string);
        assert_eq!(
            result,
            Err("cannot parse into hexadecimal: invalid digit found in string".to_string())
        );
    }
}
