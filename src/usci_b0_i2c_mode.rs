#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    pub ucb0ctl1: UCB0CTL1,
    #[doc = "0x01 - USCI B0 Control Register 0"]
    pub ucb0ctl0: UCB0CTL0,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    pub ucb0br0: UCB0BR0,
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    pub ucb0br1: UCB0BR1,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - USCI B0 Status Register"]
    pub ucb0stat: UCB0STAT,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    pub ucb0rxbuf: UCB0RXBUF,
    _reserved6: [u8; 0x01],
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    pub ucb0txbuf: UCB0TXBUF,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - USCI B0 I2C Own Address"]
    pub ucb0i2coa: UCB0I2COA,
    #[doc = "0x12 - USCI B0 I2C Slave Address"]
    pub ucb0i2csa: UCB0I2CSA,
    _reserved9: [u8; 0x08],
    #[doc = "0x1c - USCI B0 Interrupt Enable Register"]
    pub ucb0ie: UCB0IE,
    #[doc = "0x1d - USCI B0 Interrupt Flags Register"]
    pub ucb0ifg: UCB0IFG,
    #[doc = "0x1e - USCI B0 Interrupt Vector Register"]
    pub ucb0iv: UCB0IV,
}
#[doc = "UCB0CTL1 (rw) register accessor: an alias for `Reg<UCB0CTL1_SPEC>`"]
pub type UCB0CTL1 = crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0CTL0 (rw) register accessor: an alias for `Reg<UCB0CTL0_SPEC>`"]
pub type UCB0CTL0 = crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0BR0 (rw) register accessor: an alias for `Reg<UCB0BR0_SPEC>`"]
pub type UCB0BR0 = crate::Reg<ucb0br0::UCB0BR0_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 (rw) register accessor: an alias for `Reg<UCB0BR1_SPEC>`"]
pub type UCB0BR1 = crate::Reg<ucb0br1::UCB0BR1_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0STAT (rw) register accessor: an alias for `Reg<UCB0STAT_SPEC>`"]
pub type UCB0STAT = crate::Reg<ucb0stat::UCB0STAT_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat;
#[doc = "UCB0RXBUF (rw) register accessor: an alias for `Reg<UCB0RXBUF_SPEC>`"]
pub type UCB0RXBUF = crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF (rw) register accessor: an alias for `Reg<UCB0TXBUF_SPEC>`"]
pub type UCB0TXBUF = crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "UCB0IE (rw) register accessor: an alias for `Reg<UCB0IE_SPEC>`"]
pub type UCB0IE = crate::Reg<ucb0ie::UCB0IE_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "UCB0IFG (rw) register accessor: an alias for `Reg<UCB0IFG_SPEC>`"]
pub type UCB0IFG = crate::Reg<ucb0ifg::UCB0IFG_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg;
#[doc = "UCB0I2COA (rw) register accessor: an alias for `Reg<UCB0I2COA_SPEC>`"]
pub type UCB0I2COA = crate::Reg<ucb0i2coa::UCB0I2COA_SPEC>;
#[doc = "USCI B0 I2C Own Address"]
pub mod ucb0i2coa;
#[doc = "UCB0I2CSA (rw) register accessor: an alias for `Reg<UCB0I2CSA_SPEC>`"]
pub type UCB0I2CSA = crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>;
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
#[doc = "UCB0IV (rw) register accessor: an alias for `Reg<UCB0IV_SPEC>`"]
pub type UCB0IV = crate::Reg<ucb0iv::UCB0IV_SPEC>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv;
