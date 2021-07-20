use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy)]
pub struct Hash<const N: usize>([u8; N]);

impl<const N: usize> Hash<N> {
    pub fn from_str(string: &str) -> Result<Self, String> {
        let mut array = [0_u8; N];

        string
            .trim_start_matches("0x")
            .as_bytes()
            .chunks(2)
            .enumerate()
            .for_each(|(i, bytes)| {
                let parsed = u8::from_str_radix(std::str::from_utf8(bytes).unwrap(), 16)
                    .expect("input contains invalid data");
                array[i] = parsed;
            });

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

impl<const N: usize> Serialize for Hash<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, const N: usize> Deserialize<'de> for Hash<N> {
    fn deserialize<D>(deserializer: D) -> Result<Hash<N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HashVisitor::<N>)
    }
}

struct HashVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for HashVisitor<N> {
    type Value = Hash<N>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a hex string")
    }

    fn visit_str<T: serde::de::Error>(self, value: &str) -> Result<Self::Value, T> {
        let result = Self::Value::from_str(value)
            .map_err(|e| T::custom(format!("Deserialization error: {:?}", e)))?;
        Ok(result)
    }

    fn visit_string<T: serde::de::Error>(self, value: String) -> Result<Self::Value, T> {
        let result = Self::Value::from_str(value.as_str())
            .map_err(|e| T::custom(format!("Deserialization error: {:?}", e)))?;
        Ok(result)
    }
}
