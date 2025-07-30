//Copyright 2019-2025 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2019-2025
/*!

Extended, no-constraint type transmutation API, featuring safe checks and const-ready logic.

# concat_arrays

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
*/

#![allow(non_snake_case)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::needless_doctest_main)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "stderr"), no_std)]

/// Basic functions for dealing with memory.
///
/// (An optional module for ensuring compatibility with the standard library, which is turned on and off with the `compatible_stdapi` build flag.)
#[cfg_attr(docsrs, doc(cfg(feature = "compatible_stdapi")))]
#[cfg(any(test, feature = "compatible_stdapi"))]
pub mod mem {
	pub use crate::raw::transmute_unchecked;
	/// Reinterprets the bits of a value of one type as another type.
	/// The function is completely constant, in case of a size mismatch, a panic pops up.
	pub use crate::try_transmute_or_panic as transmute;
}

#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
pub mod err;
mod raw;

#[cfg_attr(docsrs, doc(cfg(feature = "to")))]
#[cfg(any(test, feature = "to"))]
pub mod to;

#[cfg_attr(docsrs, doc(cfg(feature = "contract")))]
#[cfg(any(test, feature = "contract"))]
pub mod contract;

#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
use crate::err::TransmuteErr;
pub use crate::raw::transmute_unchecked;

/// A constant function reinterprets the bits of a value of one type as another type.
///
/// # Safety
///
/// If the sizes do not match, a panic arises.
#[track_caller]
#[cfg_attr(
	all(feature = "transmute-inline", not(feature = "transmute-inline-always")),
	inline
)]
#[cfg_attr(feature = "transmute-inline-always", inline(always))]
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
pub const unsafe fn try_transmute_or_panic<D, To>(in_data: D) -> To {
	use crate::err::TransmuteErrKind;
	pub use crate::raw::transmute_unchecked;
	use core::mem::size_of;
	{
		// Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();

		if size_d != size_to {
			let errkind = TransmuteErrKind::size_mismatch(size_d, size_to);

			errkind.unwrap();
		}
	}

	unsafe { transmute_unchecked(in_data) }
}

/// A constant function reinterprets the bits of a value of one type as another type.
///
/// # Safety
///
/// If the size does not match, an error occurs.
#[cfg_attr(
	all(feature = "transmute-inline", not(feature = "transmute-inline-always")),
	inline
)]
#[cfg_attr(feature = "transmute-inline-always", inline(always))]
#[cfg_attr(docsrs, doc(cfg(feature = "support_size_check_transmute")))]
#[cfg(any(test, feature = "support_size_check_transmute"))]
pub const unsafe fn try_transmute<D, To>(in_data: D) -> Result<To, TransmuteErr<D>> {
	pub use crate::raw::transmute_unchecked;
	use core::mem::size_of;
	{
		// Data dimension check
		let size_d = size_of::<D>();
		let size_to = size_of::<To>();

		if size_d != size_to {
			let err = TransmuteErr::size_mismatch(size_d, size_to, in_data);

			return Err(err);
		}
	}

	Ok(unsafe { transmute_unchecked(in_data) })
}
