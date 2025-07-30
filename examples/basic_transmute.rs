use cluFullTransmute::transmute_unchecked;

fn main() {
	let bytes: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
	let number: u32 = unsafe { transmute_unchecked(bytes) };

	println!("Transmuted value: {number:#010X}");
}
