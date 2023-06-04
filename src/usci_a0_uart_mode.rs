#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    pub uca0ctl1: UCA0CTL1,
    #[doc = "0x01 - USCI A0 Control Register 0"]
    pub uca0ctl0: UCA0CTL0,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    pub uca0br0: UCA0BR0,
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    pub uca0br1: UCA0BR1,
    #[doc = "0x08 - USCI A0 Modulation Control"]
    pub uca0mctl: UCA0MCTL,
    _reserved5: [u8; 0x01],
    #[doc = "0x0a - USCI A0 Status Register"]
    pub uca0stat: UCA0STAT,
    _reserved6: [u8; 0x01],
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    pub uca0rxbuf: UCA0RXBUF,
    _reserved7: [u8; 0x01],
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    pub uca0txbuf: UCA0TXBUF,
    _reserved8: [u8; 0x01],
    #[doc = "0x10 - USCI A0 LIN Control"]
    pub uca0abctl: UCA0ABCTL,
    _reserved9: [u8; 0x01],
    #[doc = "0x12 - USCI A0 IrDA Transmit Control"]
    pub uca0irtctl: UCA0IRTCTL,
    #[doc = "0x13 - USCI A0 IrDA Receive Control"]
    pub uca0irrctl: UCA0IRRCTL,
    _reserved11: [u8; 0x08],
    #[doc = "0x1c - USCI A0 Interrupt Enable Register"]
    pub uca0ie: UCA0IE,
    #[doc = "0x1d - USCI A0 Interrupt Flags Register"]
    pub uca0ifg: UCA0IFG,
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    pub uca0iv: UCA0IV,
}
#[doc = "UCA0CTL1 (rw) register accessor: an alias for `Reg<UCA0CTL1_SPEC>`"]
pub type UCA0CTL1 = crate::Reg<uca0ctl1::UCA0CTL1_SPEC>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
#[doc = "UCA0CTL0 (rw) register accessor: an alias for `Reg<UCA0CTL0_SPEC>`"]
pub type UCA0CTL0 = crate::Reg<uca0ctl0::UCA0CTL0_SPEC>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "UCA0BR0 (rw) register accessor: an alias for `Reg<UCA0BR0_SPEC>`"]
pub type UCA0BR0 = crate::Reg<uca0br0::UCA0BR0_SPEC>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0;
#[doc = "UCA0BR1 (rw) register accessor: an alias for `Reg<UCA0BR1_SPEC>`"]
pub type UCA0BR1 = crate::Reg<uca0br1::UCA0BR1_SPEC>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1;
#[doc = "UCA0MCTL (rw) register accessor: an alias for `Reg<UCA0MCTL_SPEC>`"]
pub type UCA0MCTL = crate::Reg<uca0mctl::UCA0MCTL_SPEC>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctl;
#[doc = "UCA0STAT (rw) register accessor: an alias for `Reg<UCA0STAT_SPEC>`"]
pub type UCA0STAT = crate::Reg<uca0stat::UCA0STAT_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat;
#[doc = "UCA0RXBUF (rw) register accessor: an alias for `Reg<UCA0RXBUF_SPEC>`"]
pub type UCA0RXBUF = crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf;
#[doc = "UCA0TXBUF (rw) register accessor: an alias for `Reg<UCA0TXBUF_SPEC>`"]
pub type UCA0TXBUF = crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf;
#[doc = "UCA0ABCTL (rw) register accessor: an alias for `Reg<UCA0ABCTL_SPEC>`"]
pub type UCA0ABCTL = crate::Reg<uca0abctl::UCA0ABCTL_SPEC>;
#[doc = "USCI A0 LIN Control"]
pub mod uca0abctl;
#[doc = "UCA0IRTCTL (rw) register accessor: an alias for `Reg<UCA0IRTCTL_SPEC>`"]
pub type UCA0IRTCTL = crate::Reg<uca0irtctl::UCA0IRTCTL_SPEC>;
#[doc = "USCI A0 IrDA Transmit Control"]
pub mod uca0irtctl;
#[doc = "UCA0IRRCTL (rw) register accessor: an alias for `Reg<UCA0IRRCTL_SPEC>`"]
pub type UCA0IRRCTL = crate::Reg<uca0irrctl::UCA0IRRCTL_SPEC>;
#[doc = "USCI A0 IrDA Receive Control"]
pub mod uca0irrctl;
#[doc = "UCA0IE (rw) register accessor: an alias for `Reg<UCA0IE_SPEC>`"]
pub type UCA0IE = crate::Reg<uca0ie::UCA0IE_SPEC>;
#[doc = "USCI A0 Interrupt Enable Register"]
pub mod uca0ie;
#[doc = "UCA0IFG (rw) register accessor: an alias for `Reg<UCA0IFG_SPEC>`"]
pub type UCA0IFG = crate::Reg<uca0ifg::UCA0IFG_SPEC>;
#[doc = "USCI A0 Interrupt Flags Register"]
pub mod uca0ifg;
#[doc = "UCA0IV (rw) register accessor: an alias for `Reg<UCA0IV_SPEC>`"]
pub type UCA0IV = crate::Reg<uca0iv::UCA0IV_SPEC>;
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv;
