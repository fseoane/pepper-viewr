use glib::translate::*;

use crate::UsbDevice;

#[derive(Clone, Debug)]
pub struct FakeUsbContext;

impl rusb::UsbContext for FakeUsbContext {
    fn as_raw(&self) -> *mut rusb::ffi::libusb_context {
        unimplemented!()
    }
}

impl UsbDevice {
    #[doc(alias = "spice_usb_device_get_libusb_device")]
    #[doc(alias = "get_libusb_device")]
    pub fn libusb_device(&self) -> Option<rusb::Device<FakeUsbContext>> {
        let ptr = unsafe { ffi::spice_usb_device_get_libusb_device(self.to_glib_none().0) };
        std::ptr::NonNull::new(ptr as *mut _)
            .map(|ptr| unsafe { rusb::Device::from_libusb(FakeUsbContext {}, ptr) })
    }
}
