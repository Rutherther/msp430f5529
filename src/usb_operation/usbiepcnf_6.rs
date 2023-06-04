#[doc = "Register `USBIEPCNF_6` reader"]
pub struct R(crate::R<USBIEPCNF_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIEPCNF_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIEPCNF_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIEPCNF_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIEPCNF_6` writer"]
pub struct W(crate::W<USBIEPCNF_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIEPCNF_6_SPEC>;
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
impl From<crate::W<USBIEPCNF_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIEPCNF_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBIIE` reader - USB - Transaction Interrupt indication enable"]
pub type USBIIE_R = crate::BitReader<bool>;
#[doc = "Field `USBIIE` writer - USB - Transaction Interrupt indication enable"]
pub type USBIIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIEPCNF_6_SPEC, bool, O>;
#[doc = "Field `STALL` reader - USB - Stall Condition"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - USB - Stall Condition"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIEPCNF_6_SPEC, bool, O>;
#[doc = "Field `DBUF` reader - USB - Double Buffer Enable"]
pub type DBUF_R = crate::BitReader<bool>;
#[doc = "Field `DBUF` writer - USB - Double Buffer Enable"]
pub type DBUF_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIEPCNF_6_SPEC, bool, O>;
#[doc = "Field `TOGGLE` reader - USB - Toggle Bit"]
pub type TOGGLE_R = crate::BitReader<bool>;
#[doc = "Field `TOGGLE` writer - USB - Toggle Bit"]
pub type TOGGLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIEPCNF_6_SPEC, bool, O>;
#[doc = "Field `UBME` reader - USB - UBM In-Endpoint Enable"]
pub type UBME_R = crate::BitReader<bool>;
#[doc = "Field `UBME` writer - USB - UBM In-Endpoint Enable"]
pub type UBME_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIEPCNF_6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - USB - Transaction Interrupt indication enable"]
    #[inline(always)]
    pub fn usbiie(&self) -> USBIIE_R {
        USBIIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - Stall Condition"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB - Double Buffer Enable"]
    #[inline(always)]
    pub fn dbuf(&self) -> DBUF_R {
        DBUF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - Toggle Bit"]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB - UBM In-Endpoint Enable"]
    #[inline(always)]
    pub fn ubme(&self) -> UBME_R {
        UBME_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - USB - Transaction Interrupt indication enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbiie(&mut self) -> USBIIE_W<2> {
        USBIIE_W::new(self)
    }
    #[doc = "Bit 3 - USB - Stall Condition"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<3> {
        STALL_W::new(self)
    }
    #[doc = "Bit 4 - USB - Double Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbuf(&mut self) -> DBUF_W<4> {
        DBUF_W::new(self)
    }
    #[doc = "Bit 5 - USB - Toggle Bit"]
    #[inline(always)]
    #[must_use]
    pub fn toggle(&mut self) -> TOGGLE_W<5> {
        TOGGLE_W::new(self)
    }
    #[doc = "Bit 7 - USB - UBM In-Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ubme(&mut self) -> UBME_W<7> {
        UBME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Endpoint_6: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_6](index.html) module"]
pub struct USBIEPCNF_6_SPEC;
impl crate::RegisterSpec for USBIEPCNF_6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbiepcnf_6::R](R) reader structure"]
impl crate::Readable for USBIEPCNF_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_6::W](W) writer structure"]
impl crate::Writable for USBIEPCNF_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIEPCNF_6 to value 0"]
impl crate::Resettable for USBIEPCNF_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
