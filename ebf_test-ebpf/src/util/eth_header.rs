#[repr(C, packed(1))]
pub struct EthHeader {
    pub dst_addr: [u8; 6],
    pub src_addr: [u8; 6],
    pub ether_type: u16,
}

#[repr(u16)]
pub enum EthType {
    IPv4 = 0x0800_u16.to_be(),
    ARP = 0x0806_u16.to_be(),
    IPv6 = 0x86DD_u16.to_be(),
}

pub const EthHdrLen: u16 = 14;
