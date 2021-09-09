// This flag enables the use of array_chunks in nightly mode
#![feature(array_chunks)]

mod consts;
mod sha256;

pub use sha256::Sha256;

pub trait Hasher {
    fn digest<T: AsRef<str>>(&self, input: T) -> zgc_common::H256;
}
