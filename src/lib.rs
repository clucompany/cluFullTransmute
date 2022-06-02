//Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2022
/*!

A more complete and extended version of data type conversion without constraint checks.

# Library Features

1. Casting any type A to any type B without checking the dimension of the data.
2. Ability to use transmutation in constant functions.
3. Possibility of delayed transmutation through contracts.
4. Ability to work without the standard library
5. Extended support for rust versions, this library was originally designed to support permanent transmutation in circumstances where this cannot be done.

# !!! ATTENTION !!!

1. When converting types without checking the size of the data, you really need to understand what you are doing.
2. You must understand the specifics of the platform you are using.


# Use

### 1. GenericType

```rust

use cluFullTransmute::mem::force_transmute;

/*
	This example is notable because by the Rust standard it does not allow 
	converting common types A to B, since it cannot check their sizes, 
	this example solves this.
	
	Additionally, as an example, we manipulate the Drop::drop function.
*/

#[repr(transparent)]
struct A<T>(T);
#[repr(transparent)]
struct B<T>(T);

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Strange behavior of the internal library.");
	}
}

impl<T> B<T> {
	pub fn my_fn(&self) {}
}

impl<T> Drop for B<T> {
	fn drop(&mut self) {
		
	}
}

fn main() {
	let a: A<usize> = A(9999usize); // We expect panic at the death of A.
	let b: B<usize> = unsafe { force_transmute(a) }; // type A no longer exists, it is now type B.
	
	assert_eq!(b.0, 9999usize); // Checking the value
	b.my_fn();
	
	drop(b);
	// That's it, no panic, type B.
}
```

### 2. DataTransmutContract

```rust

use cluFullTransmute::mem::contract::DataTransmutContract;

/*
	For example, we will sign a contract to convert a String to a Vec<u8>, 
	although this may not be exactly the case.
	
	Contracts are needed to create more secure APIs using transmutation in 
	situations where it can't be proven.
*/

/// 
struct MyData {
	data: DataTransmutContract<&'static str, &'static [u8]>,
}

impl MyData {
	#[inline]
	const fn new(data: &'static str) -> Self {
		let data = unsafe {
			// DataTransmutContract::force_new
			// 
			
			// The `checksize_new_or_panic` function can only guarantee equality of data 
			// dimensions, creating a contract is always unsafe, since the transmutation 
			// of such data types can only be proven orally. But after signing the 
			// transmutation contract, all functions for working with the transmuted are 
			// not marked as unsafe.
			//
			DataTransmutContract::checksize_new_or_panic(data)
		};
		Self {
			data,
		}	
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
	const C_DATA: &'static str = "Test";
	
	// &'static str
	let data = MyData::new(C_DATA);
	assert_eq!(data.as_data(), C_DATA); // const_readtype: &'static str
	assert_eq!(data.as_sliceu8(), C_DATA.as_bytes()); //const_readtype &'static [u8]
	//
	
	// &'static u8
	let vec = data.into(); // const_transmute: &'static str -> &'static [u8]
	assert_eq!(vec, C_DATA.as_bytes());
}
```

*/

#![allow(non_snake_case)]

#![no_std]

/// Basic functions for dealing with memory. 
/// 
/// (To facilitate compatibility with the standard library, a similar file hierarchy was made.)
pub mod mem {
	/// Reinterprets the bits of a value of one type as another type. 
	pub mod transmute;
	
	/// Data Transformation Contract.
	pub mod contract;
	
	/*
		Left only for compatibility with std.
	*/
	pub use transmute::check_sizedata_transmute as transmute;
	pub use transmute::check_sizedata_transmute;
	pub use transmute::force_transmute;
	pub use transmute::inline_force_transmute;
}

/*
	Left for ease of use only.
*/
pub use self::mem::transmute::force_transmute;
pub use self::mem::transmute::check_sizedata_transmute;
pub use self::mem::transmute::check_sizedata_transmute as transmute;
