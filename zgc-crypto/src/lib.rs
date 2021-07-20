#![feature(array_chunks)]
// STEP 1 >> Preprocessing
// 1) convert to binary
// 2) append a single 1 bit
// 3) pad with 0 until length is a multiple of 512
// 4) replace last 64 bytes with the input data length
// STEP 2 >> Initialization of auxiliary hash variables
// 1) first 32 bits of the fractional part of the
//    square root of the first 8 primes (2, 3, 5, 7, 11, 13, 17, 19)
// 2) first 32 bits of the fractional part of the
//    cubic root of the first 64 primes (2, 3, 5, 7, 11, ..., 311)
// STEP 3 >> for every 512 bit chunk:
// 1) create a message schedule
// 2) compression
// 3) modify hash values
// STEP 4 >> concatenate final hash

mod consts;
use consts::{HASHES, ROUND_CONSTANTS};

pub fn sha256(input: String) -> [u8; 32] {
    let processed = preprocess(input);

    let mut hashes = HASHES;
    // FOR_EACH CHUNK
    processed.array_chunks::<64>().for_each(|chunk| {
        let scheduled = schedule(chunk);
        compress(&mut hashes, &scheduled);
    });

    let mut digest = [0_u8; 32];
    digest
        .array_chunks_mut::<4>()
        .enumerate()
        .for_each(|(i, chunk)| chunk.copy_from_slice(&hashes[i].to_be_bytes()));
    //digest[0..4].copy_from_slice(&hashes[0].to_be_bytes());
    //digest[4..8].copy_from_slice(&hashes[1].to_be_bytes());
    //digest[8..12].copy_from_slice(&hashes[2].to_be_bytes());
    //digest[12..16].copy_from_slice(&hashes[3].to_be_bytes());
    //digest[16..20].copy_from_slice(&hashes[4].to_be_bytes());
    //digest[20..24].copy_from_slice(&hashes[5].to_be_bytes());
    //digest[24..28].copy_from_slice(&hashes[6].to_be_bytes());
    //digest[28..].copy_from_slice(&hashes[7].to_be_bytes());

    digest
}

fn right_rotate(num: u32, by: usize) -> u32 {
    let right_shifted = num >> by;
    let left_shifted = num << (32 - by);
    right_shifted | left_shifted
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

fn schedule(chunk_512: &[u8]) -> Vec<u32> {
    //let mut message_schedule = Vec::<u32>::new();
    //for i in 0..processed_input_vec.len() / 4 {
    //    let mut u32_bytes = [0_u8; 4];
    //    u32_bytes.copy_from_slice(&processed_input_vec[4 * i..4 * i + 4]);
    //    message_schedule.push(u32::from_be_bytes(u32_bytes));
    //}

    //let message_schedule = processed_input_vec
    //    .chunks(4)
    //    .map(|chunk| {
    //        let mut u32_bytes = [0_u8; 4];
    //        u32_bytes.copy_from_slice(chunk);
    //        u32::from_be_bytes(u32_bytes)
    //    })
    //    .collect::<Vec<u32>>();
    debug_assert_eq!(chunk_512.len(), 64, "chunk has to be 64 bytes long");
    let mut scheduled = chunk_512
        .array_chunks::<4>()
        .map(|chunk| u32::from_be_bytes(*chunk))
        .collect::<Vec<u32>>();

    scheduled.reserve_exact(64);

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

fn compress(hash_values: &mut [u32], scheduled: &[u32]) {
    let mut a = hash_values[0];
    let mut b = hash_values[1];
    let mut c = hash_values[2];
    let mut d = hash_values[3];
    let mut e = hash_values[4];
    let mut f = hash_values[5];
    let mut g = hash_values[6];
    let mut h = hash_values[7];
    // NOTE Compression
    for i in 0..64 {
        let rotated_a = right_rotate(a, 2) ^ right_rotate(a, 13) ^ right_rotate(a, 22);
        let rotated_e = right_rotate(e, 6) ^ right_rotate(e, 11) ^ right_rotate(e, 25);
        let auxiliary_1 = (e & f) ^ ((!e) & g);
        let auxiliary_2 = h
            .wrapping_add(rotated_e)
            .wrapping_add(auxiliary_1)
            .wrapping_add(ROUND_CONSTANTS[i])
            .wrapping_add(scheduled[i]);
        let auxiliary_3 = (a & b) ^ (a & c) ^ (b & c);
        let auxiliary_4 = rotated_a.wrapping_add(auxiliary_3);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(auxiliary_2);
        d = c;
        c = b;
        b = a;
        a = auxiliary_2.wrapping_add(auxiliary_4);
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
    use super::*;
    use zgc_common::slice_to_string;

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
        assert_eq!(right_rotate(0, 7), 0 << 25);
        assert_eq!(right_rotate(1032, 31), 2064);
        assert_eq!(right_rotate(50000, 31), 100000);
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

    #[test]
    fn encoding() {
        let encoded = sha256(String::from(""));
        assert_eq!(
            slice_to_string(&encoded),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        let encoded = sha256(String::from("hello world"));
        assert_eq!(
            slice_to_string(&encoded),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        let encoded = sha256(String::from("test data"));
        assert_eq!(
            slice_to_string(&encoded),
            "916f0027a575074ce72a331777c3478d6513f786a591bd892da1a577bf2335f9"
        );

        let encoded = sha256(String::from(
            "Do you think that this sentence is definitely longer than 64 bytes?",
        ));
        assert_eq!(
            slice_to_string(&encoded),
            "fba4ec9f441ffbadbf3a21a9976976f34bf2448702c47279677ab594979a3bb9"
        );
    }
}
