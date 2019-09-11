# cluFullTransmute
[![Build Status](https://travis-ci.org/clucompany/cluFullTransmute.svg?branch=master)](https://travis-ci.org/clucompany/cluFullTransmute)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluFullTransmute)](https://crates.io/crates/cluFullTransmute)
[![Documentation](https://docs.rs/cluFullTransmute/badge.svg)](https://docs.rs/cluFullTransmute)

A more complete and advanced version of data transmutation.

# Opportunities
1. Reduction of any A to any B, without checking the dimensionality of the data.
2. The ability to use transmute with const functions.
3. Possibility of delayed transmutation.
4. The library uses #![no_std]

# A warning!

1. This library only works in a nightly compiler, we expect stabilization features.
2. You really need to understand what you are doing.


# Use

1. Easy

```rust
use cluFullTransmute::mem::full_transmute;

fn main() {
	let a: bool = unsafe{ full_transmute(1u8) };
	assert_eq!(a, true);
	
	let b: bool = unsafe{ full_transmute(0u8) };
	assert_eq!(b, false);
	
	// Why does this work?
	//
	// Is bool one bit?
	// No, bool is not one bit, but u8.
	//
	assert_eq!(std::mem::size_of::<bool>(), 1);
}
```

2. GenericType

```rust
use cluFullTransmute::mem::full_transmute;

#[allow(dead_code)]
struct A<T>(T);

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Strange behavior of the internal library.");
	}
}

#[allow(dead_code)]
struct B<T>(T);

impl<T> B<T> {
	pub fn my_fn(&self) {}
}

fn main() {
	let data = A(9999usize); //ignore drop!
	
	let b: B<usize> = unsafe{ full_transmute(data) };
	assert_eq!(b.0, 9999usize);
	
	b.my_fn();
}
```

3. MaybeTransmute

```rust
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
```

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
