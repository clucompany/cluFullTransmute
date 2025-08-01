<div id="header" align="center">

  <b>[clufulltransmute]</b>
  
  ( Extended, no-constraint type transmutation API, featuring safe checks and const-ready logic. )
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
cluFullTransmute = "1.4.1"
```

and this to your source code:
```rust
use cluFullTransmute::try_transmute;
use cluFullTransmute::try_transmute_or_panic;
use cluFullTransmute::transmute_unchecked;
```

## Example

### concat_arrays

Purpose: Combines two arrays of the same size `[T; N]` into a single fixed-length array `[T; N*2]`.

```rust
use cluFullTransmute::try_transmute_or_panic;

pub const fn concat_arrays<T, const N: usize, const NDOUBLE: usize>(
	a: [T; N],
	b: [T; N],
) -> [T; NDOUBLE] {
	#[repr(C)]
	struct Pair<T, const N: usize> {
		a: [T; N],
		b: [T; N],
	}

	unsafe { try_transmute_or_panic(Pair { a, b }) }
}

fn main() {
	const A: [u8; 4] = [1, 2, 3, 4];
	const B: [u8; 4] = [5, 6, 7, 8];
	const C: [u8; 8] = concat_arrays(A, B);

	println!("{C:?}"); // [1, 2, 3, 4, 5, 6, 7, 8]
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
