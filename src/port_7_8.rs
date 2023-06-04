#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 7 Input"]
    pub p7in: P7IN,
    #[doc = "0x01 - Port 8 Input"]
    pub p8in: P8IN,
    #[doc = "0x02 - Port 7 Output"]
    pub p7out: P7OUT,
    #[doc = "0x03 - Port 8 Output"]
    pub p8out: P8OUT,
    #[doc = "0x04 - Port 7 Direction"]
    pub p7dir: P7DIR,
    #[doc = "0x05 - Port 8 Direction"]
    pub p8dir: P8DIR,
    #[doc = "0x06 - Port 7 Resistor Enable"]
    pub p7ren: P7REN,
    #[doc = "0x07 - Port 8 Resistor Enable"]
    pub p8ren: P8REN,
    #[doc = "0x08 - Port 7 Drive Strenght"]
    pub p7ds: P7DS,
    #[doc = "0x09 - Port 8 Drive Strenght"]
    pub p8ds: P8DS,
    #[doc = "0x0a - Port 7 Selection"]
    pub p7sel: P7SEL,
    #[doc = "0x0b - Port 8 Selection"]
    pub p8sel: P8SEL,
}
#[doc = "P7IN (rw) register accessor: an alias for `Reg<P7IN_SPEC>`"]
pub type P7IN = crate::Reg<p7in::P7IN_SPEC>;
#[doc = "Port 7 Input"]
pub mod p7in;
#[doc = "P8IN (rw) register accessor: an alias for `Reg<P8IN_SPEC>`"]
pub type P8IN = crate::Reg<p8in::P8IN_SPEC>;
#[doc = "Port 8 Input"]
pub mod p8in;
#[doc = "P7OUT (rw) register accessor: an alias for `Reg<P7OUT_SPEC>`"]
pub type P7OUT = crate::Reg<p7out::P7OUT_SPEC>;
#[doc = "Port 7 Output"]
pub mod p7out;
#[doc = "P8OUT (rw) register accessor: an alias for `Reg<P8OUT_SPEC>`"]
pub type P8OUT = crate::Reg<p8out::P8OUT_SPEC>;
#[doc = "Port 8 Output"]
pub mod p8out;
#[doc = "P7DIR (rw) register accessor: an alias for `Reg<P7DIR_SPEC>`"]
pub type P7DIR = crate::Reg<p7dir::P7DIR_SPEC>;
#[doc = "Port 7 Direction"]
pub mod p7dir;
#[doc = "P8DIR (rw) register accessor: an alias for `Reg<P8DIR_SPEC>`"]
pub type P8DIR = crate::Reg<p8dir::P8DIR_SPEC>;
#[doc = "Port 8 Direction"]
pub mod p8dir;
#[doc = "P7REN (rw) register accessor: an alias for `Reg<P7REN_SPEC>`"]
pub type P7REN = crate::Reg<p7ren::P7REN_SPEC>;
#[doc = "Port 7 Resistor Enable"]
pub mod p7ren;
#[doc = "P8REN (rw) register accessor: an alias for `Reg<P8REN_SPEC>`"]
pub type P8REN = crate::Reg<p8ren::P8REN_SPEC>;
#[doc = "Port 8 Resistor Enable"]
pub mod p8ren;
#[doc = "P7DS (rw) register accessor: an alias for `Reg<P7DS_SPEC>`"]
pub type P7DS = crate::Reg<p7ds::P7DS_SPEC>;
#[doc = "Port 7 Drive Strenght"]
pub mod p7ds;
#[doc = "P8DS (rw) register accessor: an alias for `Reg<P8DS_SPEC>`"]
pub type P8DS = crate::Reg<p8ds::P8DS_SPEC>;
#[doc = "Port 8 Drive Strenght"]
pub mod p8ds;
#[doc = "P7SEL (rw) register accessor: an alias for `Reg<P7SEL_SPEC>`"]
pub type P7SEL = crate::Reg<p7sel::P7SEL_SPEC>;
#[doc = "Port 7 Selection"]
pub mod p7sel;
#[doc = "P8SEL (rw) register accessor: an alias for `Reg<P8SEL_SPEC>`"]
pub type P8SEL = crate::Reg<p8sel::P8SEL_SPEC>;
#[doc = "Port 8 Selection"]
pub mod p8sel;
