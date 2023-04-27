use core::marker;
use kernel::bindings;

pub(crate) struct DeviceOperationsVtable<T>(marker::PhantomData<T>);

// A trait for the device operations of the dummy driver
// These functions are implemented in rust_dummy.rs
pub(crate) trait DeviceOperations {
    fn dummy_dev_init(dev: *mut bindings::net_device) -> i32;
    fn dummy_dev_uninit(dev: *mut bindings::net_device);
    fn dummy_xmit(skb: *mut bindings::sk_buff, dev: *mut bindings::net_device) -> i32;
    fn set_multicast_list(dev: *mut bindings::net_device);
    fn dummy_get_stats64(dev: *mut bindings::net_device, stats: *mut bindings::rtnl_link_stats64);
    fn dummy_change_carrier(dev: *mut bindings::net_device, new_carrier: bool) -> i32;
}

// Wrap the Rust functions above in external C functions because bindings::net_device_ops expects external "C" functions
impl<T: DeviceOperations> DeviceOperationsVtable<T> {
    unsafe extern "C" fn init_callback(dev: *mut bindings::net_device) -> core::ffi::c_int {
        T::dummy_dev_init(dev)
    }

    unsafe extern "C" fn uninit_callback(dev: *mut bindings::net_device) {
        T::dummy_dev_uninit(dev)
    }

    unsafe extern "C" fn xmit_callback(skb: *mut bindings::sk_buff, dev: *mut bindings::net_device) -> core::ffi::c_int {
        T::dummy_xmit(skb, dev)
    }

    unsafe extern "C" fn set_multicast_list_callback(dev: *mut bindings::net_device) {
        T::set_multicast_list(dev)
    }

    unsafe extern "C" fn get_stats64_callback(dev: *mut bindings::net_device, stats: *mut bindings::rtnl_link_stats64) {
        T::dummy_get_stats64(dev, stats)
    }

    unsafe extern "C" fn change_carrier_callback(dev: *mut bindings::net_device, new_carrier: bool) -> core::ffi::c_int {
        T::dummy_change_carrier(dev, new_carrier)
    }

    const VTABLE: bindings::net_device_ops = bindings::net_device_ops {
        ndo_init: Some(Self::init_callback),
        ndo_uninit: Some(Self::uninit_callback),
        ndo_open: None,
        ndo_stop: None,
        ndo_start_xmit: Some(Self::xmit_callback),
        ndo_features_check: None,
        ndo_select_queue: None,
        ndo_change_rx_flags: None,
        ndo_set_rx_mode: Some(Self::set_multicast_list),
        // TODO: ignored
        ndo_set_mac_address: None,
        // TODO: ignored
        ndo_validate_addr: None,
        ndo_do_ioctl: None,
        ndo_eth_ioctl: None,
        ndo_siocbond: None,
        ndo_siocwandev: None,
        ndo_siocdevprivate: None,
        ndo_set_config: None,
        ndo_change_mtu: None,
        ndo_neigh_setup: None,
        ndo_tx_timeout: None,
        ndo_get_stats64: Some(Self::get_stats64_callback),
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
        ndo_change_carrier: Some(Self::change_carrier_callback),
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

    pub(crate) const unsafe fn build() -> &'static bindings::net_device_ops {
        &Self::VTABLE
    }
}

pub(crate) struct EthToolOperationsVtable<T>(marker::PhantomData<T>);

// A trait for the ethtool operations of the dummy driver
pub(crate) trait DeviceOperations {
}


impl EthToolOperationsVtable {
    const VTABLE: bindings::ethtool_ops = bindings::ethtool_ops {
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
        get_ts_info: Some(bindings::ethtool_op_get_ts_info),
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

    pub(crate) const unsafe fn build() -> &'static bindings::ethtool_ops {
        &Self::VTABLE
    }
}
 