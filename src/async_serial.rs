use usbd_serial::SerialPort;
use tokio::io::AsyncRead;
use std::io;
use tokio::prelude::{AsyncWrite, Poll};
use usb_device::{Result as UsbResult, UsbError};
use usb_device::bus::UsbBus;
use std::borrow::BorrowMut;

pub trait GoodSerialPort {
    fn read(&mut self, data: &mut [u8]) -> UsbResult<usize>;
    fn write(&mut self, data: &[u8]) -> UsbResult<usize>;
    fn flush(&mut self) -> UsbResult<()>;
}

impl<'a, B: UsbBus, RS: BorrowMut<[u8]>, WS: BorrowMut<[u8]>> GoodSerialPort for SerialPort<'a, B, RS, WS> {
    fn read(&mut self, data: &mut [u8]) -> UsbResult<usize> {
        SerialPort::read(self, data)
    }

    fn write(&mut self, data: &[u8]) -> UsbResult<usize> {
        SerialPort::write(self, data)
    }

    fn flush(&mut self) -> UsbResult<()> {
        SerialPort::flush(self)
    }
}

pub struct AsyncSerialPort<T> {
    inner: T
}

impl<T> AsyncSerialPort<T> {
    pub fn new(serial: T) -> Self where T: GoodSerialPort {
        Self {
            inner: serial
        }
    }
}

impl<T: GoodSerialPort> io::Read for AsyncSerialPort<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.inner.read(buf) {
            Ok(size) if size > 0 => Ok(size),
            Ok(_) | Err(UsbError::WouldBlock) => Err(io::ErrorKind::WouldBlock.into()),
            Err(_) => Err(io::ErrorKind::Other.into()),
        }
    }
}

impl<T: GoodSerialPort> AsyncRead for AsyncSerialPort<T> {
}

impl<T: GoodSerialPort> io::Write for AsyncSerialPort<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner.write(buf) {
            Ok(size) if size > 0 => Ok(size),
            Ok(_) | Err(UsbError::WouldBlock) => Err(io::ErrorKind::WouldBlock.into()),
            Err(_) => Err(io::ErrorKind::Other.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.inner.flush() {
            Ok(()) => Ok(()),
            Err(UsbError::WouldBlock) => Err(io::ErrorKind::WouldBlock.into()),
            Err(_) => Err(io::ErrorKind::Other.into()),
        }
    }
}

impl<T: GoodSerialPort> AsyncWrite for AsyncSerialPort<T> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        unimplemented!()
    }
}
