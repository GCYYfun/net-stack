#![no_std]
#![feature(untagged_unions)]

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod custom_test;

pub mod net;
pub mod resource;

// mod e1000;
// pub use e1000::*;

// #[macro_use]
extern crate alloc;

use alloc::collections::BTreeMap;
// use alloc::string::String;
// use alloc::string::ToString;
use alloc::sync::Arc;
// use alloc::vec::Vec;

// smoltcp
use smoltcp::iface::{Interface, InterfaceBuilder, NeighborCache, Route, Routes};
use smoltcp::phy::{Device, Loopback, Medium};
use smoltcp::socket::SocketSet;
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};

// helper
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::{debug, warn};
use spin::Mutex;
use spin::RwLock;

pub trait NetStack: Send + Sync {
    // fn interfaces(&self) -> Arc<Mutex<HashMap<usize, Interface<'static, D>>>>;
    fn poll(&self, socketset: &Mutex<SocketSet>, timestamp: Instant);
}

lazy_static! {
    // clone 和 内部可变
    // usize 要 改称 名字 或 索引 用的 \结构 要 能 hash\暂时 usize  试一下
    pub static ref NET_STACK : RwLock<Arc<Mutex<HashMap<usize,Arc<dyn NetStack>>>>> = RwLock::new(Arc::new(Mutex::new(HashMap::new())));
}

// #[derive(Default, Debug)]
// pub struct InterfaceInfo {
//     name: String,
// }

// impl From<>

// impl InterfaceInfo {
//     fn new(name: String) -> Self {
//         Self { name }
//     }
// }

#[allow(improper_ctypes)]
extern "C" {
    // fn hal_frame_alloc() -> Option<PhysAddr>;
    // fn hal_frame_dealloc(paddr: &PhysAddr);
    // fn hal_frame_alloc_contiguous(size: usize, align_log2: usize) -> Option<usize>;
    fn poll_ifaces_e1000();
    // get_net_driver() -> Vec<Arc<dyn NetDriver>>
}

pub struct Stack<D>
where
    D: for<'d> Device<'d> + Send + Sync + 'static,
{
    // iface 需要clone 和 内部 可变
    pub inner: Arc<Mutex<StackInner<D>>>,
    // resource
    // later...
    // config
    // later...
}

pub struct StackInner<D>
where
    D: for<'d> Device<'d> + Send + Sync,
{
    pub interface: Interface<'static, D>,
}

impl<D> NetStack for Stack<D>
where
    D: for<'d> Device<'d> + Send + Sync,
{
    // fn interfaces(&self) -> Arc<Mutex<HashMap<usize, Interface<'static, D>>>> {
    //     self.interfaces.clone()
    // }

    fn poll(&self, socketset: &Mutex<SocketSet>, timestamp: Instant) {
        // self.interfaces.get();
        // poll()
        // 遍历
        let mut inner = self.inner.lock();
        // warn!("key : {}",key);
        let mut sockets = socketset.lock();
        match inner.interface.poll(&mut sockets, timestamp) {
            Ok(_) => {
                // warn!("now in impl NetDriver for E1000Interface poll need SOCKET_ACTIVITY.notify_all()");
            }
            Err(err) => {
                debug!("poll got err {}", err);
            }
        }
    }
}

pub fn init() {
    loopback_init();
}

// net stack init

// A iterface init
// A iterface init

// fn interface_init(name: String, irq: Option<usize>, _header: usize, _size: usize, _index: usize) {
//     warn!("Probing {}, interrupt : {:?}", name, irq);
// }

pub fn loopback_init() {
    warn!("loopback");

    // 初始化 一个 协议栈

    // 从外界 接受 一些 配置 参数 如果 没有 选择 默认 的

    // 网络 设备
    // 默认 loopback
    let loopback = Loopback::new(Medium::Ethernet);

    // 为 设备 分配 网络 身份

    // 物理地址
    let mac: [u8; 6] = [0x52, 0x54, 0x98, 0x76, 0x54, 0x32];
    let ethernet_addr = EthernetAddress::from_bytes(&mac);
    // ip 地址
    let ip_addrs = [IpCidr::new(IpAddress::v4(127, 0, 0, 1), 24)];
    // let ip_addrs = [IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24)];
    // 路由
    let default_gateway = Ipv4Address::new(127, 0, 0, 1);
    // let default_gateway = Ipv4Address::new(10, 0, 2, 2);
    static mut ROUTES_STORAGE: [Option<(IpCidr, Route)>; 1] = [None; 1];
    let mut routes = unsafe { Routes::new(&mut ROUTES_STORAGE[..]) };
    routes.add_default_ipv4_route(default_gateway).unwrap();
    // arp缓存
    let neighbor_cache = NeighborCache::new(BTreeMap::new());

    // 设置 主要 设置 iface
    let iface = InterfaceBuilder::new(loopback)
        .ethernet_addr(ethernet_addr)
        .ip_addrs(ip_addrs)
        .routes(routes)
        .neighbor_cache(neighbor_cache)
        .finalize();

    // 创建 一个 栈
    let stack_inner = StackInner { interface: iface };
    // stack_inner.interfaces.insert(0, iface);

    // 把 栈 返回

    let stack = Stack {
        inner: Arc::new(Mutex::new(stack_inner)),
    };

    // let info = InterfaceInfo::new(String::from("smoltcp"));
    let w = NET_STACK.write();

    w.lock().insert(0, Arc::new(stack));
}

// use isomorphic_drivers::net::ethernet::intel::e1000::E1000;
// use isomorphic_drivers::net::ethernet::structs::EthernetAddress as DriverEthernetAddress;
// use isomorphic_drivers::provider::Provider;

// #[derive(Clone)]
// pub struct E1000Driver(Arc<Mutex<E1000<Dma>>>);
// use alloc::string::String;
// pub fn e1000_init(name: String, irq: Option<usize>, header: usize, size: usize, index: usize) {
//     warn!("e1000");

//     let mac: [u8; 6] = [0x52, 0x54, 0x00, 0x12, 0x34, 0x56];
//     let e1000 = E1000::new(header, size, DriverEthernetAddress::from_bytes(&mac));
//     let net_driver = E1000Driver(Arc::new(Mutex::new(e1000)));
//     // 初始化 一个 协议栈

//     // 从外界 接受 一些 配置 参数 如果 没有 选择 默认 的

//     // 网络 设备
//     // 默认 loopback
//     // let loopback = Loopback::new(Medium::Ethernet);

//     // 为 设备 分配 网络 身份

//     // 物理地址
//     let mac: [u8; 6] = [0x52, 0x54, 0x98, 0x76, 0x54, 0x32];
//     let ethernet_addr = EthernetAddress::from_bytes(&mac);
//     // ip 地址
//     let ip_addrs = [IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24)];
//     // let ip_addrs = [IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24)];
//     // 路由
//     let default_gateway = Ipv4Address::new(10, 0, 2, 2);
//     // let default_gateway = Ipv4Address::new(10, 0, 2, 2);
//     static mut routes_storage: [Option<(IpCidr, Route)>; 1] = [None; 1];
//     let mut routes = unsafe { Routes::new(&mut routes_storage[..]) };
//     routes.add_default_ipv4_route(default_gateway).unwrap();
//     // arp缓存
//     let neighbor_cache = NeighborCache::new(BTreeMap::new());

//     // 设置 主要 设置 iface
//     let iface = InterfaceBuilder::new(net_driver)
//         .ethernet_addr(ethernet_addr)
//         .ip_addrs(ip_addrs)
//         .routes(routes)
//         .neighbor_cache(neighbor_cache)
//         .finalize();

//     // 创建 一个 栈
//     let stack_inner = StackInner { interface: iface };
//     // stack_inner.interfaces.insert(0, iface);

//     // 把 栈 返回

//     let mut stack = Stack {
//         inner: Arc::new(Mutex::new(stack_inner)),
//     };

//     // let info = InterfaceInfo::new(String::from("smoltcp"));
//     let mut w = NET_STACK.write();

//     w.lock().insert(1, Arc::new(stack));
// }

// pub struct Dma;
// const PAGE_SIZE: usize = 4096;

// impl Provider for Dma {
//     const PAGE_SIZE: usize = 4096;

//     fn alloc_dma(size: usize) -> (usize, usize) {
//         let paddr = virtio_dma_alloc(size / PAGE_SIZE);
//         let vaddr = phys_to_virt(paddr);
//         (vaddr, paddr)
//     }

//     fn dealloc_dma(vaddr: usize, size: usize) {
//         let paddr = virt_to_phys(vaddr);
//         for i in 0..size / PAGE_SIZE {
//             unsafe {
//                 hal_frame_dealloc(&(paddr + i * PAGE_SIZE));
//             }
//         }
//     }
// }

// fn virtio_dma_alloc(pages: usize) -> PhysAddr {
//     let paddr = unsafe { hal_frame_alloc_contiguous(pages, 0).unwrap() };
//     // trace!("alloc DMA: paddr={:#x}, pages={}", paddr, pages);
//     paddr
// }

// const PMEM_BASE: usize = 0xFFFF800000000000;

// pub fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
//     unsafe { PMEM_BASE + paddr }
// }

// pub fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
//     unsafe { vaddr - PMEM_BASE }
// }

// type VirtAddr = usize;
// type PhysAddr = usize;

// use alloc::vec::Vec;
// use smoltcp::phy;
// use smoltcp::phy::DeviceCapabilities;
// use smoltcp::Result;

// pub struct E1000RxToken(Vec<u8>);
// pub struct E1000TxToken(E1000Driver);

// impl phy::Device<'_> for E1000Driver {
//     type RxToken = E1000RxToken;
//     type TxToken = E1000TxToken;

//     fn receive(&mut self) -> Option<(Self::RxToken, Self::TxToken)> {
//         self.0
//             .lock()
//             .receive()
//             .map(|vec| (E1000RxToken(vec), E1000TxToken(self.clone())))
//     }

//     fn transmit(&mut self) -> Option<Self::TxToken> {
//         if self.0.lock().can_send() {
//             Some(E1000TxToken(self.clone()))
//         } else {
//             None
//         }
//     }

//     fn capabilities(&self) -> DeviceCapabilities {
//         let mut caps = DeviceCapabilities::default();
//         caps.max_transmission_unit = 1536;
//         caps.max_burst_size = Some(64);
//         caps
//     }
// }

// impl phy::RxToken for E1000RxToken {
//     fn consume<R, F>(mut self, _timestamp: Instant, f: F) -> Result<R>
//     where
//         F: FnOnce(&mut [u8]) -> Result<R>,
//     {
//         // warn!("Enter : E1000TxToken");
//         f(&mut self.0)
//     }
// }

// impl phy::TxToken for E1000TxToken {
//     fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> Result<R>
//     where
//         F: FnOnce(&mut [u8]) -> Result<R>,
//     {
//         // warn!("Enter : E1000RxToken");
//         let mut buffer = [0u8; PAGE_SIZE];
//         let result = f(&mut buffer[..len]);

//         let mut driver = (self.0).0.lock();
//         driver.send(&buffer);

//         result
//     }
// }
