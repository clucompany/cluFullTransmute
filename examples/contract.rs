use cluFullTransmute::contract::Contract;

/*
	For example, we will sign a contract to convert a String to a Vec<u8>,
	although this may not be exactly the case.

	Contracts are needed to create more secure APIs using transmutation in
	situations where it can't be proven.
*/

struct MyData {
	data: Contract<&'static str, &'static [u8]>,
}

impl MyData {
	#[inline]
	const fn new(data: &'static str) -> Self {
		let data = unsafe {
			// Contract::new_checksize_or_panic
			//

			// The `new_checksize_or_panic` function can only guarantee equality of data
			// dimensions, creating a contract is always unsafe, since the transmutation
			// of such data types can only be proven orally. But after signing the
			// transmutation contract, all functions for working with the transmuted are
			// not marked as unsafe.
			//
			Contract::new_checksize_or_panic(data)
		};
		Self { data }
	}

	#[inline]
	pub fn as_data(&self) -> &'static str {
		&self.data
	}

	#[inline]
	pub fn as_sliceu8(&self) -> &'static [u8] {
		self.data.as_datato()
	}

	#[inline]
	pub fn into(self) -> &'static [u8] {
		self.data.into()
	}
}

fn main() {
	const C_DATA: &str = "Test";

	// &'static str
	let data = MyData::new(C_DATA);
	assert_eq!(data.as_data(), C_DATA); // const_readtype: &'static str
	assert_eq!(data.as_sliceu8(), C_DATA.as_bytes()); //const_readtype &'static [u8]
												  //

	// &'static u8
	let vec = data.into(); // const_transmute: &'static str -> &'static [u8]
	assert_eq!(vec, C_DATA.as_bytes());
}
