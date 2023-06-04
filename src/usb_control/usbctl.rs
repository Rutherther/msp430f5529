#[doc = "Register `USBCTL` reader"]
pub struct R(crate::R<USBCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCTL` writer"]
pub struct W(crate::W<USBCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCTL_SPEC>;
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
impl From<crate::W<USBCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - USB - Data Response Bit"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - USB - Data Response Bit"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBCTL_SPEC, bool, O>;
#[doc = "Field `FRSTE` reader - USB - Function Reset Connection Enable"]
pub type FRSTE_R = crate::BitReader<bool>;
#[doc = "Field `FRSTE` writer - USB - Function Reset Connection Enable"]
pub type FRSTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBCTL_SPEC, bool, O>;
#[doc = "Field `RWUP` reader - USB - Device Remote Wakeup Request"]
pub type RWUP_R = crate::BitReader<bool>;
#[doc = "Field `RWUP` writer - USB - Device Remote Wakeup Request"]
pub type RWUP_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBCTL_SPEC, bool, O>;
#[doc = "Field `FEN` reader - USB - Function Enable Bit"]
pub type FEN_R = crate::BitReader<bool>;
#[doc = "Field `FEN` writer - USB - Function Enable Bit"]
pub type FEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB - Data Response Bit"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - USB - Function Reset Connection Enable"]
    #[inline(always)]
    pub fn frste(&self) -> FRSTE_R {
        FRSTE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - Device Remote Wakeup Request"]
    #[inline(always)]
    pub fn rwup(&self) -> RWUP_R {
        RWUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB - Function Enable Bit"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Data Response Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Bit 4 - USB - Function Reset Connection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frste(&mut self) -> FRSTE_W<4> {
        FRSTE_W::new(self)
    }
    #[doc = "Bit 5 - USB - Device Remote Wakeup Request"]
    #[inline(always)]
    #[must_use]
    pub fn rwup(&mut self) -> RWUP_W<5> {
        RWUP_W::new(self)
    }
    #[doc = "Bit 6 - USB - Function Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FEN_W<6> {
        FEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbctl](index.html) module"]
pub struct USBCTL_SPEC;
impl crate::RegisterSpec for USBCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbctl::R](R) reader structure"]
impl crate::Readable for USBCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbctl::W](W) writer structure"]
impl crate::Writable for USBCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCTL to value 0"]
impl crate::Resettable for USBCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
