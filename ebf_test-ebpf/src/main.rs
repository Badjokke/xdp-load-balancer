#![no_std]
#![no_main]

use core::mem;
use aya_ebpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_log_ebpf::info;
mod util;
use util::EthHeader;

#[inline(always)]
fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()>{
    let start = ctx.data();
    let end = ctx.data_end();
    let size = mem::size_of::<T>();
    if start + offset + size > end {
        return Err(())
    }
    Ok((start + offset) as *const T)

}

#[xdp]
pub fn ebf_test(ctx: XdpContext) -> u32 {
    match try_ebf_test(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_ebf_test(ctx: XdpContext) -> Result<u32, ()> {
    info!(&ctx, "received a packet");
    let eth_header: *const EthHeader = ptr_at(&ctx, 0)?;
    let ip_version: u16 = unsafe{(*eth_header).ether_type};
    info!(&ctx, "protocol version: {}", ip_version);
    info!(&ctx, "protocol version: {}", 0x0800_u16.to_be());
    Ok(xdp_action::XDP_PASS)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
