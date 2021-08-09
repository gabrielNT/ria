use rand::RngCore;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octect = &self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octect[0], octect[1], octect[2], octect[3], octect[4], octect[5]
        )
    }
}

impl MacAddress {
    fn new() -> MacAddress {
        let mut octects: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octects);
        octects[0] |= 0b_0000_0011;
        MacAddress {0: octects}
    }
    
    fn is_local(&self) -> bool {
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }

    fn is_unicast(&self) -> bool {
        (self.0[0] & 0b_0000_0001) == 0b_0000_0001
    }
}

fn main() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("mac: {}", mac);
}
