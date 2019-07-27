use usb_device::device::UsbDevice;
use tokio::prelude::*;
use usb_device::bus::UsbBus;
use usb_device::class::UsbClass;

pub trait AsyncUsbDevice<'a, B: UsbBus> {
    fn poll_class<C, F>(self, class: C, f: F) -> AsyncDevicePollFuture<'a, B, C, F>
        where C: UsbClass<B>, F: FnMut(&mut C);
}

impl<'a, B: UsbBus> AsyncUsbDevice<'a, B> for UsbDevice<'a, B> {
    fn poll_class<C, F>(self, class: C, f: F) -> AsyncDevicePollFuture<'a, B, C, F>
        where C: UsbClass<B>, F: FnMut(&mut C)
    {
        AsyncDevicePollFuture {
            inner: self,
            class,
            cb: f
        }
    }
}

pub struct AsyncDevicePollFuture<'a, B: UsbBus, C, F> {
    inner: UsbDevice<'a, B>,
    class: C,
    cb: F,
}

impl<'a, B: UsbBus, C: UsbClass<B>, F: FnMut(&mut C)> Future for AsyncDevicePollFuture<'a, B, C, F> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if self.inner.poll(&mut [&mut self.class]) {
            (self.cb)(&mut self.class);
        }
        Ok(Async::NotReady)
    }
}
