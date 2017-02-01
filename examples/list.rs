extern crate hid;

fn main() {
	let hid = hid::init().unwrap();

	for device in hid.devices() {
		println!(
			"0x{:x}:0x{:x} - {:?}: {:?}",
			device.vendor_id(),
			device.product_id(),
			device.manufacturer_string(),
			device.product_string()
		);
	}
}
