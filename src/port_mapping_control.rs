#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Mapping Key register"]
    pub pmapkeyid: PMAPKEYID,
    #[doc = "0x02 - Port Mapping control register"]
    pub pmapctl: PMAPCTL,
}
#[doc = "PMAPKEYID (rw) register accessor: an alias for `Reg<PMAPKEYID_SPEC>`"]
pub type PMAPKEYID = crate::Reg<pmapkeyid::PMAPKEYID_SPEC>;
#[doc = "Port Mapping Key register"]
pub mod pmapkeyid;
#[doc = "PMAPCTL (rw) register accessor: an alias for `Reg<PMAPCTL_SPEC>`"]
pub type PMAPCTL = crate::Reg<pmapctl::PMAPCTL_SPEC>;
#[doc = "Port Mapping control register"]
pub mod pmapctl;
