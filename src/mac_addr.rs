use std::fmt::*;

#[derive(Copy, Clone)]
pub struct MacAddr(u8, u8, u8, u8, u8, u8);

impl Debug for MacAddr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "MacAddr({:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?})",
            self.0, self.1, self.2, self.3, self.4, self.5
        )
    }
}

impl Display for MacAddr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}",
            self.0, self.1, self.2, self.3, self.4, self.5
        )
    }
}
