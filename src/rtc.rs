#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Real Timer Control 0/1"]
    pub rtcctl01: RTCCTL01,
    #[doc = "0x02 - Real Timer Control 2/3"]
    pub rtcctl23: RTCCTL23,
    _reserved2: [u8; 0x04],
    #[doc = "0x08 - Real Timer Prescale Timer 0 Control"]
    pub rtcps0ctl: RTCPS0CTL,
    #[doc = "0x0a - Real Timer Prescale Timer 1 Control"]
    pub rtcps1ctl: RTCPS1CTL,
    #[doc = "0x0c - Real Timer Prescale Timer Control"]
    pub rtcps: RTCPS,
    #[doc = "0x0e - Real Time Clock Interrupt Vector"]
    pub rtciv: RTCIV,
    #[doc = "0x10 - Real Time Clock Seconds"]
    pub rtcsec: RTCSEC,
    #[doc = "0x11 - Real Time Clock Minutes"]
    pub rtcmin: RTCMIN,
    #[doc = "0x12 - Real Time Clock Hour"]
    pub rtchour: RTCHOUR,
    #[doc = "0x13 - Real Time Clock Day of week"]
    pub rtcdow: RTCDOW,
    #[doc = "0x14 - Real Time Clock Day"]
    pub rtcday: RTCDAY,
    #[doc = "0x15 - Real Time Clock Month"]
    pub rtcmon: RTCMON,
    #[doc = "0x16 - Real Time Clock Year"]
    pub rtcyear: RTCYEAR,
    #[doc = "0x18 - Real Time Clock Alarm Min"]
    pub rtcamin: RTCAMIN,
    #[doc = "0x19 - Real Time Clock Alarm Hour"]
    pub rtcahour: RTCAHOUR,
    #[doc = "0x1a - Real Time Clock Alarm Day of week"]
    pub rtcadow: RTCADOW,
    #[doc = "0x1b - Real Time Clock Alarm Day"]
    pub rtcaday: RTCADAY,
}
#[doc = "RTCSEC (rw) register accessor: an alias for `Reg<RTCSEC_SPEC>`"]
pub type RTCSEC = crate::Reg<rtcsec::RTCSEC_SPEC>;
#[doc = "Real Time Clock Seconds"]
pub mod rtcsec;
#[doc = "RTCMIN (rw) register accessor: an alias for `Reg<RTCMIN_SPEC>`"]
pub type RTCMIN = crate::Reg<rtcmin::RTCMIN_SPEC>;
#[doc = "Real Time Clock Minutes"]
pub mod rtcmin;
#[doc = "RTCHOUR (rw) register accessor: an alias for `Reg<RTCHOUR_SPEC>`"]
pub type RTCHOUR = crate::Reg<rtchour::RTCHOUR_SPEC>;
#[doc = "Real Time Clock Hour"]
pub mod rtchour;
#[doc = "RTCDOW (rw) register accessor: an alias for `Reg<RTCDOW_SPEC>`"]
pub type RTCDOW = crate::Reg<rtcdow::RTCDOW_SPEC>;
#[doc = "Real Time Clock Day of week"]
pub mod rtcdow;
#[doc = "RTCDAY (rw) register accessor: an alias for `Reg<RTCDAY_SPEC>`"]
pub type RTCDAY = crate::Reg<rtcday::RTCDAY_SPEC>;
#[doc = "Real Time Clock Day"]
pub mod rtcday;
#[doc = "RTCMON (rw) register accessor: an alias for `Reg<RTCMON_SPEC>`"]
pub type RTCMON = crate::Reg<rtcmon::RTCMON_SPEC>;
#[doc = "Real Time Clock Month"]
pub mod rtcmon;
#[doc = "RTCAMIN (rw) register accessor: an alias for `Reg<RTCAMIN_SPEC>`"]
pub type RTCAMIN = crate::Reg<rtcamin::RTCAMIN_SPEC>;
#[doc = "Real Time Clock Alarm Min"]
pub mod rtcamin;
#[doc = "RTCAHOUR (rw) register accessor: an alias for `Reg<RTCAHOUR_SPEC>`"]
pub type RTCAHOUR = crate::Reg<rtcahour::RTCAHOUR_SPEC>;
#[doc = "Real Time Clock Alarm Hour"]
pub mod rtcahour;
#[doc = "RTCADOW (rw) register accessor: an alias for `Reg<RTCADOW_SPEC>`"]
pub type RTCADOW = crate::Reg<rtcadow::RTCADOW_SPEC>;
#[doc = "Real Time Clock Alarm Day of week"]
pub mod rtcadow;
#[doc = "RTCADAY (rw) register accessor: an alias for `Reg<RTCADAY_SPEC>`"]
pub type RTCADAY = crate::Reg<rtcaday::RTCADAY_SPEC>;
#[doc = "Real Time Clock Alarm Day"]
pub mod rtcaday;
#[doc = "RTCCTL01 (rw) register accessor: an alias for `Reg<RTCCTL01_SPEC>`"]
pub type RTCCTL01 = crate::Reg<rtcctl01::RTCCTL01_SPEC>;
#[doc = "Real Timer Control 0/1"]
pub mod rtcctl01;
#[doc = "RTCCTL23 (rw) register accessor: an alias for `Reg<RTCCTL23_SPEC>`"]
pub type RTCCTL23 = crate::Reg<rtcctl23::RTCCTL23_SPEC>;
#[doc = "Real Timer Control 2/3"]
pub mod rtcctl23;
#[doc = "RTCPS0CTL (rw) register accessor: an alias for `Reg<RTCPS0CTL_SPEC>`"]
pub type RTCPS0CTL = crate::Reg<rtcps0ctl::RTCPS0CTL_SPEC>;
#[doc = "Real Timer Prescale Timer 0 Control"]
pub mod rtcps0ctl;
#[doc = "RTCPS1CTL (rw) register accessor: an alias for `Reg<RTCPS1CTL_SPEC>`"]
pub type RTCPS1CTL = crate::Reg<rtcps1ctl::RTCPS1CTL_SPEC>;
#[doc = "Real Timer Prescale Timer 1 Control"]
pub mod rtcps1ctl;
#[doc = "RTCPS (rw) register accessor: an alias for `Reg<RTCPS_SPEC>`"]
pub type RTCPS = crate::Reg<rtcps::RTCPS_SPEC>;
#[doc = "Real Timer Prescale Timer Control"]
pub mod rtcps;
#[doc = "RTCIV (rw) register accessor: an alias for `Reg<RTCIV_SPEC>`"]
pub type RTCIV = crate::Reg<rtciv::RTCIV_SPEC>;
#[doc = "Real Time Clock Interrupt Vector"]
pub mod rtciv;
#[doc = "RTCYEAR (rw) register accessor: an alias for `Reg<RTCYEAR_SPEC>`"]
pub type RTCYEAR = crate::Reg<rtcyear::RTCYEAR_SPEC>;
#[doc = "Real Time Clock Year"]
pub mod rtcyear;
