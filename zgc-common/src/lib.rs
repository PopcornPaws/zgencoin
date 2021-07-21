/// Formats a byte stream into a hexadecimal `String` representation.
pub fn slice_to_string(slice: &[u8]) -> String {
    slice.iter().map(|byte| format!("{:02x}", byte)).collect()
}

#[test]
fn slice_to_string_conversion() {
    let bytes = &[];
    assert_eq!(slice_to_string(bytes), "");

    let bytes = &[0x22, 0x11, 0xdd, 0xff];
    assert_eq!(slice_to_string(bytes), "2211ddff");

    let bytes = &[0xa0; 15];
    assert_eq!(slice_to_string(bytes), "a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0");
}
