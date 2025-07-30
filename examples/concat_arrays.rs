use cluFullTransmute::try_transmute_or_panic;

// Combines two arrays of the same size `[T; N]` into a single fixed-length array `[T; N*2]`.

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
