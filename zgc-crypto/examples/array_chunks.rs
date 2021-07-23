#![feature(array_chunks)]

// an example to show how array_chunks makes the code
// easier to read and maintain and leaves less room
// for error during development
//
// to run this example you need to run 
//
// cargo run --example array_chunks
//
// from the zgc-crypto directory

fn schedule(input: &[u8]) -> Vec<u32> {
    let mut message_schedule = Vec::<u32>::new();
    for i in 0..input.len() / 4 {
        let mut u32_bytes = [0_u8; 4];
        u32_bytes.copy_from_slice(&input[4 * i..4 * i + 4]);
        message_schedule.push(u32::from_be_bytes(u32_bytes));
    }
    message_schedule
}

fn schedule_iter(input: &[u8]) -> Vec<u32> {
    input
        .chunks(4)
        .map(|chunk| {
            let mut u32_bytes = [0_u8; 4];
            u32_bytes.copy_from_slice(chunk);
            u32::from_be_bytes(u32_bytes)
        })
        .collect()
}

fn schedule_nightly(input: &[u8]) -> Vec<u32> {
    input
        .array_chunks::<4>()
        .map(|chunk| u32::from_be_bytes(*chunk))
        .collect()
}

fn main() {
    let a = vec![255_u8; 64];
    let b = schedule(&a);
    let c = schedule_iter(&a);
    let d = schedule_nightly(&a);
    assert_eq!(b, c);
    assert_eq!(b, d);

    println!("It works!");
}
