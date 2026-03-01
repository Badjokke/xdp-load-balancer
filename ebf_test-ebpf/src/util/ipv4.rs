#[repr(C, packed(1))]
pub struct Ipv4Header {
    version_ihl: u8,
    dscp_ecn: u8,
    pub total_len: u16,
    pub identification: u16,
    flags_fragment_offset: u16,
    ttl_protocol: u16,
    pub header_checksum: u16,
    pub source_address: u32,
    pub destination_address: u32,
}
pub const IPV4_HDR_LEN: usize = 20;
impl Ipv4Header {
    pub fn get_version(&self) -> u8 {
        (self.version_ihl >> 4) & 0xF
    }

    pub fn get_dscp(&self) -> u8 {
        (self.dscp_ecn >> 2) & 0x3F
    }

    pub fn get_ecn(&self) -> u8 {
        self.dscp_ecn & 0x03
    }

    pub fn get_ihl(&self) -> u8 {
        self.version_ihl & 0xF
    }

    pub fn get_ttl(&self) -> u8 {
        ((self.ttl_protocol & 0xF) >> 8) as u8
    }

    pub fn get_protocol(&self) -> u8 {
        u16::from_be(self.ttl_protocol) as u8
    }

    pub fn get_ttl_protocol(&self) -> u16 {
        self.ttl_protocol
    }
}

#[repr(u8)]
pub enum IpVersion {
    Ipv4Version = 0x04_u8,
}

#[repr(u8)]
pub enum IpProtocol {
    UDP = 0x11_u8.to_be(),
    TCP = 0x60_u8.to_be(),
    CHAOS = 0x01_u8.to_be(),
}
