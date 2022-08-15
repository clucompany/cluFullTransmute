# cluFullTransmute
[![CI](https://github.com/clucompany/cluFullTransmute/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/cluFullTransmute/actions/workflows/CI.yml)
[![Build Status](https://travis-ci.org/clucompany/cluFullTransmute.svg?branch=master)](https://travis-ci.org/clucompany/cluFullTransmute)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/cluFullTransmute)](https://crates.io/crates/cluFullTransmute)
[![Documentation](https://docs.rs/cluFullTransmute/badge.svg)](https://docs.rs/cluFullTransmute)

A more complete and extended version of data type conversion without constraint checks.

# Library Features

1. Casting any type A to any type B with generic data without and with data dimension checking.
2. Ability to use transmutation in constant functions in very old versions of rust.
3. Possibility of delayed transmutation through contracts.
4. Ability to work without the standard library.

# !!! ATTENTION !!!

1. When converting types without checking the size of the data, you really need to understand what you are doing.
2. You must understand the specifics of the platform you are using.


# Use

### 1. GenericType

```rust
use core::fmt::Display;
use cluFullTransmute::transmute::transmute_or_panic;

/// Implementation of a simple transmutation with a generic parameter inside.

#[derive(Debug)]
#[repr(transparent)]
struct A<T> {
	#[allow(dead_code)]
	data: T
}

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Invalid beh");
	}
}

#[derive(Debug)]
#[repr(transparent)]
struct B<T> where T: Display {
	data: T,
}

impl<T> Drop for B<T> where T: Display {
	fn drop(&mut self) {
		println!("{}", self.data);
	}
}

fn main() {
	let a: A<u16> = A { // original and panic when falling
		data: 1024
	};
	println!("in: {:?}", a);
	
	let b: B<u16> = unsafe { transmute_or_panic(a) };
	println!("out: {:?}", b);
	
	drop(b); // <--- println!
}
```

### 2. Contract

```rust
use cluFullTransmute::contract::Contract;

/*
	For example, we will sign a contract to convert a String to a Vec<u8>, 
	although this may not be exactly the case.
	
	Contracts are needed to create more secure APIs using transmutation in 
	situations where it can't be proven.
*/

/// 
struct MyData {
	data: Contract<&'static str, &'static [u8]>,
}

impl MyData {
	#[inline]
	const fn new(data: &'static str) -> Self {
		let data = unsafe {
			// Contract::force_new
			// 
			
			// The `checksize_new_or_panic` function can only guarantee equality of data 
			// dimensions, creating a contract is always unsafe, since the transmutation 
			// of such data types can only be proven orally. But after signing the 
			// transmutation contract, all functions for working with the transmuted are 
			// not marked as unsafe.
			//
			Contract::checksize_new_or_panic(data)
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


# License

Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
