#![no_std]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::custom_test::test_runner)]

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod custom_test;

// mod e1000;
// pub use e1000::*;


#[macro_use]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec::Vec;

// smoltcp
use smoltcp::iface::{Interface, InterfaceBuilder, NeighborCache, Route, Routes};
use smoltcp::phy::{self, Device, DeviceCapabilities, Loopback, Medium};
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use smoltcp::socket::SocketSet;
use smoltcp::time::Instant;


// helper
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use spin::Mutex;
use spin::RwLock;


pub trait NetStack : Send + Sync {
    // fn interfaces(&self) -> Arc<Mutex<HashMap<usize, Interface<'static, D>>>>;
    fn poll(&self,socketset: &Mutex<SocketSet>,timestamp:Instant);
}


lazy_static! {
    // may be RwLock later ??
    // usize 要 改称 名字 或 索引 用的 \结构 要 能 hash\暂时 usize  试一下
    pub static ref NET_STACK : RwLock<HashMap<usize,Arc<dyn NetStack>>> = RwLock::new(HashMap::new());
}

pub struct InterfaceInfo {
    name: String,
}

impl InterfaceInfo {
    fn new(name: String) -> Self {
        Self { name }
    }
}


pub struct Stack<D> where D:for<'d> Device<'d> + Send + Sync  {
    // iface 需要clone 和 内部 可变
    pub interfaces: Arc<Mutex<HashMap<usize, Interface<'static, D>>>>,
    // resource
    // later...
    // config
    // later...
}

impl<D> NetStack for Stack<D> where D: for<'d> Device<'d> + Send + Sync{
    // fn interfaces(&self) -> Arc<Mutex<HashMap<usize, Interface<'static, D>>>> {
    //     self.interfaces.clone()
    // }

    fn poll(&self,socketset: &Mutex<SocketSet>,timestamp:Instant) {
        // self.interfaces.get();
        // poll()
        // 遍历 
        for (key,value) in self.interfaces.lock().iter_mut() {
            // warn!("key : {}",key);
            let mut sockets = socketset.lock();
            match value.poll(&mut sockets, timestamp) {
                Ok(_) => {
                    // warn!("now in impl NetDriver for E1000Interface poll need SOCKET_ACTIVITY.notify_all()");
                }
                Err(err) => {
                    debug!("poll got err {}", err);
                }
            }
        }
    }
}

pub fn init() {
    warn!("hello");

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
    static mut routes_storage: [Option<(IpCidr, Route)>; 1] = [None; 1];
    let mut routes = unsafe { Routes::new(&mut routes_storage[..]) };
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

    // 把 栈 返回
    let mut stack = Stack {
        interfaces: Arc::new(Mutex::new(HashMap::new())),
    };
    stack.interfaces.lock().insert(0, iface);

    let s = Arc::new(stack);
    NET_STACK.write().insert(0,s);
}