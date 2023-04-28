#[allow(unused)]

use kernel::{
    bindings,
    c_str,
    device,
    driver,
    net,
};
use kernel::prelude::*;
use core::ffi::c_char;
use core::ffi::c_void;
mod rust_dummy_defs;
use rust_dummy_defs::*;


// #define DRV_NAME         "dummy"
const temp: &'static [u8; 6] = b"dummy\x00";
const DRV_NAME: *const c_char = temp.as_ptr() as *const c_char;
const numdummies: i32 = 1;

/* fake multicast ability */
// static void set_multicast_list(struct net_device *dev)
// {
// }

fn set_multicast_list(dev: *mut bindings::net_device) {
}


// static void dummy_get_stats64(struct net_device *dev,
// 			      struct rtnl_link_stats64 *stats)
// {
// 	dev_lstats_read(dev, &stats->tx_packets, &stats->tx_bytes);
// }

fn dummy_get_stats64(dev: *mut bindings::net_device, stats: *mut bindings::rtnl_link_stats64) {
    unsafe {
        bindings::dev_lstats_read(dev, &mut (*stats).tx_packets, &mut (*stats).tx_bytes);
    }
}

// static netdev_tx_t dummy_xmit(struct sk_buff *skb, struct net_device *dev)
// {
// 	dev_lstats_add(dev, skb->len);

// 	skb_tx_timestamp(skb);
// 	dev_kfree_skb(skb);
// 	return NETDEV_TX_OK;
// }

fn dummy_xmit(skb: *mut bindings::sk_buff, dev: *mut bindings::net_device) -> i32 {
    unsafe {
        // TODO: these 2 functions don't exist in bindgen
        // bindings::dev_lstats_add(dev, (*skb).len);
        // bindings::skb_tx_timestamp(skb);
        bindings::__kfree_skb(skb);
    }
    bindings::netdev_tx_NETDEV_TX_OK
}

// static int dummy_dev_init(struct net_device *dev)
// {
// 	dev->lstats = netdev_alloc_pcpu_stats(struct pcpu_lstats);
// 	if (!dev->lstats)
// 		return -ENOMEM;

// 	return 0;
// }

fn dummy_dev_init(dev: *mut bindings::net_device) -> i32 {
    // TODO: netdev_alloc_pcpu_stats not in bindgen, ignoring
    return 0;
}

// static void dummy_dev_uninit(struct net_device *dev)
// {
// 	free_percpu(dev->lstats);
// }


// Skipped

// fn dummy_dev_uninit(dev: *mut bindings::net_device) {
//     unsafe {
            // I can get to lstats, but c_void is expected as the parameter here
//         bindings::free_percpu(((*dev).__bindgen_anon_1.lstats));
//     }
// }

// static int dummy_change_carrier(struct net_device *dev, bool new_carrier)
// {
// 	if (new_carrier)
// 		netif_carrier_on(dev);
// 	else
// 		netif_carrier_off(dev);
// 	return 0;
// }

fn dummy_change_carrier(dev: *mut bindings::net_device, new_carrier: bool) -> i32 {
    unsafe {
        if new_carrier {
            bindings::netif_carrier_on(dev);
        } else {
            bindings::netif_carrier_off(dev);
        }
    }
    0
}

unsafe extern "C" fn init_callback(dev: *mut bindings::net_device) -> core::ffi::c_int {
    dummy_dev_init(dev)
}

// unsafe extern "C" fn uninit_callback(dev: *mut bindings::net_device) {
//     dummy_dev_uninit(dev)
// }

unsafe extern "C" fn xmit_callback(skb: *mut bindings::sk_buff, dev: *mut bindings::net_device) -> core::ffi::c_int {
    dummy_xmit(skb, dev)
}

unsafe extern "C" fn set_multicast_list_callback(dev: *mut bindings::net_device) {
    set_multicast_list(dev)
}

unsafe extern "C" fn get_stats64_callback(dev: *mut bindings::net_device, stats: *mut bindings::rtnl_link_stats64) {
    dummy_get_stats64(dev, stats)
}

unsafe extern "C" fn change_carrier_callback(dev: *mut bindings::net_device, new_carrier: bool) -> core::ffi::c_int {
    dummy_change_carrier(dev, new_carrier)
}

// static const struct net_device_ops dummy_netdev_ops = {
// 	.ndo_init		= dummy_dev_init,
// 	.ndo_uninit		= dummy_dev_uninit,
// 	.ndo_start_xmit		= dummy_xmit,
// 	.ndo_validate_addr	= eth_validate_addr,
// 	.ndo_set_rx_mode	= set_multicast_list,
// 	.ndo_set_mac_address	= eth_mac_addr,
// 	.ndo_get_stats64	= dummy_get_stats64,
// 	.ndo_change_carrier	= dummy_change_carrier,
// };

const dummy_netdev_ops: bindings::net_device_ops = bindings::net_device_ops {
    ndo_init: Some(init_callback),
    ndo_uninit: None,
    ndo_start_xmit: Some(xmit_callback),
    // Ignore validate
    ndo_validate_addr: None,
    ndo_set_rx_mode: Some(set_multicast_list_callback),
    // Ignore set mac address
    // ndo_set_mac_address: bindings::eth_mac_addr,
    ndo_set_mac_address: None,
    ndo_get_stats64: Some(get_stats64_callback),
    ndo_change_carrier: Some(change_carrier_callback),

    // Get compile errors if these aren't here
    // This list is taken from https://github.com/fujita/rust-e1000/blob/master/ops.rs#L30

    ndo_open: None,
    ndo_stop: None,
    ndo_features_check: None,
    ndo_select_queue: None,
    ndo_change_rx_flags: None,
    ndo_do_ioctl: None,
    ndo_eth_ioctl: None,
    ndo_siocbond: None,
    ndo_siocwandev: None,
    ndo_siocdevprivate: None,
    ndo_set_config: None,
    ndo_change_mtu: None,
    ndo_neigh_setup: None,
    ndo_tx_timeout: None,
    ndo_has_offload_stats: None,
    ndo_get_offload_stats: None,
    ndo_get_stats: None,
    ndo_vlan_rx_add_vid: None,
    ndo_vlan_rx_kill_vid: None,
    // ndo_poll_controller: None,
    // ndo_netpoll_setup: None,
    // ndo_netpoll_cleanup: None,
    ndo_set_vf_mac: None,
    ndo_set_vf_vlan: None,
    ndo_set_vf_rate: None,
    ndo_set_vf_spoofchk: None,
    ndo_set_vf_trust: None,
    ndo_get_vf_config: None,
    ndo_set_vf_link_state: None,
    ndo_get_vf_stats: None,
    ndo_set_vf_port: None,
    ndo_get_vf_port: None,
    ndo_get_vf_guid: None,
    ndo_set_vf_guid: None,
    ndo_set_vf_rss_query_en: None,
    ndo_setup_tc: None,
    // ndo_fcoe_enable: None,
    // ndo_fcoe_disable: None,
    // ndo_fcoe_ddp_setup: None,
    // ndo_fcoe_ddp_done: None,
    // ndo_fcoe_ddp_target: None,
    // ndo_fcoe_get_hbainfo: None,
    // ndo_fcoe_get_wwn: None,
    ndo_rx_flow_steer: None,
    ndo_add_slave: None,
    ndo_del_slave: None,
    ndo_get_xmit_slave: None,
    ndo_sk_get_lower_dev: None,
    ndo_fix_features: None,
    ndo_set_features: None,
    ndo_neigh_construct: None,
    ndo_neigh_destroy: None,
    ndo_fdb_add: None,
    ndo_fdb_del: None,
    ndo_fdb_del_bulk: None,
    ndo_fdb_dump: None,
    ndo_fdb_get: None,
    ndo_bridge_setlink: None,
    ndo_bridge_getlink: None,
    ndo_bridge_dellink: None,
    ndo_get_phys_port_id: None,
    ndo_get_port_parent_id: None,
    ndo_get_phys_port_name: None,
    ndo_dfwd_add_station: None,
    ndo_dfwd_del_station: None,
    ndo_set_tx_maxrate: None,
    ndo_get_iflink: None,
    ndo_fill_metadata_dst: None,
    ndo_set_rx_headroom: None,
    ndo_bpf: None,
    ndo_xdp_xmit: None,
    ndo_xdp_get_xmit_slave: None,
    ndo_xsk_wakeup: None,
    ndo_get_devlink_port: None,
    ndo_tunnel_ctl: None,
    ndo_get_peer_dev: None,
    ndo_fill_forward_path: None,
    ndo_get_tstamp: None,
};

// static const struct ethtool_ops dummy_ethtool_ops = {
// 	.get_ts_info		= ethtool_op_get_ts_info,
// };

// TODO: Might be unnecessary
const dummy_ethtoolops: bindings::ethtool_ops = bindings::ethtool_ops {
    get_ts_info: Some(bindings::ethtool_op_get_ts_info),
    _bitfield_1: bindings::__BindgenBitfieldUnit::new([0; 1]),
    supported_coalesce_params: 0,
    supported_ring_params: 0,
    get_drvinfo: None,
    get_regs_len: None,
    get_regs: None,
    get_wol: None,
    set_wol: None,
    get_msglevel: None,
    set_msglevel: None,
    nway_reset: None,
    get_link: None,
    get_link_ext_state: None,
    get_eeprom_len: None,
    get_eeprom: None,
    set_eeprom: None,
    get_coalesce: None,
    set_coalesce: None,
    get_ringparam: None,
    set_ringparam: None,
    get_pause_stats: None,
    get_pauseparam: None,
    set_pauseparam: None,
    self_test: None,
    get_strings: None,
    set_phys_id: None,
    get_ethtool_stats: None,
    begin: None,
    complete: None,
    get_priv_flags: None,
    set_priv_flags: None,
    get_sset_count: None,
    get_rxnfc: None,
    set_rxnfc: None,
    flash_device: None,
    reset: None,
    get_rxfh_key_size: None,
    get_rxfh_indir_size: None,
    get_rxfh: None,
    set_rxfh: None,
    get_rxfh_context: None,
    set_rxfh_context: None,
    get_channels: None,
    set_channels: None,
    get_dump_flag: None,
    get_dump_data: None,
    set_dump: None,
    get_module_info: None,
    get_module_eeprom: None,
    get_eee: None,
    set_eee: None,
    get_tunable: None,
    set_tunable: None,
    get_per_queue_coalesce: None,
    set_per_queue_coalesce: None,
    get_link_ksettings: None,
    set_link_ksettings: None,
    get_fec_stats: None,
    get_fecparam: None,
    set_fecparam: None,
    get_ethtool_phy_stats: None,
    get_phy_tunable: None,
    set_phy_tunable: None,
    get_module_eeprom_by_page: None,
    get_eth_phy_stats: None,
    get_eth_mac_stats: None,
    get_eth_ctrl_stats: None,
    get_rmon_stats: None,
    get_module_power_mode: None,
    set_module_power_mode: None,
};

// static void dummy_setup(struct net_device *dev)
// {
// 	ether_setup(dev);

// 	/* Initialize the device structure. */
// 	dev->netdev_ops = &dummy_netdev_ops;
// 	dev->ethtool_ops = &dummy_ethtool_ops;
// 	dev->needs_free_netdev = true;

// 	/* Fill in device structure with ethernet-generic values. */
// 	dev->flags |= IFF_NOARP;
// 	dev->flags &= ~IFF_MULTICAST;
// 	dev->priv_flags |= IFF_LIVE_ADDR_CHANGE | IFF_NO_QUEUE;
// 	dev->features	|= NETIF_F_SG | NETIF_F_FRAGLIST;
// 	dev->features	|= NETIF_F_GSO_SOFTWARE;
// 	dev->features	|= NETIF_F_HW_CSUM | NETIF_F_HIGHDMA | NETIF_F_LLTX;
// 	dev->features	|= NETIF_F_GSO_ENCAP_ALL;
// 	dev->hw_features |= dev->features;
// 	dev->hw_enc_features |= dev->features;
// 	eth_hw_addr_random(dev);

// 	dev->min_mtu = 0;
// 	dev->max_mtu = 0;
// }

// https://elixir.free-electrons.com/linux/v4.7/source/include/linux/etherdevice.h#L221
#[inline]
fn eth_random_addr(addr: *mut u8) {
    bindings::get_random_bytes(addr as *mut c_void, ETH_ALEN);
    *addr = *addr & 0xFE;           // Clear multicast bit
    *addr = *addr | 0x02;           // Set local assignment bit (IEEE802)
}

// https://elixir.bootlin.com/linux/v4.9/source/include/linux/etherdevice.h#L261
#[inline]
fn eth_hw_addr_random(dev: *mut bindings::net_device) {
    (*dev).addr_assign_type = NET_ADDR_RANDOM;
    let mut byte = (*dev).dev_addr;
    eth_random_addr(byte);
}

fn dummy_setup(dev: *mut bindings::net_device) {
    unsafe {
        bindings::ether_setup(dev);
        (*dev).netdev_ops = &dummy_netdev_ops;
        (*dev).ethtool_ops = &dummy_ethtoolops;
        (*dev).needs_free_netdev = true;
        (*dev).flags |= IFF_NOARP;
        // Rust's bitwise not is !, not ~
        (*dev).flags &= !IFF_MULTICAST;
        (*dev).priv_flags |= IFF_LIVE_ADDR_CHANGE | IFF_NO_QUEUE;
        (*dev).features |= NETIF_F_SG | NETIF_F_FRAGLIST;
        (*dev).features |= NETIF_F_GSO_SOFTWARE;
        (*dev).features |= NETIF_F_HW_CSUM | NETIF_F_HIGHDMA | NETIF_F_LLTX;
        (*dev).features |= NETIF_F_GSO_ENCAP_ALL;
        (*dev).hw_features |= (*dev).features;
        (*dev).hw_enc_features |= (*dev).features;
        // This is a static inline function and thus is not in bindings
        // Therefore it is re-implemented here
        eth_hw_addr_random(dev);
        (*dev).min_mtu = 0;
        (*dev).max_mtu = 0;
    }
}

unsafe extern "C" fn setup_callback(dev: *mut bindings::net_device) {
    dummy_setup(dev);
}

// static int dummy_validate(struct nlattr *tb[], struct nlattr *data[],
// 			  struct netlink_ext_ack *extack)
// {
// 	if (tb[IFLA_ADDRESS]) {
// 		if (nla_len(tb[IFLA_ADDRESS]) != ETH_ALEN)
// 			return -EINVAL;
// 		if (!is_valid_ether_addr(nla_data(tb[IFLA_ADDRESS])))
// 			return -EADDRNOTAVAIL;
// 	}
// 	return 0;
// }

// fn dummy_validate(tb: *mut *mut bindings::nlattr, data: *mut *mut bindings::nlattr, extack: *mut bindings::netlink_ext_ack) -> i32 {
//     unsafe {
//         if !tb.is_null() && !(*tb.offset(bindings::IFLA_ADDRESS as isize)).is_null() {
//             // TODO: nla_len doesn't exist
//             if bindings::nla_len(*tb.offset(bindings::IFLA_ADDRESS as isize)) != bindings::ETH_ALEN {
//                 return -bindings::EINVAL;
//             }
//             // TODO: is_valid_ether_addr doesn't exist
//             if bindings::is_valid_ether_addr(bindings::nla_data(*tb.offset(bindings::IFLA_ADDRESS as isize))) == 0 {
//                 return -bindings::EADDRNOTAVAIL;
//             }
//         }
//     }
//     0
// }

// static struct rtnl_link_ops dummy_link_ops __read_mostly = {
// 	.kind		= DRV_NAME,
// 	.setup		= dummy_setup,
// 	.validate	= dummy_validate,
// };

// TODO: not sure about read_mostly in the original C
// Skip validate because nla_len does not exist in bindings, and validate is not an important function

const lhead: bindings::list_head = bindings::list_head {
    next: 0 as *mut bindings::list_head,
    prev: 0 as *mut bindings::list_head,
};

// Do NOT try to initialize without default(), there are many weird fields
const policy: bindings::nla_policy = Default::default();
const mut_policy: *mut bindings::nla_policy = &mut policy;

const dummy_link_ops: bindings::rtnl_link_ops = bindings::rtnl_link_ops {
    kind: DRV_NAME,
    setup: Some(setup_callback),
    // Ignore validate because nla_len does not exist in bindings
    // Also, validate isn't important
    // validate: dummy_validate,
    alloc: None,
    changelink: None,
    dellink: None,
    fill_info: None,
    fill_linkxstats: None,
    fill_slave_info: None,
    fill_xstats: None,
    get_link_net: None,
    get_linkxstats_size: None,
    get_num_rx_queues: None,
    get_num_tx_queues: None,
    get_size: None,
    get_slave_size: None,
    get_xstats_size: None,
    list: lhead,
    maxtype: 0,
    netns_refund: false,
    newlink: None,
    policy: mut_policy,
    priv_size: 0,
    slave_changelink: None,
    slave_maxtype: 0,
    slave_policy: mut_policy,
    validate: None,
};

// /* Number of dummy devices to be set up by this module. */
// module_param(numdummies, int, 0);
// MODULE_PARM_DESC(numdummies, "Number of dummy pseudo devices");

// static int __init dummy_init_one(void)
// {
// 	struct net_device *dev_dummy;
// 	int err;

// 	dev_dummy = alloc_netdev(0, "dummy%d", NET_NAME_ENUM, dummy_setup);
// 	if (!dev_dummy)
// 		return -ENOMEM;

// 	dev_dummy->rtnl_link_ops = &dummy_link_ops;
// 	err = register_netdevice(dev_dummy);
// 	if (err < 0)
// 		goto err;
// 	return 0;

// err:
// 	free_netdev(dev_dummy);
// 	return err;
// }

fn dummy_init_one() -> i32 {
    // alloc_netdev is a #define macro and thus not in bindings
    // However, it's just an alias for a call to alloc_netdev_mqs
    // Source: https://elixir.bootlin.com/linux/v4.4/source/include/linux/netdevice.h#L3407
    // BINDINGS::NET_NAME_ENUM is defined as u32 but alloc_netdev_mqs expects u8
    // The cast is fine since the actual value is 1
    let dev_dummy = unsafe { bindings::alloc_netdev_mqs(0, DRV_NAME, bindings::NET_NAME_ENUM as u8, Some(setup_callback), 1, 1) };
    if dev_dummy.is_null() {
        return -(bindings::ENOMEM as i32);
    }
    unsafe {
        (*dev_dummy).rtnl_link_ops = &dummy_link_ops;
    }
    let err = unsafe { bindings::register_netdevice(dev_dummy) };
    if err < 0 {
        unsafe {
            bindings::free_netdev(dev_dummy);
        }
        return err;
    }
    0
}

// static int __init dummy_init_module(void)
// {
// 	int i, err = 0;

// 	down_write(&pernet_ops_rwsem);
// 	rtnl_lock();
// 	err = __rtnl_link_register(&dummy_link_ops);
// 	if (err < 0)
// 		goto out;

// 	for (i = 0; i < numdummies && !err; i++) {
// 		err = dummy_init_one();
// 		cond_resched();
// 	}
// 	if (err < 0)
// 		__rtnl_link_unregister(&dummy_link_ops);

// out:
// 	rtnl_unlock();
// 	up_write(&pernet_ops_rwsem);

// 	return err;
// }

// Boilerplate from other Rust kernel modules (e.g. Rust e1000, NVMe driver)
struct Dummy {

}

// Boilerplate from other Rust kernel modules (e.g. Rust e1000, NVMe driver)
impl kernel::Module for Dummy {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        let mut err = 0;
        unsafe {
            bindings::down_write(&mut bindings::pernet_ops_rwsem);
            bindings::rtnl_lock();
            err = bindings::__rtnl_link_register(&mut dummy_link_ops);
            if err < 0 {
                bindings::rtnl_unlock();
                bindings::up_write(&mut bindings::pernet_ops_rwsem);
                // Can't return err here, return type is Result<Self>
            }

            // Original code is (int i = 0; i < numdummies && !err; i++)
            for i in 0..numdummies {
                if err != 0 {
                    break;
                }
                err = dummy_init_one();
                bindings::cond_resched();
            }
            if err < 0 {
                bindings::__rtnl_link_unregister(&mut dummy_link_ops);
            }
        }
        Ok(Dummy{})
    }
}

// Don't need exit

// static void __exit dummy_cleanup_module(void)
// {
// 	rtnl_link_unregister(&dummy_link_ops);
// }

// module_init(dummy_init_module);
// module_exit(dummy_cleanup_module);
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_RTNL_LINK(DRV_NAME);

module! {
    type: Dummy,
    name: "rust_dummy",
    author: "Jesse Wei and Madison Lester",
    description: "Rust dummy network driver",
    license: "GPL v2",
}
