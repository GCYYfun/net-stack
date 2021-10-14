// helper

// alloc
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

// smoltcp
use smoltcp::iface::Interface;
use smoltcp::iface::InterfaceBuilder;
use smoltcp::iface::NeighborCache;
use smoltcp::iface::Route;
use smoltcp::iface::Routes;
use smoltcp::phy::Device;
use smoltcp::phy::{self, DeviceCapabilities};
use smoltcp::socket::SocketSet;
use smoltcp::time::Instant;
use smoltcp::wire::EthernetAddress;
use smoltcp::wire::IpAddress;
use smoltcp::wire::IpCidr;
use smoltcp::wire::Ipv4Address;
use smoltcp::Result;
// driver
use isomorphic_drivers::net::ethernet::intel::e1000::E1000;
use isomorphic_drivers::net::ethernet::structs::EthernetAddress as DriverEthernetAddress;
use isomorphic_drivers::provider;
// crate
const PAGE_SIZE:usize = 4096;

use spin::Mutex;
use hashbrown::HashMap;
// pub struct MacDevice(Arc<Mutex<Mac<Provider>>>);
pub struct E1000Device(Arc<Mutex<E1000<Provider>>>);
pub struct E1000RxToken(Vec<u8>);
pub struct E1000TxToken(E1000Device);

impl E1000Device{
    fn poll(&self,socketset: &Mutex<SocketSet>,timestamp:Instant){};
}

impl phy::Device<'_> for E1000 {
    type RxToken = E1000RxToken;
    type TxToken = E1000TxToken;

    fn receive(&mut self) -> Option<(Self::RxToken,Self::TxToken)> {
        self
        .receive()
        .map(|vec| (E1000RxToken(vec), E1000TxToken(self.clone())))
    }
    fn transmit(&mut self) -> Option<Self::TxToken> {
        if self.can_send() {
            Some(E1000TxToken(self.clone()))
        } else {
            None
        }
    }
    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1536;
        caps.max_burst_size = Some(64);
        caps
    };
}

impl phy::TxToken for E1000TxToken {
    fn consume<R, F>(self, len:usize,f:F) -> Result<R>
    where
        F:FnOnce(&mut [u8]) -> Result<R>,
    {
        let mut buffer = [0u8; 4096];
        let result = f(&mut buffer[..len]);
        let mut driver = (self.0).0.lock();
        driver.send(&buffer);

        result
    }
}

impl phy::RxToken for E1000RxToken {
    fn consume<R,F>(self,len:usize,f:F) -> Result<R>
    where
        F:FnOnce(&mut [u8]) -> Result<R>,
    {
        f(&mut self.0)
    }
}


pub fn e1000_init() {
    warn!("Probing e1000 {}", name);

    // randomly generated
    let mac: [u8; 6] = [0x52, 0x54, 0x98, 0x76, 0x54, 0x32];
    // 52:54:98:76:54:32

    let e1000 = E1000::new(header, size, DriverEthernetAddress::from_bytes(&mac));

    let net_driver = E1000Driver(Arc::new(Mutex::new(e1000)));

    let ethernet_addr = EthernetAddress::from_bytes(&mac);
    let ip_addrs = [IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24)];
    // let ip_addrs = [IpCidr::new(IpAddress::v4(127,0, 0,1), 24)];
    let default_gateway = Ipv4Address::new(10, 0, 2, 2);
    // let default_gateway = Ipv4Address::new(127, 0, 0, 1);
    let neighbor_cache = NeighborCache::new(BTreeMap::new());
    static mut routes_storage: [Option<(IpCidr, Route)>; 1] = [None; 1];
    let mut routes = unsafe { Routes::new(&mut routes_storage[..]) };
    routes.add_default_ipv4_route(default_gateway).unwrap();
    let iface = InterfaceBuilder::new(e1000)
        .ethernet_addr(ethernet_addr)
        .ip_addrs(ip_addrs)
        .routes(routes)
        .neighbor_cache(neighbor_cache)
        .finalize();

    // warn!("e1000 interface {} up with addr 10.0.{}.2/24", name, index);
    // let e1000_iface = E1000Interface {
    //     iface: Mutex::new(iface),
    //     driver: net_driver.clone(),
    //     name,
    //     irq,
    // };
    // // 把 栈 返回
    let mut stack = Stack {
        
        interfaces: Arc::new(Mutex::new(HashMap::new())),
        // interfaces: Mutex::new(HashMap::new()),
    };
    // Mutex::new();
    stack.interfaces.lock().insert(0, iface);

    // let x = *NET_STACK.read();
    let s = Arc::new(stack);
    NET_STACK.write().insert(1,s);
}


pub struct Provider;

impl provider::Provider for Provider {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_dma(size: usize) -> (usize, usize) {
        let paddr = virtio_dma_alloc(size / PAGE_SIZE);
        let vaddr = phys_to_virt(paddr);
        (vaddr, paddr)
    }

    fn dealloc_dma(vaddr: usize, size: usize) {
        let paddr = virt_to_phys(vaddr);
        for i in 0..size / PAGE_SIZE {
            unsafe {
                dealloc_frame(&(paddr + i * PAGE_SIZE));
            }
        }
    }
}

fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    unsafe { PMEM_BASE + paddr }
}

fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    unsafe { vaddr - PMEM_BASE }
}