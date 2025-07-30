// Basic example using cluFullTransmute with size check.

use cluFullTransmute::try_transmute_or_panic;

fn main() {
	let input: [u8; 4] = [0x01, 0x02, 0x03, 0x04];

	// Attempt safe transmute to u32
	let value: u32 = unsafe { try_transmute_or_panic(input) };

	println!("Transmuted value: {value:#010X}");

	// Example of failed transmutation:
	let bad_input: [u8; 3] = [0x01, 0x02, 0x03];

	match unsafe { cluFullTransmute::try_transmute::<_, u32>(bad_input) } {
		Ok(v) => println!("Unexpected success: {v}"),
		Err(e) => {
			println!("Transmutation failed due to: {e}");
			if e.kind().is_size_mismatch() {
				println!(
					"Expected size: {:?}, Actual size: {:?}",
					e.into_kind(),
					e.as_data().len()
				);
			}
		}
	}
}
