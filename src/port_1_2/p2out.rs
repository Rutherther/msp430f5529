#[doc = "Register `P2OUT` reader"]
pub struct R(crate::R<P2OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2OUT` writer"]
pub struct W(crate::W<P2OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2OUT_SPEC>;
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
impl From<crate::W<P2OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2OUT0` reader - P2OUT0"]
pub type P2OUT0_R = crate::BitReader<bool>;
#[doc = "Field `P2OUT0` writer - P2OUT0"]
pub type P2OUT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2OUT_SPEC, bool, O>;
#[doc = "Field `P2OUT1` reader - P2OUT1"]
pub type P2OUT1_R = crate::BitReader<bool>;
#[doc = "Field `P2OUT1` writer - P2OUT1"]
pub type P2OUT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2OUT_SPEC, bool, O>;
#[doc = "Field `P2OUT2` reader - P2OUT2"]
pub type P2OUT2_R = crate::BitReader<bool>;
#[doc = "Field `P2OUT2` writer - P2OUT2"]
pub type P2OUT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2OUT_SPEC, bool, O>;
#[doc = "Field `P2OUT3` reader - P2OUT3"]
pub type P2OUT3_R = crate::BitReader<bool>;
#[doc = "Field `P2OUT3` writer - P2OUT3"]
pub type P2OUT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2OUT_SPEC, bool, O>;
#[doc = "Field `P2OUT4` reader - P2OUT4"]
pub type P2OUT4_R = crate::BitReader<bool>;
#[doc = "Field `P2OUT4` writer - P2OUT4"]
pub type P2OUT4_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2OUT_SPEC, bool, O>;
#[doc = "Field `P2OUT5` reader - P2OUT5"]
pub type P2OUT5_R = crate::BitReader<bool>;
#[doc = "Field `P2OUT5` writer - P2OUT5"]
pub type P2OUT5_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2OUT_SPEC, bool, O>;
#[doc = "Field `P2OUT6` reader - P2OUT6"]
pub type P2OUT6_R = crate::BitReader<bool>;
#[doc = "Field `P2OUT6` writer - P2OUT6"]
pub type P2OUT6_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2OUT_SPEC, bool, O>;
#[doc = "Field `P2OUT7` reader - P2OUT7"]
pub type P2OUT7_R = crate::BitReader<bool>;
#[doc = "Field `P2OUT7` writer - P2OUT7"]
pub type P2OUT7_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2OUT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - P2OUT0"]
    #[inline(always)]
    pub fn p2out0(&self) -> P2OUT0_R {
        P2OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2OUT1"]
    #[inline(always)]
    pub fn p2out1(&self) -> P2OUT1_R {
        P2OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2OUT2"]
    #[inline(always)]
    pub fn p2out2(&self) -> P2OUT2_R {
        P2OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2OUT3"]
    #[inline(always)]
    pub fn p2out3(&self) -> P2OUT3_R {
        P2OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2OUT4"]
    #[inline(always)]
    pub fn p2out4(&self) -> P2OUT4_R {
        P2OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2OUT5"]
    #[inline(always)]
    pub fn p2out5(&self) -> P2OUT5_R {
        P2OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2OUT6"]
    #[inline(always)]
    pub fn p2out6(&self) -> P2OUT6_R {
        P2OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2OUT7"]
    #[inline(always)]
    pub fn p2out7(&self) -> P2OUT7_R {
        P2OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn p2out0(&mut self) -> P2OUT0_W<0> {
        P2OUT0_W::new(self)
    }
    #[doc = "Bit 1 - P2OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn p2out1(&mut self) -> P2OUT1_W<1> {
        P2OUT1_W::new(self)
    }
    #[doc = "Bit 2 - P2OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn p2out2(&mut self) -> P2OUT2_W<2> {
        P2OUT2_W::new(self)
    }
    #[doc = "Bit 3 - P2OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn p2out3(&mut self) -> P2OUT3_W<3> {
        P2OUT3_W::new(self)
    }
    #[doc = "Bit 4 - P2OUT4"]
    #[inline(always)]
    #[must_use]
    pub fn p2out4(&mut self) -> P2OUT4_W<4> {
        P2OUT4_W::new(self)
    }
    #[doc = "Bit 5 - P2OUT5"]
    #[inline(always)]
    #[must_use]
    pub fn p2out5(&mut self) -> P2OUT5_W<5> {
        P2OUT5_W::new(self)
    }
    #[doc = "Bit 6 - P2OUT6"]
    #[inline(always)]
    #[must_use]
    pub fn p2out6(&mut self) -> P2OUT6_W<6> {
        P2OUT6_W::new(self)
    }
    #[doc = "Bit 7 - P2OUT7"]
    #[inline(always)]
    #[must_use]
    pub fn p2out7(&mut self) -> P2OUT7_W<7> {
        P2OUT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Port 2 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2out](index.html) module"]
pub struct P2OUT_SPEC;
impl crate::RegisterSpec for P2OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2out::R](R) reader structure"]
impl crate::Readable for P2OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2out::W](W) writer structure"]
impl crate::Writable for P2OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P2OUT to value 0"]
impl crate::Resettable for P2OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
