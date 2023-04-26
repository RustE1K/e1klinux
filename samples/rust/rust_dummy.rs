#[allow(unused)]

use kernel::{
    bindings,
};


/* fake multicast ability */
fn set_multicast_list(dev: *mut bindings::net_device) -> Option<i32> {
    None
}

// static void dummy_get_stats64(struct net_device *dev,
// 			      struct rtnl_link_stats64 *stats)
// {
// 	dev_lstats_read(dev, &stats->tx_packets, &stats->tx_bytes);
// }

fn dummy_get_stats64(dev: *mut bindings::net_device, stats: *mut bindings::rtnl_link_stats64) -> Option<i32> {
    unsafe {
        bindings::dev_lstats_read(dev, &mut (*stats).tx_packets, &mut (*stats).tx_bytes);
    }
    None
}

// static netdev_tx_t dummy_xmit(struct sk_buff *skb, struct net_device *dev)
// {
// 	dev_lstats_add(dev, skb->len);

// 	skb_tx_timestamp(skb);
// 	dev_kfree_skb(skb);
// 	return NETDEV_TX_OK;
// }

// TODO: these 2 functions don't exist in bindgen
fn dummy_xmit(skb: *mut bindings::sk_buff, dev: *mut bindings::net_device) -> Option<i32> {
    unsafe {
        // bindings::dev_lstats_add(dev, (*skb).len);
        // bindings::skb_tx_timestamp(skb);
        bindings::__kfree_skb(skb);
    }
    None
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
    // unsafe {
    //     bindings::free_percpu((*dev).stats);
    // }
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

const dummy_netdev_ops: bindings::net_device_ops = {
    ndo_init: dummy_dev_init,
    ndo_uninit: dummy_dev_uninit,
    ndo_start_xmit: dummy_xmit,
    ndo_validate_addr: bindings::eth_validate_addr,
    ndo_set_rx_mode: set_multicast_list,
    ndo_set_mac_address: bindings::eth_mac_addr,
    ndo_get_stats64: dummy_get_stats64,
    ndo_change_carrier: dummy_change_carrier,
};

// static const struct ethtool_ops dummy_ethtool_ops = {
// 	.get_ts_info		= ethtool_op_get_ts_info,
// };

const dummy_ethtoolops: bindings::ethtool_ops  = {
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
        (*dev).flags |= bindings::IFF_NOARP;
        (*dev).flags &= !bindings::IFF_MULTICAST;
        (*dev).priv_flags |= bindings::IFF_LIVE_ADDR_CHANGE | bindings::IFF_NO_QUEUE;
        (*dev).features |= bindings::NETIF_F_SG | bindings::NETIF_F_FRAGLIST;
        (*dev).features |= bindings::NETIF_F_GSO_SOFTWARE;
        (*dev).features |= bindings::NETIF_F_HW_CSUM | bindings::NETIF_F_HIGHDMA | bindings::NETIF_F_LLTX;
        (*dev).features |= bindings::NETIF_F_GSO_ENCAP_ALL;
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

fn dummy_validate(tb: *mut *mut bindings::nlattr, data: *mut *mut bindings::nlattr, extack: *mut bindings::netlink_ext_ack) -> i32 {
    unsafe {
        if !tb.is_null() && !(*tb.offset(bindings::IFLA_ADDRESS as isize)).is_null() {
            // TODO: nla_len doesn't exist
            if bindings::nla_len(*tb.offset(bindings::IFLA_ADDRESS as isize)) != bindings::ETH_ALEN {
                return -bindings::EINVAL;
            }
            // TODO: is_valid_ether_addr doesn't exist
            if bindings::is_valid_ether_addr(bindings::nla_data(*tb.offset(bindings::IFLA_ADDRESS as isize))) == 0 {
                return -bindings::EADDRNOTAVAIL;
            }
        }
    }
    0
}

// static struct rtnl_link_ops dummy_link_ops __read_mostly = {
// 	.kind		= DRV_NAME,
// 	.setup		= dummy_setup,
// 	.validate	= dummy_validate,
// };

// TODO: not sure about read_mostly in the original C
const dummy_link_ops: bindings::rtnl_link_ops = {
    kind: DRV_NAME,
    setup: dummy_setup,
    validate: dummy_validate,
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
        return -bindings::ENOMEM;
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

struct Dummy {

}

impl kernel::Module for Dummy {
    fn init(name: &'static Cstr, module: &'static ThisModule) -> Result<Self> {
        let mut err = 0;
        let numdummies: int = 1;
        unsafe {
            bindings::down_write(&mut bindings::pernet_ops_rwsem);
            bindings::rtnl_lock();
            err = bindings::__rtnl_link_register(&dummy_link_ops);
            if err < 0 {
                bindings::rtnl_unlock();
                bindings::up_write(&mut bindings::pernet_ops_rwsem);
                // TODO: not sure what to return here
            }

            for i in 0..numdummies && !err {
                err = dummy_init_one();
                bindings::cond_resched();
            }
            if err < 0 {
                bindings::__rtnl_link_unregister(&dummy_link_ops);
            }
        }
        Ok(E1000{})
    }
}

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
