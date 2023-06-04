#[doc = "Register `UCB1IFG_SPI` reader"]
pub struct R(crate::R<UCB1IFG_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1IFG_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB1IFG_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB1IFG_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1IFG_SPI` writer"]
pub struct W(crate::W<UCB1IFG_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1IFG_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UCB1IFG_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB1IFG_SPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG` reader - USCI Receive Interrupt Flag"]
pub type UCRXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIFG` writer - USCI Receive Interrupt Flag"]
pub type UCRXIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB1IFG_SPI_SPEC, bool, O>;
#[doc = "Field `UCTXIFG` reader - USCI Transmit Interrupt Flag"]
pub type UCTXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIFG` writer - USCI Transmit Interrupt Flag"]
pub type UCTXIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB1IFG_SPI_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W<0> {
        UCRXIFG_W::new(self)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uctxifg(&mut self) -> UCTXIFG_W<1> {
        UCTXIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg_spi](index.html) module"]
pub struct UCB1IFG_SPI_SPEC;
impl crate::RegisterSpec for UCB1IFG_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb1ifg_spi::R](R) reader structure"]
impl crate::Readable for UCB1IFG_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1ifg_spi::W](W) writer structure"]
impl crate::Writable for UCB1IFG_SPI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB1IFG_SPI to value 0"]
impl crate::Resettable for UCB1IFG_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
