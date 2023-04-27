#![allow(dead_code)]

// Sources: https://elixir.bootlin.com/linux/latest/source/include/uapi/linux/if.h#L92
// https://elixir.free-electrons.com/linux/v4.9/source/include/linux/netdevice.h#L1423
pub(crate) const IFF_NOARP: u32 = 1 << 7;
pub(crate) const IFF_MULTICAST: u32 = 1 << 12;
pub(crate) const IFF_LIVE_ADDR_CHANGE: u64 = 1 << 15;
pub(crate) const IFF_NO_QUEUE: u64 = 1 << 21;

// The rest of these are not easily available online
// I got these by running dummy.c and printing out their values
// See https://imgur.com/a/hi4ilNp
// These constants are long long unsigned int (%llu format specifier in C), i.e. u64 in Rust
// Some of these are bitwise OR's of other bits but I just hardcode them
pub(crate) const NETIF_F_SG: u64 = 1;
pub(crate) const NETIF_F_FRAGLIST: u64 = 64;
pub(crate) const NETIF_F_GSO_SOFTWARE: u64 = 1075642368;
pub(crate) const NETIF_F_HW_CSUM: u64 = 8;
pub(crate) const NETIF_F_HIGHDMA: u64 = 32;
pub(crate) const NETIF_F_LLTX: u64 = 4096;
pub(crate) const NETIF_F_GSO_ENCAP_ALL: u64 = 264241152;
