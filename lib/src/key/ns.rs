use derive::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Key)]
pub struct Ns {
	__: u8,
	_a: u8,
	_b: u8,
	_c: u8,
	pub ns: String,
}

pub fn new(ns: &str) -> Ns {
	Ns::new(ns.to_string())
}

pub fn prefix() -> Vec<u8> {
	let mut k = super::kv::new().encode().unwrap();
	k.extend_from_slice(&[0x21, 0x6e, 0x73, 0x00]);
	k
}

pub fn suffix() -> Vec<u8> {
	let mut k = super::kv::new().encode().unwrap();
	k.extend_from_slice(&[0x21, 0x6e, 0x73, 0xff]);
	k
}

impl Ns {
	pub fn new(ns: String) -> Ns {
		Ns {
			__: 0x2f, // /
			_a: 0x21, // !
			_b: 0x6e, // n
			_c: 0x73, // s
			ns,
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn key() {
		use super::*;
		#[rustfmt::skip]
		let val = Ns::new(
			"test".to_string(),
		);
		let enc = Ns::encode(&val).unwrap();
		let dec = Ns::decode(&enc).unwrap();
		assert_eq!(val, dec);
	}
}
