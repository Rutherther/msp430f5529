#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port J Input"]
    pub pjin: PJIN,
    #[doc = "0x02 - Port J Output"]
    pub pjout: PJOUT,
    #[doc = "0x04 - Port J Direction"]
    pub pjdir: PJDIR,
    #[doc = "0x06 - Port J Resistor Enable"]
    pub pjren: PJREN,
    #[doc = "0x08 - Port J Drive Strenght"]
    pub pjds: PJDS,
}
#[doc = "PJIN (rw) register accessor: an alias for `Reg<PJIN_SPEC>`"]
pub type PJIN = crate::Reg<pjin::PJIN_SPEC>;
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "PJOUT (rw) register accessor: an alias for `Reg<PJOUT_SPEC>`"]
pub type PJOUT = crate::Reg<pjout::PJOUT_SPEC>;
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "PJDIR (rw) register accessor: an alias for `Reg<PJDIR_SPEC>`"]
pub type PJDIR = crate::Reg<pjdir::PJDIR_SPEC>;
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "PJREN (rw) register accessor: an alias for `Reg<PJREN_SPEC>`"]
pub type PJREN = crate::Reg<pjren::PJREN_SPEC>;
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "PJDS (rw) register accessor: an alias for `Reg<PJDS_SPEC>`"]
pub type PJDS = crate::Reg<pjds::PJDS_SPEC>;
#[doc = "Port J Drive Strenght"]
pub mod pjds;
