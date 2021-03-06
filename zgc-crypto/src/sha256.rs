// STEP 1 >> Initialization of auxiliary hash variables
// 1) first 32 bits of the fractional part of the
//    square root of the first 8 primes (2, 3, 5, 7, 11, 13, 17, 19)
// 2) first 32 bits of the fractional part of the
//    cubic root of the first 64 primes (2, 3, 5, 7, 11, ..., 311)
// STEP 2 >> Preprocessing
// 1) convert to binary
// 2) append a single 1 bit
// 3) pad with 0 until length is a multiple of 512
// 4) replace last 64 bits with the input data length
// STEP 3 >> for every 512 bit chunk:
// 1) create a message schedule
// 2) compression
// 3) modify hash values
// STEP 4 >> concatenate final hash

use crate::consts::{HASHES, ROUND_CONSTANTS};
use crate::Hasher;

use zgc_common::H256;

pub struct Sha256;

impl Sha256 {
    pub fn new() -> Self {
        Self
    }
}

impl Hasher for Sha256 {
    fn digest(&self, input: String) -> H256 {
        let processed = preprocess(input);
        // since HASHES [u8; 8] is a Copy type (because it's not expensive to copy
        // 8 u32 numbers) it doesn't get moved out of scope, it is simply copied
        // into `hashes`
        let mut hashes = HASHES;
        // for each chunk, schedule them and compress them
        processed.chunks(64).for_each(|chunk| {
            let scheduled = schedule(chunk);
            compress(&mut hashes, &scheduled);
        });

        // digest the 8 u32 hash values that were
        // successively modified in the chunk loop
        //
        // array_chunks_mut gives you a mutable reference to
        // a [u8; 4] fixed array in which we can copy
        // the i^th u32 of `hashes` converted into big
        // endian bytes.
        let mut digest = [0_u8; 32];
        digest
            .array_chunks_mut::<4>()
            .enumerate()
            .for_each(|(i, chunk)| chunk.copy_from_slice(&hashes[i].to_be_bytes()));

        H256::new(digest)
    }
}

/// Right rotates a 32 bit unsigned integer by a given number.
///
/// Note, that without the modulo division and the if-else logic, the function
/// would panic at runtime if we wanted to shift the number by a value greater
/// than or equal to 32.
fn right_rotate(num: u32, mut by: usize) -> u32 {
    by %= 32;
    if by == 0 {
        num
    } else {
        let right_shifted = num >> by;
        let left_shifted = num << (32 - by);
        right_shifted | left_shifted
    }
}

/// Preprocesses the input data.
///
/// A String type is a dynamic type allocated on the heap,
/// therefore it doesn't implement the Copy trait. This means,
/// that if the input is passed to this function by value,
/// it is moved into the scope of this function and cannot
/// be used anymore in the parent function.
fn preprocess(input: String) -> Vec<u8> {
    // as_bytes(&self) vs into_bytes(self)
    // as_bytes -> input is passed by reference so it doesn't get moved out of
    // scope
    // into_bytes -> input is passed by value so it does get moved out of scope
    // and cannot be used afterwards
    let mut input_bytes = input.into_bytes(); // Vec<u8>
    let original_bytes_len = input_bytes.len(); // in bytes!

    // reserve some memory for the Vec<u8> (dynamically allocated on the heap)
    // so that it doesn't reallocate every time it runs out of available space
    // when we push to it consecutively
    input_bytes.reserve(64);
    input_bytes.push(0b1000_0000);
    while input_bytes.len() % 64 != 0 {
        input_bytes.push(0_u8)
    }

    // original length in bits
    let original_length_in_bits = (original_bytes_len * 8).to_be_bytes();
    // last 8 bytes
    let last_8_bytes = input_bytes.len() - 8..input_bytes.len();
    // copy the original length (in bits) to the last 64 bits of the padded data
    input_bytes[last_8_bytes].copy_from_slice(&original_length_in_bits); // panics if lengths are not equal

    input_bytes
}

/// Performs the scheduling step.
///
/// In every chunk loop, a 512 bit long byte stream is converted
/// into u32 words which are then padded by 0 to have length 64.
///
/// Thereafter, a bitwise operation loop is performed on the padded
/// vector.
fn schedule(chunk_512: &[u8]) -> Vec<u32> {
    let mut scheduled = chunk_512
        .array_chunks::<4>()
        .map(|chunk| u32::from_be_bytes(*chunk))
        .collect::<Vec<u32>>();

    // reserve additional space here as well to
    // avoid reallocations on the heap
    scheduled.reserve_exact(48);
    for _ in 0..48 {
        scheduled.push(0)
    }

    for i in 16..64 {
        let right_rotate_7 = right_rotate(scheduled[i - 15], 7);
        let right_rotate_18 = right_rotate(scheduled[i - 15], 18);
        let right_shift_3 = scheduled[i - 15] >> 3;

        let xor_1 = right_rotate_7 ^ right_rotate_18 ^ right_shift_3;

        let right_rotate_17 = right_rotate(scheduled[i - 2], 17);
        let right_rotate_19 = right_rotate(scheduled[i - 2], 19);
        let right_shift_10 = scheduled[i - 2] >> 10;

        let xor_2 = right_rotate_17 ^ right_rotate_19 ^ right_shift_10;

        scheduled[i] = scheduled[i - 16]
            .wrapping_add(xor_1)
            .wrapping_add(scheduled[i - 7])
            .wrapping_add(xor_2);
    }
    scheduled
}

/// Performs the compression step.
///
/// In every chunk loop, the hash values are updated in place
/// using the scheduled values generated in [`schedule`].
fn compress(hash_values: &mut [u32], scheduled: &[u32]) {
    let mut a = hash_values[0];
    let mut b = hash_values[1];
    let mut c = hash_values[2];
    let mut d = hash_values[3];
    let mut e = hash_values[4];
    let mut f = hash_values[5];
    let mut g = hash_values[6];
    let mut h = hash_values[7];

    for i in 0..64 {
        let rotated_a = right_rotate(a, 2) ^ right_rotate(a, 13) ^ right_rotate(a, 22);
        let rotated_e = right_rotate(e, 6) ^ right_rotate(e, 11) ^ right_rotate(e, 25);
        let ch = (e & f) ^ ((!e) & g);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp_1 = h
            .wrapping_add(rotated_e)
            .wrapping_add(ch)
            .wrapping_add(ROUND_CONSTANTS[i])
            .wrapping_add(scheduled[i]);
        let temp_2 = rotated_a.wrapping_add(maj);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp_1);
        d = c;
        c = b;
        b = a;
        a = temp_1.wrapping_add(temp_2);
    }

    hash_values[0] = hash_values[0].wrapping_add(a);
    hash_values[1] = hash_values[1].wrapping_add(b);
    hash_values[2] = hash_values[2].wrapping_add(c);
    hash_values[3] = hash_values[3].wrapping_add(d);
    hash_values[4] = hash_values[4].wrapping_add(e);
    hash_values[5] = hash_values[5].wrapping_add(f);
    hash_values[6] = hash_values[6].wrapping_add(g);
    hash_values[7] = hash_values[7].wrapping_add(h);
}

#[cfg(test)]
mod test {
    use super::*; // bring everything from the level above into scope

    ///// Formats a byte stream into a hexadecimal `String` representation.
    //fn slice_to_string(slice: &[u8]) -> String {
    //    slice.iter().map(|byte| format!("{:02x}", byte)).collect()
    //}

    //#[test]
    //fn slice_to_string_conversion() {
    //    let bytes = &[];
    //    assert_eq!(slice_to_string(bytes), "");

    //    let bytes = &[0x22, 0, 0xdd, 0x0f];
    //    assert_eq!(slice_to_string(bytes), "2200dd0f");

    //    let bytes = &[0xa0; 15];
    //    assert_eq!(slice_to_string(bytes), "a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0");
    //}

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

    #[test]
    fn right_rotation() {
        let a = 0b_1000_0000_1000_0000_1000_0000_1000_0000_u32;
        let b = right_rotate(a, 7);
        assert_eq!(b, 0b_0000_0001_0000_0001_0000_0001_0000_0001_u32);

        assert_eq!(right_rotate(1, 7), 1 << 25);
        assert_eq!(right_rotate(0, 7), 0);
        assert_eq!(right_rotate(1032, 31), 2064);
        assert_eq!(right_rotate(50000, 31), 100000);

        assert_eq!(right_rotate(1032, 32), 1032);
        assert_eq!(right_rotate(2, 33), 1);
    }

    #[test]
    fn encoding() {
        let hasher = Sha256::new();

        let encoded = hasher.digest(String::from(""));
        assert_eq!(
            encoded.to_string(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        let encoded = hasher.digest(String::from("hello world"));
        assert_eq!(
            encoded.to_string(),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        let encoded = hasher.digest(String::from("test data"));
        assert_eq!(
            encoded.to_string(),
            "916f0027a575074ce72a331777c3478d6513f786a591bd892da1a577bf2335f9"
        );

        let encoded = hasher.digest(String::from(
            "Do you think that this sentence is definitely longer than 64 bytes?",
        ));
        assert_eq!(
            encoded.to_string(),
            "fba4ec9f441ffbadbf3a21a9976976f34bf2448702c47279677ab594979a3bb9"
        );
    }
}
