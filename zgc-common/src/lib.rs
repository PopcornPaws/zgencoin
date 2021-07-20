/// Formats a byte stream into a hexadecimal `String` representation.
pub fn slice_to_string(slice: &[u8]) -> String {
    slice.iter().map(|byte| format!("{:02x}", byte)).collect()
}
