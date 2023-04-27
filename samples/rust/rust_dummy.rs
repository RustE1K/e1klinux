#[allow(unused)]

use kernel::{
    bindings,
    c_str,
    device,
    driver,
    net,
};
use kernel::prelude::*;
use core::ffi::c_void;

mod rust_dummy_defs;
use rust_dummy_defs::*;

// #define DRV_NAME         "dummy"
// const DRV_NAME: &str = "dummy";
const numdummies: i32 = 1;

/* fake multicast ability */
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

// TODO: these 2 functions don't exist in bindgen
fn dummy_xmit(skb: *mut bindings::sk_buff, dev: *mut bindings::net_device) -> i32 {
    unsafe {
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
    return 0;
}

// static void dummy_dev_uninit(struct net_device *dev)
// {
// 	free_percpu(dev->lstats);
// }

fn dummy_dev_uninit(dev: *mut bindings::net_device) {
    unsafe {
        bindings::free_percpu((*dev).stats);
    }
}

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
    ndo_init: Some(dummy_dev_init),
    ndo_uninit: dummy_dev_uninit,
    ndo_start_xmit: dummy_xmit,
    // Ignore validate
    ndo_validate_addr: None,
    ndo_set_rx_mode: set_multicast_list,
    // Ignore set mac address
    // ndo_set_mac_address: bindings::eth_mac_addr,
    ndo_set_mac_address: None,
    ndo_get_stats64: dummy_get_stats64,
    ndo_change_carrier: dummy_change_carrier,

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

const dummy_ethtoolops: bindings::ethtool_ops = bindings::ethtool_ops {
    get_ts_info: bindings::ethtool_op_get_ts_info,
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
        bindings::eth_hw_addr_random(dev);
        (*dev).min_mtu = 0;
        (*dev).max_mtu = 0;
    }
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
// Also, skip validate because nla_len does not exist in bindings, and validate is not an important function
const dummy_link_ops: bindings::rtnl_link_ops = bindings::rtnl_link_ops {
    // kind: DRV_NAME,
    setup: dummy_setup,
    // Ignore validate
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
    kind: None,
    list: None,
    maxtype: None,
    netns_refund: None,
    newlink: None,
    policy: None,
    priv_size: None,
    slave_changelink: None,
    slave_maxtype: None,
    slave_policy: None,
    validate: None,
};

// /* Number of dummy devices to be set up by this module. */
// module_param(numdummies, int, 0);
// MODULE_PARM_DESC(numdummies, "Number of dummy pseudo devices");

// TODO: Not sure how to handle

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
    let dev_dummy = unsafe { bindings::alloc_netdev(0, "dummy%d", bindings::NET_NAME_ENUM, dummy_setup) };
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

            // Original code said for (int i = 0; i < numdummies && !err; i++)
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
    author: "Jesse Wei"
    description: "Rust dummy network driver",
    license: "GPL v2",
}
