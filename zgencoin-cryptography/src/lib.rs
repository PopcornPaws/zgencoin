//fn sha256(input: String) -> [u8; 32] {
//    // STEP 1 >> Preprocessing
//    // 1) convert to binary
//    // 2) append a single 1 bit
//    // 3) pad with 0 until length is a multiple of 512
//    // 4) replace last 64 bytes with the input data length
//    // STEP 2 >> Initialization of auxiliary hash variables
//    // 1) first 32 bits of the fractional part of the
//    //    square root of the first 8 primes (2, 3, 5, 7, 11, 13, 17, 19)
//    // 2) first 32 bits of the fractional part of the 
//    //    cubic root of the first 64 primes (2, 3, 5, 7, 11, ..., 311)
//    // STEP 3 >> for every 512 bit chunk:
//    // 1) create a message schedule
//    // 2) compression
//    // 3) modify hash values
//    // STEP 4 >> concatenate final hash
//}

fn sha256(input: String) -> [u8; 32] {
    let processed_input_vec = preprocess(input);
    todo!();
}

fn preprocess(input: String) -> Vec<u8> {
    // as_bytes vs into_bytes
    let mut input_bytes = input.into_bytes(); // Vec<u8>
    let original_length = input_bytes.len(); // in bytes!

    input_bytes.push(0b1000_0000);

    while input_bytes.len() % 64 != 0 {
        input_bytes.push(0_u8)
    }

    let original_length_in_bits = (original_length * 8).to_be_bytes();

    let copy_range = input_bytes.len() - 8..input_bytes.len();

    input_bytes[copy_range].copy_from_slice(&original_length_in_bits); // panics if lengths are not equal

    input_bytes
}

#[cfg(test)]
mod test {
    use super::preprocess;

    #[test]
    fn preprocessing() {
        let preprocessed_data = preprocess("hello".to_owned());
        assert_eq!(preprocessed_data.len(), 64);
        assert_eq!(preprocessed_data.len(), 64);
        assert_eq!(preprocessed_data[56..], [0_u8, 0, 0, 0, 0, 0, 0, 40]);
    }
}
