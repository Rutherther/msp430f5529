#[doc = "Register `USBIFG` reader"]
pub struct R(crate::R<USBIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIFG` writer"]
pub struct W(crate::W<USBIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIFG_SPEC>;
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
impl From<crate::W<USBIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPOWIFG` reader - USB - Setup Overwrite Interrupt Flag"]
pub type STPOWIFG_R = crate::BitReader<bool>;
#[doc = "Field `STPOWIFG` writer - USB - Setup Overwrite Interrupt Flag"]
pub type STPOWIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIFG_SPEC, bool, O>;
#[doc = "Field `SETUPIFG` reader - USB - Setup Interrupt Flag"]
pub type SETUPIFG_R = crate::BitReader<bool>;
#[doc = "Field `SETUPIFG` writer - USB - Setup Interrupt Flag"]
pub type SETUPIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIFG_SPEC, bool, O>;
#[doc = "Field `RESRIFG` reader - USB - Function Resume Request Interrupt Flag"]
pub type RESRIFG_R = crate::BitReader<bool>;
#[doc = "Field `RESRIFG` writer - USB - Function Resume Request Interrupt Flag"]
pub type RESRIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIFG_SPEC, bool, O>;
#[doc = "Field `SUSRIFG` reader - USB - Function Suspend Request Interrupt Flag"]
pub type SUSRIFG_R = crate::BitReader<bool>;
#[doc = "Field `SUSRIFG` writer - USB - Function Suspend Request Interrupt Flag"]
pub type SUSRIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIFG_SPEC, bool, O>;
#[doc = "Field `RSTRIFG` reader - USB - Function Reset Request Interrupt Flag"]
pub type RSTRIFG_R = crate::BitReader<bool>;
#[doc = "Field `RSTRIFG` writer - USB - Function Reset Request Interrupt Flag"]
pub type RSTRIFG_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Flag"]
    #[inline(always)]
    pub fn stpowifg(&self) -> STPOWIFG_R {
        STPOWIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Flag"]
    #[inline(always)]
    pub fn setupifg(&self) -> SETUPIFG_R {
        SETUPIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Flag"]
    #[inline(always)]
    pub fn resrifg(&self) -> RESRIFG_R {
        RESRIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Flag"]
    #[inline(always)]
    pub fn susrifg(&self) -> SUSRIFG_R {
        SUSRIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Flag"]
    #[inline(always)]
    pub fn rstrifg(&self) -> RSTRIFG_R {
        RSTRIFG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stpowifg(&mut self) -> STPOWIFG_W<0> {
        STPOWIFG_W::new(self)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn setupifg(&mut self) -> SETUPIFG_W<2> {
        SETUPIFG_W::new(self)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn resrifg(&mut self) -> RESRIFG_W<5> {
        RESRIFG_W::new(self)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn susrifg(&mut self) -> SUSRIFG_W<6> {
        SUSRIFG_W::new(self)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstrifg(&mut self) -> RSTRIFG_W<7> {
        RSTRIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbifg](index.html) module"]
pub struct USBIFG_SPEC;
impl crate::RegisterSpec for USBIFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbifg::R](R) reader structure"]
impl crate::Readable for USBIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbifg::W](W) writer structure"]
impl crate::Writable for USBIFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIFG to value 0"]
impl crate::Resettable for USBIFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
