

use cluFullTransmute::mem::MaybeTransmute;

struct MyData {
	data: MaybeTransmute<String, Vec<u8>>,
}

impl MyData {
	#[inline]
	pub fn new<I: Into<String>>(t: I) -> Self {
		Self::__new(t.into())	
	}
	
	#[inline]
	const fn __new(t: String) -> Self {
		Self {
			data: MaybeTransmute::new(t),	
		}	
	}
	
	#[inline]
	pub fn as_string(&mut self) -> &mut String {
		&mut self.data
	}
	
	#[inline]
	pub fn into(self) -> Vec<u8> {
		unsafe { self.data.into() }
	}
}


fn main() {
	let mut data = MyData::new("Test");
	assert_eq!(data.as_string().as_bytes(), b"Test");
	assert_eq!(data.as_string(), "Test");
	
	
	let vec = data.into();
	assert_eq!(vec, b"Test");
	
}
