use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

use sys::*;
use Result as Res;
use {Error, Devices};

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

pub struct Manager;

unsafe impl Send for Manager { }

pub fn init() -> Res<Manager> {
	if INITIALIZED.load(Ordering::Relaxed) {
		return Err(Error::Initialized);
	}

	let status = unsafe { hid_init() };

	if status != 0 {
		return Err(Error::from(status));
	}

	INITIALIZED.store(true, Ordering::Relaxed);

	Ok(Manager)
}

impl Drop for Manager {
	fn drop(&mut self) {
		let status = unsafe { hid_exit() };

		if status != 0 {
			panic!("hid_exit() failed");
		}

		INITIALIZED.store(false, Ordering::Relaxed);
	}
}

impl Manager {
	pub fn find(&self, vendor: Option<u16>, product: Option<u16>) -> Devices {
		unsafe {
			Devices::new(vendor, product)
		}
	}

	pub fn devices(&self) -> Devices {
		self.find(None, None)
	}
}
