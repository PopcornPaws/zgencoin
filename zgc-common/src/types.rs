use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::convert::{TryFrom, TryInto};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    pub fn zero() -> Self {
        Self([0_u8; N])
    }

    pub fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl<const N: usize> Default for Hash<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> From<[u8; N]> for Hash<N> {
    fn from(array: [u8; N]) -> Self {
        Self(array)
    }
}

impl<const N: usize> From<&[u8; N]> for Hash<N> {
    fn from(array_ref: &[u8; N]) -> Self {
        Self(array_ref.to_owned())
    }
}

impl<const N: usize> TryFrom<&[u8]> for Hash<N> {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let result: [u8; N] = bytes
            .try_into()
            .map_err(|_| format!("input (&[u8]) has {} bytes, expected {}", bytes.len(), N))?;
        Ok(Self(result))
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
        write!(f, "a byte array with length {}", N)
    }

    //fn visit_bytes<T: serde::de::Error>(self, value: &[u8]) -> Result<Self::Value, T> {
    fn visit_str<T: serde::de::Error>(self, value: &str) -> Result<Self::Value, T> {
        let result = Self::Value::try_from_str(value)
            .map_err(|e| T::custom(format!("Deserialization error: {:?}", e)))?;
        Ok(result)
    }

    //fn visit_str<T: serde::de::Error>(self, value: &str) -> Result<Self::Value, T> {
    //    let result = Self::Value::try_from_str(value)
    //        .map_err(|e| T::custom(format!("Deserialization error: {:?}", e)))?;
    //    Ok(result)
    //}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde_tests() {
        let hash = Hash::<4>::from(&[0, 2, 0xfd, 0xa0]);
        //let expected = &[0, 2, 0xfd, 0xa0];
        //serde_test::assert_tokens(&hash, &[serde_test::Token::BorrowedBytes(expected)]);
        let expected = "0002fda0";
        serde_test::assert_tokens(&hash, &[serde_test::Token::String(expected)]);

        let hash: Hash<8> = [0xff, 0xd9, 0xcc, 7, 0, 2, 0xfd, 0xa0].into();
        //let expected = &[0xff, 0xd9, 0xcc, 7, 0, 2, 0xfd, 0xa0];
        //serde_test::assert_tokens(&hash, &[serde_test::Token::BorrowedBytes(expected)]);
        let expected = "ffd9cc070002fda0";
        serde_test::assert_tokens(&hash, &[serde_test::Token::String(expected)]);
    }

    #[test]
    fn address() {
        let address_str = "0123456789abcdeffedcba9876543210aabbccdd";
        let address =
            Hash::<20>::try_from_str(address_str).expect("failed to parse address string");
        assert_eq!(address.to_string(), address_str);

        let address_str_0x = String::from("0x") + address_str;
        let address =
            Hash::<20>::try_from_str(&address_str_0x).expect("failed to parse address string");
        assert_eq!(address.to_string(), address_str);
    }

    #[test]
    fn from_invalid_string() {
        let invalid_len_str = "563fdea";
        let hash = Hash::<4>::try_from_str(invalid_len_str);
        assert_eq!(hash, Err("input length was 7, expected 8".to_owned()));

        let invalid_hex_str = "563fgdea"; // 'gd' cannot be parsed as hex
        let hash = Hash::<4>::try_from_str(invalid_hex_str);
        assert_eq!(
            hash,
            Err("cannot parse into hexadecimal: invalid digit found in string".to_owned())
        );
    }
}
