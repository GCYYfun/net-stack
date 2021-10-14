extern crate alloc;
// OS 维护 一些 静态 全局 资源
use alloc::string::String;
use alloc::vec::Vec;
use smoltcp::phy::DeviceCapabilities;
use smoltcp::socket::SocketSet;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use lazy_static::lazy_static;
use spin::RwLock;
use alloc::sync::Arc;
// 关于 网络 
lazy_static! {
    pub static ref NET_DEVICES: RwLock<Vec<Arc<dyn NetDevice>>> = RwLock::new(Vec::new());
}


// 网络设备 初始化

pub fn net_device_init() {
    // interface
}

pub trait NetDevice : Send + Sync{
    fn mac(&self) -> EthernetAddress {
        unimplemented!("not a net driver")
    }
    fn ifname(&self) -> String {
        unimplemented!("not a net driver")
    }
    fn ip_addresses(&self) -> Vec<IpCidr> {
        unimplemented!("not a net driver")
    }
    fn ipv4_address(&self) -> Option<Ipv4Address> {
        unimplemented!("not a net driver")
    }
    fn send(&self, _data: &[u8]) -> Option<usize> {
        unimplemented!("not a net driver")
    }
    fn arp(&self, _ip: IpAddress) -> Option<EthernetAddress> {
        unimplemented!("not a net driver")
    }
    fn device_cap(&self) -> DeviceCapabilities {
        unimplemented!("not a net driver")
    }
}