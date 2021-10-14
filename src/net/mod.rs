//! Linux socket objects
//!

/// missing documentation
pub mod socket_address;
pub use socket_address::*;

/// missing documentation
pub mod tcp;
pub use tcp::*;

/// missing documentation
pub mod udp;
pub use udp::*;

/// missing documentation
// pub mod raw;
// pub use raw::*;

/// missing documentation
// pub mod icmp;
// pub use icmp::*;

// pub mod stack;

#[cfg(feature = "e1000")]
use crate::poll_ifaces_e1000;

use log::*;

// ============= Socket Set =============

use crate::resource::get_net_sockets;
use spin::Mutex;

// ============= Socket Set =============

// ============= Define =============

// ========TCP

/// missing documentation
pub const TCP_SENDBUF: usize = 512 * 1024; // 512K
/// missing documentation
pub const TCP_RECVBUF: usize = 512 * 1024; // 512K

// ========UDP

/// missing documentation
pub const UDP_METADATA_BUF: usize = 1024;
/// missing documentation
pub const UDP_SENDBUF: usize = 64 * 1024; // 64K
/// missing documentation
pub const UDP_RECVBUF: usize = 64 * 1024; // 64K

// ========RAW

/// missing documentation
pub const RAW_METADATA_BUF: usize = 1024;
/// missing documentation
pub const RAW_SENDBUF: usize = 64 * 1024; // 64K
/// missing documentation
pub const RAW_RECVBUF: usize = 64 * 1024; // 64K

// ========RAW

/// missing documentation
pub const ICMP_METADATA_BUF: usize = 1024;
/// missing documentation
pub const ICMP_SENDBUF: usize = 64 * 1024; // 64K
/// missing documentation
pub const ICMP_RECVBUF: usize = 64 * 1024; // 64K

// ========Other

/// missing documentation
pub const IPPROTO_IP: usize = 0;
/// missing documentation
pub const IP_HDRINCL: usize = 3;

// ============= Define =============

// ============= SocketHandle =============

use smoltcp::socket::SocketHandle;

/// A wrapper for `SocketHandle`.
/// Auto increase and decrease reference count on Clone and Drop.
#[derive(Debug)]
struct GlobalSocketHandle(SocketHandle);

impl Clone for GlobalSocketHandle {
    fn clone(&self) -> Self {
        get_net_sockets().lock().retain(self.0);
        Self(self.0)
    }
}

impl Drop for GlobalSocketHandle {
    fn drop(&mut self) {
        let net_sockets = get_net_sockets();
        let mut sockets = net_sockets.lock();
        sockets.release(self.0);
        sockets.prune();

        // send FIN immediately when applicable
        drop(sockets);
        #[cfg(feature = "e1000")]
        unsafe { poll_ifaces_e1000() }
        #[cfg(feature = "loopback")]
        poll_ifaces_loopback();
    }
}


//  Safety: call this without SOCKETS locked
// #[cfg(feature = "e1000")]
// fn poll_ifaces_e1000() {
//     for iface in get_net_driver().iter() {
//         if let Ok(_) = iface.poll(&(*get_net_sockets())) {}
//     }
// }

#[cfg(feature = "loopback")]
use hashbrown::HashMap;
// use kernel_hal::timer_now;
#[cfg(feature = "loopback")]
use crate::{NetStack, NET_STACK};
#[cfg(feature = "loopback")]
use smoltcp::time::Instant;

#[cfg(feature = "loopback")]
use core::time::Duration;
#[cfg(feature = "loopback")]
pub fn timer_now() -> Duration {
    let tsc = unsafe { core::arch::x86_64::_rdtsc() };
    Duration::from_nanos(tsc * 1000 / 2600 as u64)
}

// /// miss doc
// #[cfg(feature = "loopback")]
// pub fn get_net_stack() -> HashMap<usize, Arc<dyn NetStack>> {
//     NET_STACK.read().clone()
// }

/// miss doc
#[cfg(feature = "loopback")]
pub fn get_net_stack() -> Arc<Mutex<HashMap<usize, Arc<dyn NetStack>>>> {
    NET_STACK.read().clone()
}

// /// miss doc
// #[cfg(feature = "loopback")]
// fn poll_ifaces_loopback() {
//     for (_key, stack) in get_net_stack().iter() {
//         let timestamp = Instant::from_millis(timer_now().as_millis() as i64);
//         stack.poll(&(*get_net_sockets()), timestamp);
//     }
// }

/// miss doc
#[cfg(feature = "loopback")]
fn poll_ifaces_loopback() {
    for (_key, stack) in get_net_stack().lock().iter() {
        let timestamp = Instant::from_millis(timer_now().as_millis() as i64);
        stack.poll(&(*get_net_sockets()), timestamp);
    }
}

use core::future::Future;
use core::pin::Pin;
use core::task::Context;
use core::task::Poll;

use smoltcp::socket::TcpSocket;
struct ConnectFuture<'a> {
    socket: &'a mut TcpSocket<'a>,
}

impl Future for ConnectFuture<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        use smoltcp::socket::TcpState;
        if self.socket.state() == TcpState::SynSent {
            self.socket.register_recv_waker(&_cx.waker().clone());
            self.socket.register_send_waker(&_cx.waker().clone());
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

struct AcceptFuture<'a> {
    socket: &'a mut TcpSocket<'a>,
}

impl Future for AcceptFuture<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.socket.is_active() {
            Poll::Ready(())
        } else {
            self.socket.register_recv_waker(&_cx.waker().clone());
            self.socket.register_send_waker(&_cx.waker().clone());
            Poll::Pending
        }
    }
}

// ============= SocketHandle =============

// ============= Endpoint =============

use smoltcp::wire::IpEndpoint;

/// missing documentation
#[derive(Clone, Debug)]
pub enum Endpoint {
    /// missing documentation
    Ip(IpEndpoint),
    /// missing documentation
    LinkLevel(LinkLevelEndpoint),
    /// missing documentation
    Netlink(NetlinkEndpoint),
}

/// missing documentation
#[derive(Clone, Debug)]
pub struct LinkLevelEndpoint {
    /// missing documentation
    pub interface_index: usize,
}

impl LinkLevelEndpoint {
    /// missing documentation
    pub fn new(ifindex: usize) -> Self {
        LinkLevelEndpoint {
            interface_index: ifindex,
        }
    }
}

/// missing documentation
#[derive(Clone, Debug)]
pub struct NetlinkEndpoint {
    /// missing documentation
    pub port_id: u32,
    /// missing documentation
    pub multicast_groups_mask: u32,
}

impl NetlinkEndpoint {
    /// missing documentation
    pub fn new(port_id: u32, multicast_groups_mask: u32) -> Self {
        NetlinkEndpoint {
            port_id,
            multicast_groups_mask,
        }
    }
}

// ============= Endpoint =============

// ============= Rand Port =============

fn rand() -> u64 {
    let mut rand = 0;
    unsafe{
        core::arch::x86_64::_rdrand64_step(&mut rand);
    }
    rand
}

#[allow(unsafe_code)]
/// missing documentation
fn get_ephemeral_port() -> u16 {
    // TODO selects non-conflict high port
    static mut EPHEMERAL_PORT: u16 = 0;
    unsafe {
        if EPHEMERAL_PORT == 0 {
            EPHEMERAL_PORT = (49152 + rand() % (65536 - 49152)) as u16;
        }
        if EPHEMERAL_PORT == 65535 {
            EPHEMERAL_PORT = 49152;
        } else {
            EPHEMERAL_PORT += 1;
        }
        EPHEMERAL_PORT
    }
}

// ============= Rand Port =============

// ============= Util =============

#[allow(unsafe_code)]
/// # Safety
/// Convert C string to Rust string
pub unsafe fn from_cstr(s: *const u8) -> &'static str {
    use core::{slice, str};
    let len = (0usize..).find(|&i| *s.add(i) == 0).unwrap();
    str::from_utf8(slice::from_raw_parts(s, len)).unwrap()
}

// ============= Util =============

use helper::error::*;
use alloc::boxed::Box;
use alloc::fmt::Debug;
use alloc::sync::Arc;
use async_trait::async_trait;
/// Common methods that a socket must have
#[async_trait]
pub trait Socket: Send + Sync + Debug {
    /// missing documentation
    async fn read(&self, data: &mut [u8]) -> (SysResult, Endpoint);
    /// missing documentation
    fn write(&self, data: &[u8], sendto_endpoint: Option<Endpoint>) -> SysResult;
    /// missing documentation
    fn poll(&self) -> (bool, bool, bool); // (in, out, err)
    /// missing documentation
    async fn connect(&self, endpoint: Endpoint) -> SysResult;
    /// missing documentation
    fn bind(&mut self, _endpoint: Endpoint) -> SysResult {
        Err(LxError::EINVAL)
    }
    /// missing documentation
    fn listen(&mut self) -> SysResult {
        Err(LxError::EINVAL)
    }
    /// missing documentation
    fn shutdown(&self) -> SysResult {
        Err(LxError::EINVAL)
    }
    /// missing documentation
    async fn accept(&mut self) -> LxResult<(Arc<Mutex<dyn Socket>>, Endpoint)> {
        Err(LxError::EINVAL)
    }
    /// missing documentation
    fn endpoint(&self) -> Option<Endpoint> {
        None
    }
    /// missing documentation
    fn remote_endpoint(&self) -> Option<Endpoint> {
        None
    }
    /// missing documentation
    fn setsockopt(&mut self, _level: usize, _opt: usize, _data: &[u8]) -> SysResult {
        // warn!("setsockopt is unimplemented");
        Ok(0)
    }
    /// missing documentation
    fn ioctl(&self, _request: usize, _arg1: usize, _arg2: usize, _arg3: usize) -> SysResult {
        // warn!("ioctl is unimplemented for this socket");
        Ok(0)
    }
    /// missing documentation
    fn fcntl(&self, _cmd: usize, _arg: usize) -> SysResult {
        // warn!("ioctl is unimplemented for this socket");
        Ok(0)
    }
}
