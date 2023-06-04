#[doc = "Register `PJREN` reader"]
pub struct R(crate::R<PJREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJREN` writer"]
pub struct W(crate::W<PJREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJREN_SPEC>;
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
impl From<crate::W<PJREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJREN0` reader - PJREN0"]
pub type PJREN0_R = crate::BitReader<bool>;
#[doc = "Field `PJREN0` writer - PJREN0"]
pub type PJREN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, PJREN_SPEC, bool, O>;
#[doc = "Field `PJREN1` reader - PJREN1"]
pub type PJREN1_R = crate::BitReader<bool>;
#[doc = "Field `PJREN1` writer - PJREN1"]
pub type PJREN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, PJREN_SPEC, bool, O>;
#[doc = "Field `PJREN2` reader - PJREN2"]
pub type PJREN2_R = crate::BitReader<bool>;
#[doc = "Field `PJREN2` writer - PJREN2"]
pub type PJREN2_W<'a, const O: u8> = crate::BitWriter<'a, u16, PJREN_SPEC, bool, O>;
#[doc = "Field `PJREN3` reader - PJREN3"]
pub type PJREN3_R = crate::BitReader<bool>;
#[doc = "Field `PJREN3` writer - PJREN3"]
pub type PJREN3_W<'a, const O: u8> = crate::BitWriter<'a, u16, PJREN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PJREN0"]
    #[inline(always)]
    pub fn pjren0(&self) -> PJREN0_R {
        PJREN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PJREN1"]
    #[inline(always)]
    pub fn pjren1(&self) -> PJREN1_R {
        PJREN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PJREN2"]
    #[inline(always)]
    pub fn pjren2(&self) -> PJREN2_R {
        PJREN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PJREN3"]
    #[inline(always)]
    pub fn pjren3(&self) -> PJREN3_R {
        PJREN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJREN0"]
    #[inline(always)]
    #[must_use]
    pub fn pjren0(&mut self) -> PJREN0_W<0> {
        PJREN0_W::new(self)
    }
    #[doc = "Bit 1 - PJREN1"]
    #[inline(always)]
    #[must_use]
    pub fn pjren1(&mut self) -> PJREN1_W<1> {
        PJREN1_W::new(self)
    }
    #[doc = "Bit 2 - PJREN2"]
    #[inline(always)]
    #[must_use]
    pub fn pjren2(&mut self) -> PJREN2_W<2> {
        PJREN2_W::new(self)
    }
    #[doc = "Bit 3 - PJREN3"]
    #[inline(always)]
    #[must_use]
    pub fn pjren3(&mut self) -> PJREN3_W<3> {
        PJREN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjren](index.html) module"]
pub struct PJREN_SPEC;
impl crate::RegisterSpec for PJREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjren::R](R) reader structure"]
impl crate::Readable for PJREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjren::W](W) writer structure"]
impl crate::Writable for PJREN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PJREN to value 0"]
impl crate::Resettable for PJREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
