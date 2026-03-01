#![no_std]
#![no_main]

use core::mem;
use aya_ebpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_log_ebpf::info;
mod util;
use util::{ETH_HDR_LEN, EthHeader, EthType};

use crate::util::{Datagram, IPV4_HDR_LEN, IpProtocol, Ipv4Header};

#[inline(always)]
fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*mut T, ()>{
    let start = ctx.data();
    let end = ctx.data_end();
    let size = mem::size_of::<T>();
    if start + offset + size > end {
        return Err(())
    }
    Ok((start + offset) as *mut T)

}

#[xdp]
pub fn forward(ctx: XdpContext) -> u32 {
    match try_forward(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_forward(ctx: XdpContext) -> Result<u32, ()> {
    info!(&ctx, "received a packet");
    let eth_header: *const EthHeader = ptr_at(&ctx, 0)?;
    let ip_version: u16 = unsafe{(*eth_header).ether_type};
    if ip_version != EthType::IPv4 as u16 {
        return Ok(xdp_action::XDP_PASS);
    }
    let ipv4_header: *const Ipv4Header = ptr_at(&ctx, ETH_HDR_LEN)?;
    let protocol: u8 = unsafe{(*ipv4_header).get_protocol()};
    if protocol != IpProtocol::UDP as u8{
        info!(&ctx, "Not udp protocol, passing as is.");
        return Ok(xdp_action::XDP_PASS);
    }
    return forward_udp_traffic_to_port(ctx, 8005);
}

fn forward_udp_traffic_to_port(ctx: XdpContext, forward_to_port_number: u16) -> Result<u32, ()> {
    let udp_header: *mut Datagram = ptr_at(&ctx, ETH_HDR_LEN + IPV4_HDR_LEN)?;
    let src_port = unsafe{(*udp_header).source_port};
    info!(&ctx, "source port: {}, forwarding to: {}", u16::from_be(src_port), forward_to_port_number);
    unsafe{(*udp_header).destination_port = forward_to_port_number.to_be()};
    Ok(xdp_action::XDP_PASS)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 4] = *b"MIT\0";
