# cluFullTransmute
[![CI](https://github.com/clucompany/cluFullTransmute/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/cluFullTransmute/actions/workflows/CI.yml)
[![Build Status](https://travis-ci.org/clucompany/cluFullTransmute.svg?branch=master)](https://travis-ci.org/clucompany/cluFullTransmute)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/cluFullTransmute)](https://crates.io/crates/cluFullTransmute)
[![Documentation](https://docs.rs/cluFullTransmute/badge.svg)](https://docs.rs/cluFullTransmute)

A more complete and extended version of data type conversion without constraint checks.

# Library Features

1. Casting any type A to any type B without checking the dimension of the data.
2. Ability to use transmutation in constant functions.
3. Possibility of delayed transmutation through contracts.
4. Ability to work without the standard library
5. Extended support for rust versions, this library was originally designed to support permanent transmutation in circumstances where this cannot be done.

# !!! ATTENTION !!!

1. When converting types without checking the size of the data, you really need to understand what you are doing.



# Use

### 1. GenericType

```rust
/*
	This example is notable because by the Rust standard it does not allow 
	converting common types A to B, since it cannot check their sizes, 
	this example solves this.
	
	Additionally, as an example, we manipulate the Drop::drop function.
*/

use cluFullTransmute::mem::force_transmute;

struct A<T>(T);
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
	let data = A(9999usize); // We expect panic at the death of A.
	
	let b: B<usize> = unsafe { force_transmute(data) }; // type A no longer exists, it is now type B.
	
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
	data: DataTransmutContract<String, Vec<u8>>,
}

impl MyData {
	#[inline]
	pub fn new<I: Into<String>>(t: I) -> Self {
		Self::__new(t.into())
	}
	
	#[inline]
	const fn __new(data: String) -> Self {
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
	pub fn as_string(&mut self) -> &mut String {
		&mut self.data
	}
	
	#[inline]
	pub fn into(self) -> Vec<u8> {
		self.data.into()
	}
}


fn main() {
	// String
	let mut data = MyData::new("Test");
	assert_eq!(data.as_string().as_bytes(), b"Test");
	assert_eq!(data.as_string(), "Test");
	//
	
	let vec = data.into(); // String -> Vec<u8>
	assert_eq!(vec, b"Test");
}
```


# License

Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
