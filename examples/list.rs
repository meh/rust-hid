extern crate hid;

fn main() {
	let hid = hid::init().unwrap();

	for device in hid.devices() {
		println!("{}:{}", device.vendor_id(), device.product_id());
	}
}
