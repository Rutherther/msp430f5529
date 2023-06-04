#[doc = "Register `USBPLLDIVB` reader"]
pub struct R(crate::R<USBPLLDIVB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLDIVB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLDIVB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLDIVB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLDIVB` writer"]
pub struct W(crate::W<USBPLLDIVB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLDIVB_SPEC>;
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
impl From<crate::W<USBPLLDIVB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLDIVB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPMB0` reader - USB - PLL feedback divider buffer Bit 0"]
pub type UPMB0_R = crate::BitReader<bool>;
#[doc = "Field `UPMB0` writer - USB - PLL feedback divider buffer Bit 0"]
pub type UPMB0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
#[doc = "Field `UPMB1` reader - USB - PLL feedback divider buffer Bit 1"]
pub type UPMB1_R = crate::BitReader<bool>;
#[doc = "Field `UPMB1` writer - USB - PLL feedback divider buffer Bit 1"]
pub type UPMB1_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
#[doc = "Field `UPMB2` reader - USB - PLL feedback divider buffer Bit 2"]
pub type UPMB2_R = crate::BitReader<bool>;
#[doc = "Field `UPMB2` writer - USB - PLL feedback divider buffer Bit 2"]
pub type UPMB2_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
#[doc = "Field `UPMB3` reader - USB - PLL feedback divider buffer Bit 3"]
pub type UPMB3_R = crate::BitReader<bool>;
#[doc = "Field `UPMB3` writer - USB - PLL feedback divider buffer Bit 3"]
pub type UPMB3_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
#[doc = "Field `UPMB4` reader - USB - PLL feedback divider buffer Bit 4"]
pub type UPMB4_R = crate::BitReader<bool>;
#[doc = "Field `UPMB4` writer - USB - PLL feedback divider buffer Bit 4"]
pub type UPMB4_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
#[doc = "Field `UPMB5` reader - USB - PLL feedback divider buffer Bit 5"]
pub type UPMB5_R = crate::BitReader<bool>;
#[doc = "Field `UPMB5` writer - USB - PLL feedback divider buffer Bit 5"]
pub type UPMB5_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
#[doc = "Field `UPQB0` reader - USB - PLL prescale divider buffer Bit 0"]
pub type UPQB0_R = crate::BitReader<bool>;
#[doc = "Field `UPQB0` writer - USB - PLL prescale divider buffer Bit 0"]
pub type UPQB0_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
#[doc = "Field `UPQB1` reader - USB - PLL prescale divider buffer Bit 1"]
pub type UPQB1_R = crate::BitReader<bool>;
#[doc = "Field `UPQB1` writer - USB - PLL prescale divider buffer Bit 1"]
pub type UPQB1_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
#[doc = "Field `UPQB2` reader - USB - PLL prescale divider buffer Bit 2"]
pub type UPQB2_R = crate::BitReader<bool>;
#[doc = "Field `UPQB2` writer - USB - PLL prescale divider buffer Bit 2"]
pub type UPQB2_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLDIVB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB - PLL feedback divider buffer Bit 0"]
    #[inline(always)]
    pub fn upmb0(&self) -> UPMB0_R {
        UPMB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - PLL feedback divider buffer Bit 1"]
    #[inline(always)]
    pub fn upmb1(&self) -> UPMB1_R {
        UPMB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - PLL feedback divider buffer Bit 2"]
    #[inline(always)]
    pub fn upmb2(&self) -> UPMB2_R {
        UPMB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - PLL feedback divider buffer Bit 3"]
    #[inline(always)]
    pub fn upmb3(&self) -> UPMB3_R {
        UPMB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB - PLL feedback divider buffer Bit 4"]
    #[inline(always)]
    pub fn upmb4(&self) -> UPMB4_R {
        UPMB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - PLL feedback divider buffer Bit 5"]
    #[inline(always)]
    pub fn upmb5(&self) -> UPMB5_R {
        UPMB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - PLL prescale divider buffer Bit 0"]
    #[inline(always)]
    pub fn upqb0(&self) -> UPQB0_R {
        UPQB0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB - PLL prescale divider buffer Bit 1"]
    #[inline(always)]
    pub fn upqb1(&self) -> UPQB1_R {
        UPQB1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB - PLL prescale divider buffer Bit 2"]
    #[inline(always)]
    pub fn upqb2(&self) -> UPQB2_R {
        UPQB2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - PLL feedback divider buffer Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn upmb0(&mut self) -> UPMB0_W<0> {
        UPMB0_W::new(self)
    }
    #[doc = "Bit 1 - USB - PLL feedback divider buffer Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn upmb1(&mut self) -> UPMB1_W<1> {
        UPMB1_W::new(self)
    }
    #[doc = "Bit 2 - USB - PLL feedback divider buffer Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn upmb2(&mut self) -> UPMB2_W<2> {
        UPMB2_W::new(self)
    }
    #[doc = "Bit 3 - USB - PLL feedback divider buffer Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn upmb3(&mut self) -> UPMB3_W<3> {
        UPMB3_W::new(self)
    }
    #[doc = "Bit 4 - USB - PLL feedback divider buffer Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn upmb4(&mut self) -> UPMB4_W<4> {
        UPMB4_W::new(self)
    }
    #[doc = "Bit 5 - USB - PLL feedback divider buffer Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn upmb5(&mut self) -> UPMB5_W<5> {
        UPMB5_W::new(self)
    }
    #[doc = "Bit 8 - USB - PLL prescale divider buffer Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn upqb0(&mut self) -> UPQB0_W<8> {
        UPQB0_W::new(self)
    }
    #[doc = "Bit 9 - USB - PLL prescale divider buffer Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn upqb1(&mut self) -> UPQB1_W<9> {
        UPQB1_W::new(self)
    }
    #[doc = "Bit 10 - USB - PLL prescale divider buffer Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn upqb2(&mut self) -> UPQB2_W<10> {
        UPQB2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL Clock Divider Buffer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbplldivb](index.html) module"]
pub struct USBPLLDIVB_SPEC;
impl crate::RegisterSpec for USBPLLDIVB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbplldivb::R](R) reader structure"]
impl crate::Readable for USBPLLDIVB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbplldivb::W](W) writer structure"]
impl crate::Writable for USBPLLDIVB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPLLDIVB to value 0"]
impl crate::Resettable for USBPLLDIVB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
