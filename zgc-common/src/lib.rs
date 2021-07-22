/// Formats a byte stream into a hexadecimal `String` representation.
pub fn slice_to_string(slice: &[u8]) -> String {
    slice.iter().map(|byte| format!("{:02x}", byte)).collect()
}

#[test]
fn slice_to_string_conversion() {
    let bytes = &[];
    assert_eq!(slice_to_string(bytes), "");

    let bytes = &[0x22, 0, 0xdd, 0x0f];
    assert_eq!(slice_to_string(bytes), "2200dd0f");

    let bytes = &[0xa0; 15];
    assert_eq!(slice_to_string(bytes), "a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0");
}
