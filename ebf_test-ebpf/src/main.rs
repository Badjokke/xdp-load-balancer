#![no_std]
#![no_main]

use core::mem;
use aya_ebpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_log_ebpf::info;
mod util;
use util::{EthHeader, EthType};

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
    let eth_header: *mut EthHeader = ptr_at(&ctx, 0)?;
    let ip_version: u16 = unsafe{(*eth_header).ether_type};
    if ip_version != EthType::IPv4 as u16 {
        return Ok(xdp_action::XDP_PASS);
    }
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
