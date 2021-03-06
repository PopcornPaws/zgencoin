use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

impl<const N: usize> Serialize for Hash<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
        //serializer.serialize_bytes(&self.0)
    }
}

impl<'de, const N: usize> Deserialize<'de> for Hash<N> {
    fn deserialize<D>(deserializer: D) -> Result<Hash<N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HashVisitor::<N>)
        //deserializer.deserialize_bytes(HashVisitor::<N>)
    }
}

struct HashVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for HashVisitor<N> {
    type Value = Hash<N>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a String slice")
    }

    //fn visit_bytes<T: serde::de::Error>(self, value: &[u8]) -> Result<Self::Value, T> {
    //    let result = Self::Value::try_from(value)
    //        .map_err(|e| T::custom(format!("Deserialization error: {:?}", e)))?;

    //    Ok(result)
    //}

    fn visit_str<T: serde::de::Error>(self, value: &str) -> Result<Self::Value, T> {
        let result = Self::Value::try_from_str(value)
            .map_err(|e| T::custom(format!("Deserialization error: {:?}", e)))?;

        Ok(result)
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
    fn serde_tests() {
        let hash = Hash::<4>::new([0, 2, 0xfd, 0xa0]);
        //let expected = "0002fda0";
        //serde_test::assert_tokens(&hash, &[serde_test::Token::String(expected)]);
        let expected = &[0, 2, 0xfd, 0xa0];
        serde_test::assert_tokens(&hash, &[serde_test::Token::BorrowedBytes(expected)]);

        let hash = Hash::<8>::new([0xff, 0xd9, 0xcc, 7, 0, 2, 0xfd, 0xa0]);
        //let expected = "ffd9cc070002fda0";
        //serde_test::assert_tokens(&hash, &[serde_test::Token::String(expected)]);
        let expected = &[0xff, 0xd9, 0xcc, 7, 0, 2, 0xfd, 0xa0];
        serde_test::assert_tokens(&hash, &[serde_test::Token::BorrowedBytes(expected)]);
    }

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
