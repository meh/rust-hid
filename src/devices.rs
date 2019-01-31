use sys::*;
use Device;

/// An iterator over the available devices.
pub struct Devices {
    cur: *mut hid_device_info,
}

impl Devices {
    #[doc(hidden)]
    pub unsafe fn new(vendor: Option<u16>, product: Option<u16>) -> Self {
        let list = hid_enumerate(vendor.unwrap_or(0), product.unwrap_or(0));

        Devices { cur: list }
    }
}

impl Iterator for Devices {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_null() {
            return None;
        }

        unsafe {
            let info = Device::new(self.cur);
            self.cur = (*self.cur).next;

            Some(info)
        }
    }
}
