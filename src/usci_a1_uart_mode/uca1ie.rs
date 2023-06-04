#[doc = "Register `UCA1IE` reader"]
pub struct R(crate::R<UCA1IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1IE` writer"]
pub struct W(crate::W<UCA1IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1IE_SPEC>;
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
impl From<crate::W<UCA1IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIE` reader - USCI Receive Interrupt Enable"]
pub type UCRXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIE` writer - USCI Receive Interrupt Enable"]
pub type UCRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IE_SPEC, bool, O>;
#[doc = "Field `UCTXIE` reader - USCI Transmit Interrupt Enable"]
pub type UCTXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIE` writer - USCI Transmit Interrupt Enable"]
pub type UCTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCA1IE_SPEC, bool, O>;
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A1 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ie](index.html) module"]
pub struct UCA1IE_SPEC;
impl crate::RegisterSpec for UCA1IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca1ie::R](R) reader structure"]
impl crate::Readable for UCA1IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1ie::W](W) writer structure"]
impl crate::Writable for UCA1IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1IE to value 0"]
impl crate::Resettable for UCA1IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
