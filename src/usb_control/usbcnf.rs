#[doc = "Register `USBCNF` reader"]
pub struct R(crate::R<USBCNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCNF` writer"]
pub struct W(crate::W<USBCNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCNF_SPEC>;
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
impl From<crate::W<USBCNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EN` reader - USB - Module enable"]
pub type USB_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_EN` writer - USB - Module enable"]
pub type USB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBCNF_SPEC, bool, O>;
#[doc = "Field `PUR_EN` reader - USB - PUR pin enable"]
pub type PUR_EN_R = crate::BitReader<bool>;
#[doc = "Field `PUR_EN` writer - USB - PUR pin enable"]
pub type PUR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBCNF_SPEC, bool, O>;
#[doc = "Field `PUR_IN` reader - USB - PUR pin input value"]
pub type PUR_IN_R = crate::BitReader<bool>;
#[doc = "Field `PUR_IN` writer - USB - PUR pin input value"]
pub type PUR_IN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBCNF_SPEC, bool, O>;
#[doc = "Field `BLKRDY` reader - USB - Block ready signal for DMA"]
pub type BLKRDY_R = crate::BitReader<bool>;
#[doc = "Field `BLKRDY` writer - USB - Block ready signal for DMA"]
pub type BLKRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBCNF_SPEC, bool, O>;
#[doc = "Field `FNTEN` reader - USB - Frame Number receive Trigger enable for DMA"]
pub type FNTEN_R = crate::BitReader<bool>;
#[doc = "Field `FNTEN` writer - USB - Frame Number receive Trigger enable for DMA"]
pub type FNTEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBCNF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB - Module enable"]
    #[inline(always)]
    pub fn usb_en(&self) -> USB_EN_R {
        USB_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - PUR pin enable"]
    #[inline(always)]
    pub fn pur_en(&self) -> PUR_EN_R {
        PUR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - PUR pin input value"]
    #[inline(always)]
    pub fn pur_in(&self) -> PUR_IN_R {
        PUR_IN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - Block ready signal for DMA"]
    #[inline(always)]
    pub fn blkrdy(&self) -> BLKRDY_R {
        BLKRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB - Frame Number receive Trigger enable for DMA"]
    #[inline(always)]
    pub fn fnten(&self) -> FNTEN_R {
        FNTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Module enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_en(&mut self) -> USB_EN_W<0> {
        USB_EN_W::new(self)
    }
    #[doc = "Bit 1 - USB - PUR pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn pur_en(&mut self) -> PUR_EN_W<1> {
        PUR_EN_W::new(self)
    }
    #[doc = "Bit 2 - USB - PUR pin input value"]
    #[inline(always)]
    #[must_use]
    pub fn pur_in(&mut self) -> PUR_IN_W<2> {
        PUR_IN_W::new(self)
    }
    #[doc = "Bit 3 - USB - Block ready signal for DMA"]
    #[inline(always)]
    #[must_use]
    pub fn blkrdy(&mut self) -> BLKRDY_W<3> {
        BLKRDY_W::new(self)
    }
    #[doc = "Bit 4 - USB - Frame Number receive Trigger enable for DMA"]
    #[inline(always)]
    #[must_use]
    pub fn fnten(&mut self) -> FNTEN_W<4> {
        FNTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Module configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcnf](index.html) module"]
pub struct USBCNF_SPEC;
impl crate::RegisterSpec for USBCNF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbcnf::R](R) reader structure"]
impl crate::Readable for USBCNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcnf::W](W) writer structure"]
impl crate::Writable for USBCNF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCNF to value 0"]
impl crate::Resettable for USBCNF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
