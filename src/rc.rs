#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ram Controller Control Register"]
    pub rcctl0: RCCTL0,
}
#[doc = "RCCTL0 (rw) register accessor: an alias for `Reg<RCCTL0_SPEC>`"]
pub type RCCTL0 = crate::Reg<rcctl0::RCCTL0_SPEC>;
#[doc = "Ram Controller Control Register"]
pub mod rcctl0;
