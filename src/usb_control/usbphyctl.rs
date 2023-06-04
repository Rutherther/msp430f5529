#[doc = "Register `USBPHYCTL` reader"]
pub struct R(crate::R<USBPHYCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYCTL` writer"]
pub struct W(crate::W<USBPHYCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYCTL_SPEC>;
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
impl From<crate::W<USBPHYCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUOUT0` reader - USB - USB Port Output Signal Bit 0"]
pub type PUOUT0_R = crate::BitReader<bool>;
#[doc = "Field `PUOUT0` writer - USB - USB Port Output Signal Bit 0"]
pub type PUOUT0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPHYCTL_SPEC, bool, O>;
#[doc = "Field `PUOUT1` reader - USB - USB Port Output Signal Bit 1"]
pub type PUOUT1_R = crate::BitReader<bool>;
#[doc = "Field `PUOUT1` writer - USB - USB Port Output Signal Bit 1"]
pub type PUOUT1_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPHYCTL_SPEC, bool, O>;
#[doc = "Field `PUIN0` reader - USB - PU0/DP Input Data"]
pub type PUIN0_R = crate::BitReader<bool>;
#[doc = "Field `PUIN0` writer - USB - PU0/DP Input Data"]
pub type PUIN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPHYCTL_SPEC, bool, O>;
#[doc = "Field `PUIN1` reader - USB - PU1/DM Input Data"]
pub type PUIN1_R = crate::BitReader<bool>;
#[doc = "Field `PUIN1` writer - USB - PU1/DM Input Data"]
pub type PUIN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPHYCTL_SPEC, bool, O>;
#[doc = "Field `PUOPE` reader - USB - USB Port Output Enable"]
pub type PUOPE_R = crate::BitReader<bool>;
#[doc = "Field `PUOPE` writer - USB - USB Port Output Enable"]
pub type PUOPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPHYCTL_SPEC, bool, O>;
#[doc = "Field `PUSEL` reader - USB - USB Port Function Select"]
pub type PUSEL_R = crate::BitReader<bool>;
#[doc = "Field `PUSEL` writer - USB - USB Port Function Select"]
pub type PUSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPHYCTL_SPEC, bool, O>;
#[doc = "Field `PUIPE` reader - USB - PHY Single Ended Input enable"]
pub type PUIPE_R = crate::BitReader<bool>;
#[doc = "Field `PUIPE` writer - USB - PHY Single Ended Input enable"]
pub type PUIPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPHYCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB - USB Port Output Signal Bit 0"]
    #[inline(always)]
    pub fn puout0(&self) -> PUOUT0_R {
        PUOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - USB Port Output Signal Bit 1"]
    #[inline(always)]
    pub fn puout1(&self) -> PUOUT1_R {
        PUOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - PU0/DP Input Data"]
    #[inline(always)]
    pub fn puin0(&self) -> PUIN0_R {
        PUIN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - PU1/DM Input Data"]
    #[inline(always)]
    pub fn puin1(&self) -> PUIN1_R {
        PUIN1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - USB Port Output Enable"]
    #[inline(always)]
    pub fn puope(&self) -> PUOPE_R {
        PUOPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB - USB Port Function Select"]
    #[inline(always)]
    pub fn pusel(&self) -> PUSEL_R {
        PUSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - PHY Single Ended Input enable"]
    #[inline(always)]
    pub fn puipe(&self) -> PUIPE_R {
        PUIPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - USB Port Output Signal Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn puout0(&mut self) -> PUOUT0_W<0> {
        PUOUT0_W::new(self)
    }
    #[doc = "Bit 1 - USB - USB Port Output Signal Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn puout1(&mut self) -> PUOUT1_W<1> {
        PUOUT1_W::new(self)
    }
    #[doc = "Bit 2 - USB - PU0/DP Input Data"]
    #[inline(always)]
    #[must_use]
    pub fn puin0(&mut self) -> PUIN0_W<2> {
        PUIN0_W::new(self)
    }
    #[doc = "Bit 3 - USB - PU1/DM Input Data"]
    #[inline(always)]
    #[must_use]
    pub fn puin1(&mut self) -> PUIN1_W<3> {
        PUIN1_W::new(self)
    }
    #[doc = "Bit 5 - USB - USB Port Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn puope(&mut self) -> PUOPE_W<5> {
        PUOPE_W::new(self)
    }
    #[doc = "Bit 7 - USB - USB Port Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn pusel(&mut self) -> PUSEL_W<7> {
        PUSEL_W::new(self)
    }
    #[doc = "Bit 8 - USB - PHY Single Ended Input enable"]
    #[inline(always)]
    #[must_use]
    pub fn puipe(&mut self) -> PUIPE_W<8> {
        PUIPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyctl](index.html) module"]
pub struct USBPHYCTL_SPEC;
impl crate::RegisterSpec for USBPHYCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbphyctl::R](R) reader structure"]
impl crate::Readable for USBPHYCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphyctl::W](W) writer structure"]
impl crate::Writable for USBPHYCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPHYCTL to value 0"]
impl crate::Resettable for USBPHYCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
