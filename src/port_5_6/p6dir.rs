#[doc = "Register `P6DIR` reader"]
pub struct R(crate::R<P6DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P6DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P6DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P6DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P6DIR` writer"]
pub struct W(crate::W<P6DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P6DIR_SPEC>;
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
impl From<crate::W<P6DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P6DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P6DIR0` reader - P6DIR0"]
pub type P6DIR0_R = crate::BitReader<bool>;
#[doc = "Field `P6DIR0` writer - P6DIR0"]
pub type P6DIR0_W<'a, const O: u8> = crate::BitWriter<'a, u8, P6DIR_SPEC, bool, O>;
#[doc = "Field `P6DIR1` reader - P6DIR1"]
pub type P6DIR1_R = crate::BitReader<bool>;
#[doc = "Field `P6DIR1` writer - P6DIR1"]
pub type P6DIR1_W<'a, const O: u8> = crate::BitWriter<'a, u8, P6DIR_SPEC, bool, O>;
#[doc = "Field `P6DIR2` reader - P6DIR2"]
pub type P6DIR2_R = crate::BitReader<bool>;
#[doc = "Field `P6DIR2` writer - P6DIR2"]
pub type P6DIR2_W<'a, const O: u8> = crate::BitWriter<'a, u8, P6DIR_SPEC, bool, O>;
#[doc = "Field `P6DIR3` reader - P6DIR3"]
pub type P6DIR3_R = crate::BitReader<bool>;
#[doc = "Field `P6DIR3` writer - P6DIR3"]
pub type P6DIR3_W<'a, const O: u8> = crate::BitWriter<'a, u8, P6DIR_SPEC, bool, O>;
#[doc = "Field `P6DIR4` reader - P6DIR4"]
pub type P6DIR4_R = crate::BitReader<bool>;
#[doc = "Field `P6DIR4` writer - P6DIR4"]
pub type P6DIR4_W<'a, const O: u8> = crate::BitWriter<'a, u8, P6DIR_SPEC, bool, O>;
#[doc = "Field `P6DIR5` reader - P6DIR5"]
pub type P6DIR5_R = crate::BitReader<bool>;
#[doc = "Field `P6DIR5` writer - P6DIR5"]
pub type P6DIR5_W<'a, const O: u8> = crate::BitWriter<'a, u8, P6DIR_SPEC, bool, O>;
#[doc = "Field `P6DIR6` reader - P6DIR6"]
pub type P6DIR6_R = crate::BitReader<bool>;
#[doc = "Field `P6DIR6` writer - P6DIR6"]
pub type P6DIR6_W<'a, const O: u8> = crate::BitWriter<'a, u8, P6DIR_SPEC, bool, O>;
#[doc = "Field `P6DIR7` reader - P6DIR7"]
pub type P6DIR7_R = crate::BitReader<bool>;
#[doc = "Field `P6DIR7` writer - P6DIR7"]
pub type P6DIR7_W<'a, const O: u8> = crate::BitWriter<'a, u8, P6DIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - P6DIR0"]
    #[inline(always)]
    pub fn p6dir0(&self) -> P6DIR0_R {
        P6DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6DIR1"]
    #[inline(always)]
    pub fn p6dir1(&self) -> P6DIR1_R {
        P6DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6DIR2"]
    #[inline(always)]
    pub fn p6dir2(&self) -> P6DIR2_R {
        P6DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6DIR3"]
    #[inline(always)]
    pub fn p6dir3(&self) -> P6DIR3_R {
        P6DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6DIR4"]
    #[inline(always)]
    pub fn p6dir4(&self) -> P6DIR4_R {
        P6DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6DIR5"]
    #[inline(always)]
    pub fn p6dir5(&self) -> P6DIR5_R {
        P6DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6DIR6"]
    #[inline(always)]
    pub fn p6dir6(&self) -> P6DIR6_R {
        P6DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6DIR7"]
    #[inline(always)]
    pub fn p6dir7(&self) -> P6DIR7_R {
        P6DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6DIR0"]
    #[inline(always)]
    #[must_use]
    pub fn p6dir0(&mut self) -> P6DIR0_W<0> {
        P6DIR0_W::new(self)
    }
    #[doc = "Bit 1 - P6DIR1"]
    #[inline(always)]
    #[must_use]
    pub fn p6dir1(&mut self) -> P6DIR1_W<1> {
        P6DIR1_W::new(self)
    }
    #[doc = "Bit 2 - P6DIR2"]
    #[inline(always)]
    #[must_use]
    pub fn p6dir2(&mut self) -> P6DIR2_W<2> {
        P6DIR2_W::new(self)
    }
    #[doc = "Bit 3 - P6DIR3"]
    #[inline(always)]
    #[must_use]
    pub fn p6dir3(&mut self) -> P6DIR3_W<3> {
        P6DIR3_W::new(self)
    }
    #[doc = "Bit 4 - P6DIR4"]
    #[inline(always)]
    #[must_use]
    pub fn p6dir4(&mut self) -> P6DIR4_W<4> {
        P6DIR4_W::new(self)
    }
    #[doc = "Bit 5 - P6DIR5"]
    #[inline(always)]
    #[must_use]
    pub fn p6dir5(&mut self) -> P6DIR5_W<5> {
        P6DIR5_W::new(self)
    }
    #[doc = "Bit 6 - P6DIR6"]
    #[inline(always)]
    #[must_use]
    pub fn p6dir6(&mut self) -> P6DIR6_W<6> {
        P6DIR6_W::new(self)
    }
    #[doc = "Bit 7 - P6DIR7"]
    #[inline(always)]
    #[must_use]
    pub fn p6dir7(&mut self) -> P6DIR7_W<7> {
        P6DIR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Port 6 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6dir](index.html) module"]
pub struct P6DIR_SPEC;
impl crate::RegisterSpec for P6DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p6dir::R](R) reader structure"]
impl crate::Readable for P6DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p6dir::W](W) writer structure"]
impl crate::Writable for P6DIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P6DIR to value 0"]
impl crate::Resettable for P6DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
