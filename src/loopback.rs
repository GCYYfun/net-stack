
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

    let w = NET_STACK.write();
    w.lock().insert(0, Arc::new(stack));

}


// {
//     pub struct LoopBack {
//         queue:VecDeque<Vec<u8>>
//     }

//     impl LoopBack {
//         pub fn new() {
//             queue:VecDeque::new()
//         }
//     }

//     impl Device for LoopBack {
//         fn receive();
//         fn transmit();
//         fn capabilities();
//     }

//     pub struct LoopBackRxToken {
//         buffer:Vec<u8>
//     }

//     pub struct LoopBackTxToken {
//         queue:VecDeque<Vec<u8>>
//     }
// }