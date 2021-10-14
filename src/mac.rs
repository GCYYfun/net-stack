// helper

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

// smoltcp

// driver
use hardware::mac::Mac;

// crate



// pub struct MacDevice(Arc<Mutex<Mac<Provider>>>);
pub struct MacDevice(Arc<Mutex<Mac>>);
pub struct MacRxToken(Vec<u8>);
pub struct MacTxToken(MacDevice);

impl phy::Device<'_> for MacDevice {
    type RxToken = MacRxToken;
    type TxToken = MacTxToken;

    fn receive(&mut self) -> Option<(Self::RxToken,Self::TxToken)> {

    }
    fn transmit(&mut self) -> Option<Self::TxToken> -> {

    }
    fn capabilities(&self) -> DeviceCapabilities -{

    };
}

impl phy::TxToken for MacTxToken {
    fn consume<R, F>(self, len:usize,f:F) -> Result<R>
    where
        F:FnOnce(&mut [u8]) -> Result<R>,
    {

    }
}

impl phy::RxToken for MacRxToken {
    fn consume<R,F>(self,len:usize,f:F) -> Result<R>
    where
        F:FnOnce(&mut [u8]) -> Result<R>,
    {

    }
}

pub fn mac_init() {
    
}