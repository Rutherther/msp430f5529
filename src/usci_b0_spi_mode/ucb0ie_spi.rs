#[doc = "Register `UCB0IE_SPI` reader"]
pub struct R(crate::R<UCB0IE_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0IE_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0IE_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0IE_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0IE_SPI` writer"]
pub struct W(crate::W<UCB0IE_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0IE_SPI_SPEC>;
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
impl From<crate::W<UCB0IE_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0IE_SPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIE` reader - USCI Receive Interrupt Enable"]
pub type UCRXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIE` writer - USCI Receive Interrupt Enable"]
pub type UCRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB0IE_SPI_SPEC, bool, O>;
#[doc = "Field `UCTXIE` reader - USCI Transmit Interrupt Enable"]
pub type UCTXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIE` writer - USCI Transmit Interrupt Enable"]
pub type UCTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB0IE_SPI_SPEC, bool, O>;
#[doc = "Field `UCSTTIE` reader - START Condition interrupt enable"]
pub type UCSTTIE_R = crate::BitReader<bool>;
#[doc = "Field `UCSTTIE` writer - START Condition interrupt enable"]
pub type UCSTTIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB0IE_SPI_SPEC, bool, O>;
#[doc = "Field `UCSTPIE` reader - STOP Condition interrupt enable"]
pub type UCSTPIE_R = crate::BitReader<bool>;
#[doc = "Field `UCSTPIE` writer - STOP Condition interrupt enable"]
pub type UCSTPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB0IE_SPI_SPEC, bool, O>;
#[doc = "Field `UCALIE` reader - Arbitration Lost interrupt enable"]
pub type UCALIE_R = crate::BitReader<bool>;
#[doc = "Field `UCALIE` writer - Arbitration Lost interrupt enable"]
pub type UCALIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB0IE_SPI_SPEC, bool, O>;
#[doc = "Field `UCNACKIE` reader - NACK Condition interrupt enable"]
pub type UCNACKIE_R = crate::BitReader<bool>;
#[doc = "Field `UCNACKIE` writer - NACK Condition interrupt enable"]
pub type UCNACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB0IE_SPI_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie(&mut self) -> UCRXIE_W<0> {
        UCRXIE_W::new(self)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie(&mut self) -> UCTXIE_W<1> {
        UCTXIE_W::new(self)
    }
    #[doc = "Bit 2 - START Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucsttie(&mut self) -> UCSTTIE_W<2> {
        UCSTTIE_W::new(self)
    }
    #[doc = "Bit 3 - STOP Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucstpie(&mut self) -> UCSTPIE_W<3> {
        UCSTPIE_W::new(self)
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucalie(&mut self) -> UCALIE_W<4> {
        UCALIE_W::new(self)
    }
    #[doc = "Bit 5 - NACK Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucnackie(&mut self) -> UCNACKIE_W<5> {
        UCNACKIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ie_spi](index.html) module"]
pub struct UCB0IE_SPI_SPEC;
impl crate::RegisterSpec for UCB0IE_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0ie_spi::R](R) reader structure"]
impl crate::Readable for UCB0IE_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ie_spi::W](W) writer structure"]
impl crate::Writable for UCB0IE_SPI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB0IE_SPI to value 0"]
impl crate::Resettable for UCB0IE_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
