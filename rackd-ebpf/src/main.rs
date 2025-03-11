#![no_std]
#![no_main]
// #![allow(nonstandard_style, dead_code)]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

use core::mem;
use aya_ebpf::{bindings::xdp_action::{self}, macros::{map, xdp}, maps::Array, programs::XdpContext};
use aya_log_ebpf::info;
use network_types::{eth::{EthHdr, EtherType}, icmp::IcmpHdr, ip::{IpProto, Ipv4Hdr, Ipv6Hdr}, udp::UdpHdr};

#[map]
pub static mut IPV6_GATEWAY: Array<u128> = Array::with_max_entries(1, 0);
#[map]
pub static mut IPV4_GATEWAY: Array<u32> = Array::with_max_entries(1, 0);

#[xdp]
pub fn program(ctx: XdpContext) -> u32 {
    match try_work(ctx) {
        Ok(ret) => return ret,
        Err(_) => return xdp_action::XDP_ABORTED
    }

    fn try_work(ctx: XdpContext) -> Result<u32, ()> {
        let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
        // info!(&ctx, "received packet");
        match unsafe { (*ethhdr).ether_type } {
            EtherType::Ipv4 => watch_ipv4_gateway(&ctx),
            EtherType::Ipv6 => watch_ipv6_gateway(&ctx),
            _ => return Ok(xdp_action::XDP_PASS)
        }
    }

    fn watch_ipv4_gateway(ctx: &XdpContext) -> Result<u32, ()> {
        let hdr: *const Ipv4Hdr = unsafe { ptr_at(ctx, EthHdr::LEN)? };

        if unsafe { (*hdr).proto } != IpProto::Udp {
            return Ok(xdp_action::XDP_PASS);
        }

        let udphdr: *const UdpHdr = unsafe { ptr_at(ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
        // let src_port = u16::from_be(unsafe { (*udphdr).source });
        let dst_port = u16::from_be(unsafe { (*udphdr).dest });
        // info!(ctx, "UDP header - source: {}, dest: {}", src_port, dst_port);
        
        if dst_port != 68 {
            return Ok(xdp_action::XDP_PASS);
        }

        let ipv4_gateway = unsafe { 
            let ptr = IPV4_GATEWAY.get_ptr_mut(0).ok_or(())?;
            &mut *ptr
        };
        let ipv4_addr = unsafe { (*hdr).src_addr() };
        info!(ctx, "Gateway: {}", ipv4_addr);
        *ipv4_gateway = ipv4_addr.to_bits(); 
        Ok(xdp_action::XDP_PASS)
    }

    fn watch_ipv6_gateway(ctx: &XdpContext) -> Result<u32, ()> {
        let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
        match unsafe { (*ethhdr).ether_type } {
            EtherType::Ipv6 => {}
            _ => return Ok(xdp_action::XDP_PASS)
        }

        let ipv6hdr: *const Ipv6Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
        match unsafe { (*ipv6hdr).next_hdr } {
            IpProto::Ipv6Icmp => {}
            _ => return Ok(xdp_action::XDP_PASS)
        }

        let icmp6hdr: *const IcmpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)? };
        match unsafe { (*icmp6hdr).type_ } {
            134 => {},  // Router Advertisement
            _ => return Ok(xdp_action::XDP_PASS)
        }
        
        let ipv6_gateway = unsafe { 
            let ptr = IPV6_GATEWAY.get_ptr_mut(0).ok_or(())?;
            &mut *ptr
        };
        *ipv6_gateway = unsafe { (*ipv6hdr).src_addr() }.to_bits(); 
        Ok(xdp_action::XDP_PASS)
    }
}

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    let ptr = (start + offset) as *const T;
    Ok(&*ptr)
}

