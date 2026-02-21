#[repr(C, packed(1))]
pub struct EthHeader {
    pub dst_addr: [u8; 6],
    pub src_addr: [u8; 6],
    pub ether_type: u16,
}
