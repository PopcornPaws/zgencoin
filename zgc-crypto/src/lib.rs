// This flag enables the use of array_chunks in nightly mode
#![feature(array_chunks)]

mod consts;
mod sha256;

pub use sha256::Sha256;

pub trait Hasher {
    fn digest(&self, input: String) -> zgc_common::H256;
}
