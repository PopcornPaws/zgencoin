// TODO #![feature(array_chunks)]
//
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

mod consts;
use consts::{HASH_VALUES, ROUND_CONSTANTS};

pub fn sha256(input: String) -> [u8; 32] {
    let processed_input_vec = preprocess(input);
    //let message_schedule = processed_input_vec
    //    .array_chunks::<4>()
    //    .map(|chunk| u32::from_be_bytes(*chunk))
    //    .collect::<Vec<u32>>();
    let mut message_schedule = Vec::<u32>::new();
    for i in 0..processed_input_vec.len() / 4 - 1 {
        let mut u32_bytes = [0_u8; 4];
        u32_bytes.copy_from_slice(&processed_input_vec[4 * i..4 * i + 4]);
        message_schedule.push(u32::from_be_bytes(u32_bytes));
    }

    //let message_schedule = processed_input_vec
    //    .chunks(4)
    //    .map(|chunk| {
    //        let mut u32_bytes = [0_u8; 4];
    //        u32_bytes.copy_from_slice(chunk);
    //        u32::from_be_bytes(u32_bytes)
    //    })
    //    .collect::<Vec<u32>>();
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

    // original length in bits
    let original_length_in_bits = (original_length * 8).to_be_bytes();
    // last 8 bytes
    let copy_range = input_bytes.len() - 8..input_bytes.len();
    // copy the original length (in bits) to the last 64 bits of the padded data
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
        assert_eq!(preprocessed_data[5], 0b1000_0000);
        assert_eq!(preprocessed_data[56..], [0_u8, 0, 0, 0, 0, 0, 0, 40]);

        for element in &preprocessed_data[6..56] {
            assert_eq!(*element, 0_u8);
        }

        let preprocessed_data =
            preprocess(String::from_utf8(vec![15_u8; 100]).expect("invalid utf8 bytearray"));
        assert_eq!(preprocessed_data.len(), 128);
        assert_eq!(preprocessed_data[100], 0b1000_0000);
        assert_eq!(preprocessed_data[120..], [0_u8, 0, 0, 0, 0, 0, 3, 32]); // length is 800 in bits

        for element in &preprocessed_data[101..120] {
            assert_eq!(*element, 0_u8);
        }
    }
}
