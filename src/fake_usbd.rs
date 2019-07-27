#![allow(unused)]
use usb_device::bus::{UsbBus, PollResult};
use usb_device::{UsbDirection, Result};
use usb_device::prelude::*;
use usb_device::class_prelude::*;
use usb_device::endpoint::EndpointAddress;

pub struct FakeBus;

impl FakeBus {
    pub fn new() -> UsbBusAllocator<Self> {
        let bus = FakeBus;

        UsbBusAllocator::new(bus)
    }
}

impl UsbBus for FakeBus {
    fn alloc_ep(&mut self, ep_dir: UsbDirection, ep_addr: Option<EndpointAddress>, ep_type: EndpointType, max_packet_size: u16, interval: u8) -> Result<EndpointAddress> {
        unimplemented!()
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
        PollResult::None
    }
}
