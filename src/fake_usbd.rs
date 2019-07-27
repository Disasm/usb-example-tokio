#![allow(unused)]
use usb_device::bus::{UsbBus, PollResult};
use usb_device::{UsbDirection, Result};
use usb_device::prelude::*;
use usb_device::class_prelude::*;
use usb_device::endpoint::EndpointAddress;

struct EndpointAllocator {
    next_free: u8,
}

impl EndpointAllocator {
    fn new() -> Self {
        Self {
            next_free: 1,
        }
    }

    fn allocate(&mut self, ep_dir: UsbDirection, ep_addr: Option<EndpointAddress>) -> EndpointAddress {
        if let Some(addr) = ep_addr {
            addr
        } else {
            let ep_index = self.next_free;
            self.next_free += 1;
            EndpointAddress::from_parts(ep_index as usize, ep_dir)
        }
    }
}

pub struct FakeBus {
    ep_allocator: EndpointAllocator,
}

impl FakeBus {
    pub fn new() -> UsbBusAllocator<Self> {
        let bus = FakeBus {
            ep_allocator: EndpointAllocator::new()
        };

        UsbBusAllocator::new(bus)
    }
}

impl UsbBus for FakeBus {
    fn alloc_ep(&mut self, ep_dir: UsbDirection, ep_addr: Option<EndpointAddress>, ep_type: EndpointType, max_packet_size: u16, interval: u8) -> Result<EndpointAddress> {
        Ok(self.ep_allocator.allocate(ep_dir, ep_addr))
    }

    fn enable(&mut self) {
    }

    fn reset(&self) {
    }

    fn set_device_address(&self, addr: u8) {
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize> {
        Err(UsbError::WouldBlock)
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize> {
        Err(UsbError::WouldBlock)
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        unimplemented!()
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        unimplemented!()
    }

    fn suspend(&self) {
    }

    fn resume(&self) {
    }

    fn poll(&self) -> PollResult {
        PollResult::Data {
            ep_out: 0,
            ep_in_complete: 0,
            ep_setup: 0
        }
    }
}
