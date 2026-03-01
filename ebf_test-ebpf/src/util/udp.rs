#[repr(C, packed(1))]
pub struct Datagram {
    pub source_port: u16,
    pub destination_port: u16,
    pub length: u16,
    pub checksum: u16,
}
pub const UDP_HEADER_LEN: usize = 8;

impl Datagram {
    pub fn get_payload_length(&self) -> u16 {
        self.length - UDP_HEADER_LEN as u16
    }
}
