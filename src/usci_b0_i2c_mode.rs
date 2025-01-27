#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    pub ucb0ctl1: crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>,
    #[doc = "0x01 - USCI B0 Control Register 0"]
    pub ucb0ctl0: crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    pub ucb0br0: crate::Reg<ucb0br0::UCB0BR0_SPEC>,
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    pub ucb0br1: crate::Reg<ucb0br1::UCB0BR1_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - USCI B0 Status Register"]
    pub ucb0stat: crate::Reg<ucb0stat::UCB0STAT_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    pub ucb0rxbuf: crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    pub ucb0txbuf: crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - USCI B0 I2C Own Address"]
    pub ucb0i2coa: crate::Reg<ucb0i2coa::UCB0I2COA_SPEC>,
    #[doc = "0x12 - USCI B0 I2C Slave Address"]
    pub ucb0i2csa: crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x1c - USCI B0 Interrupt Enable Register"]
    pub ucb0ie: crate::Reg<ucb0ie::UCB0IE_SPEC>,
    #[doc = "0x1d - USCI B0 Interrupt Flags Register"]
    pub ucb0ifg: crate::Reg<ucb0ifg::UCB0IFG_SPEC>,
    #[doc = "0x1e - USCI B0 Interrupt Vector Register"]
    pub ucb0iv: crate::Reg<ucb0iv::UCB0IV_SPEC>,
}
#[doc = "UCB0CTL1 register accessor: an alias for `Reg<UCB0CTL1_SPEC>`"]
pub type UCB0CTL1 = crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0CTL0 register accessor: an alias for `Reg<UCB0CTL0_SPEC>`"]
pub type UCB0CTL0 = crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0BR0 register accessor: an alias for `Reg<UCB0BR0_SPEC>`"]
pub type UCB0BR0 = crate::Reg<ucb0br0::UCB0BR0_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 register accessor: an alias for `Reg<UCB0BR1_SPEC>`"]
pub type UCB0BR1 = crate::Reg<ucb0br1::UCB0BR1_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0STAT register accessor: an alias for `Reg<UCB0STAT_SPEC>`"]
pub type UCB0STAT = crate::Reg<ucb0stat::UCB0STAT_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat;
#[doc = "UCB0RXBUF register accessor: an alias for `Reg<UCB0RXBUF_SPEC>`"]
pub type UCB0RXBUF = crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF register accessor: an alias for `Reg<UCB0TXBUF_SPEC>`"]
pub type UCB0TXBUF = crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "UCB0IE register accessor: an alias for `Reg<UCB0IE_SPEC>`"]
pub type UCB0IE = crate::Reg<ucb0ie::UCB0IE_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "UCB0IFG register accessor: an alias for `Reg<UCB0IFG_SPEC>`"]
pub type UCB0IFG = crate::Reg<ucb0ifg::UCB0IFG_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg;
#[doc = "UCB0I2COA register accessor: an alias for `Reg<UCB0I2COA_SPEC>`"]
pub type UCB0I2COA = crate::Reg<ucb0i2coa::UCB0I2COA_SPEC>;
#[doc = "USCI B0 I2C Own Address"]
pub mod ucb0i2coa;
#[doc = "UCB0I2CSA register accessor: an alias for `Reg<UCB0I2CSA_SPEC>`"]
pub type UCB0I2CSA = crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>;
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
#[doc = "UCB0IV register accessor: an alias for `Reg<UCB0IV_SPEC>`"]
pub type UCB0IV = crate::Reg<ucb0iv::UCB0IV_SPEC>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv;
