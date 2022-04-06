
use cluFullTransmute::mem::force_transmute;

fn main() {
	let a: bool = unsafe { force_transmute(1u8) };
	assert_eq!(a, true);
	
	let b: bool = unsafe { force_transmute(0u8) };
	assert_eq!(b, false);
	
	// Why does this work?
	//
	// Is bool one bit?
	// No, bool is not one bit, but u8.
	//
	assert_eq!(std::mem::size_of::<bool>(), 1);
}
