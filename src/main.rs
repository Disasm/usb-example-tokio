use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use tokio::codec::{BytesCodec, FramedRead, FramedWrite};
use crate::async_usb_device::AsyncUsbDevice;
use tokio::runtime::current_thread;

//mod async_serial;
mod async_usb_device;
mod fake_usbd;
use fake_usbd::FakeBus;

fn main() {
    let stdin = tokio_stdin_stdout::stdin(0);
    let stdout = tokio_stdin_stdout::stdout(0);
    let _stdin_stream = FramedRead::new(stdin, BytesCodec::new());
    let _stdout_stream = FramedWrite::new(stdout, BytesCodec::new());

    let usb_bus = FakeBus::new();

    let serial = SerialPort::new(&usb_bus);

    let usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x5824, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    let poller = usb_dev.poll_class(serial, |serial| {
        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        },
                        _ => {},
                    }
                }
            }
            _ => {}
        }
    });

    let mut runtime = current_thread::Runtime::new().unwrap();
    runtime.block_on(poller).unwrap();
}
