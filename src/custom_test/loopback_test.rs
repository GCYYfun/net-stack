// loopback 测试
// 1. 测试 设备 初始化
// 2. 测试 设备 发送
// 3. 测试 设备 接受
// 4. 测试 XX
#[test]
fn loopback_test() {
    assert_eq!(1, 1);
}

#[test]
fn loopback_init() {
    use crate::loopback_init;
    loopback_init();
}

// 测试 网卡 的 创建
// 1. loopback
// 2. e1000

// 测试 socket
// 1. tcp socket api
// 2. udp socket api
