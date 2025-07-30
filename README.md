<div id="header" align="center">

  <b>[clufulltransmute]</b>
  
  ( A more complete and extended version of data type conversion without constraint checks. )
  </br></br>

<div id="badges">
  <a href="./LICENSE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/cluFullTransmute">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/cluFullTransmute">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>
	
  [![CI](https://github.com/clucompany/cluFullTransmute/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/cluFullTransmute/actions/workflows/CI.yml) 


</div>
</div>

## !!! ATTENTION

1. When converting types without checking the size of the data, you really need to understand what you are doing.
2. You must understand the specifics of the platform you are using.

## Library features

1. Casting any type A to any type B with generic data without and with data dimension checking.
2. Ability to use transmutation in constant functions in very old versions of rust.
3. Possibility of delayed transmutation through contracts.
4. Ability to work without the standard library.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
cluFullTransmute = "1.3.1"
```

and this to your source code:
```rust
use cluFullTransmute::mem::transmute;
```

## Example

```rust
use cluFullTransmute::try_transmute_or_panic;
use core::fmt::Display;

/*
	For example, let's write some code with a Drop trait that panics when dropped and
	holds some data. We then transmute this data to another similar struct and check
	that we have effectively overridden the Drop trait and have a different struct
	with some data.

	We can also remove the Drop trait altogether or do any number of other things.
*/

/// Struct to panic when dropped
#[derive(Debug)]
#[repr(transparent)]
struct PanicWhenDrop<T>(T);

impl<T> Drop for PanicWhenDrop<T> {
	fn drop(&mut self) {
		panic!("panic, discovered `drop(PanicWhenDrop);`");
	}
}

/// Struct to print value when dropped
#[derive(Debug)]
#[repr(transparent)]
struct PrintlnWhenDrop<T: Display>(T)
where
	T: Display;

impl<T> Drop for PrintlnWhenDrop<T>
where
	T: Display,
{
	fn drop(&mut self) {
		println!("println: {}", self.0);
	}
}

fn main() {
	let a: PanicWhenDrop<u16> = PanicWhenDrop(1024);
	println!("in a: {:?}", a);

	let b: PrintlnWhenDrop<u16> = unsafe { try_transmute_or_panic(a as PanicWhenDrop<u16>) };
	println!("out b: {:?}", b);

	drop(b); // <--- drop, PrintlnWhenDrop!
}
```

<a href="./examples">
  See all
</a>

## License

This project has a single license (LICENSE-APACHE-2.0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2019-2025 #UlinProject</b>
	
  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License

<div align="left">
  <a href="./LICENSE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>
    
  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>
