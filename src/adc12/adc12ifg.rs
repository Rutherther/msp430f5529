#[doc = "Register `ADC12IFG` reader"]
pub struct R(crate::R<ADC12IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IFG` writer"]
pub struct W(crate::W<ADC12IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IFG_SPEC>;
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
impl From<crate::W<ADC12IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12IFG0` reader - ADC12 Memory 0 Interrupt Flag"]
pub type ADC12IFG0_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG0` writer - ADC12 Memory 0 Interrupt Flag"]
pub type ADC12IFG0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG1` reader - ADC12 Memory 1 Interrupt Flag"]
pub type ADC12IFG1_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG1` writer - ADC12 Memory 1 Interrupt Flag"]
pub type ADC12IFG1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG2` reader - ADC12 Memory 2 Interrupt Flag"]
pub type ADC12IFG2_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG2` writer - ADC12 Memory 2 Interrupt Flag"]
pub type ADC12IFG2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG3` reader - ADC12 Memory 3 Interrupt Flag"]
pub type ADC12IFG3_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG3` writer - ADC12 Memory 3 Interrupt Flag"]
pub type ADC12IFG3_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG4` reader - ADC12 Memory 4 Interrupt Flag"]
pub type ADC12IFG4_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG4` writer - ADC12 Memory 4 Interrupt Flag"]
pub type ADC12IFG4_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG5` reader - ADC12 Memory 5 Interrupt Flag"]
pub type ADC12IFG5_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG5` writer - ADC12 Memory 5 Interrupt Flag"]
pub type ADC12IFG5_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG6` reader - ADC12 Memory 6 Interrupt Flag"]
pub type ADC12IFG6_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG6` writer - ADC12 Memory 6 Interrupt Flag"]
pub type ADC12IFG6_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG7` reader - ADC12 Memory 7 Interrupt Flag"]
pub type ADC12IFG7_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG7` writer - ADC12 Memory 7 Interrupt Flag"]
pub type ADC12IFG7_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG8` reader - ADC12 Memory 8 Interrupt Flag"]
pub type ADC12IFG8_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG8` writer - ADC12 Memory 8 Interrupt Flag"]
pub type ADC12IFG8_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG9` reader - ADC12 Memory 9 Interrupt Flag"]
pub type ADC12IFG9_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG9` writer - ADC12 Memory 9 Interrupt Flag"]
pub type ADC12IFG9_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG10` reader - ADC12 Memory 10 Interrupt Flag"]
pub type ADC12IFG10_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG10` writer - ADC12 Memory 10 Interrupt Flag"]
pub type ADC12IFG10_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG11` reader - ADC12 Memory 11 Interrupt Flag"]
pub type ADC12IFG11_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG11` writer - ADC12 Memory 11 Interrupt Flag"]
pub type ADC12IFG11_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG12` reader - ADC12 Memory 12 Interrupt Flag"]
pub type ADC12IFG12_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG12` writer - ADC12 Memory 12 Interrupt Flag"]
pub type ADC12IFG12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG13` reader - ADC12 Memory 13 Interrupt Flag"]
pub type ADC12IFG13_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG13` writer - ADC12 Memory 13 Interrupt Flag"]
pub type ADC12IFG13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG14` reader - ADC12 Memory 14 Interrupt Flag"]
pub type ADC12IFG14_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG14` writer - ADC12 Memory 14 Interrupt Flag"]
pub type ADC12IFG14_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
#[doc = "Field `ADC12IFG15` reader - ADC12 Memory 15 Interrupt Flag"]
pub type ADC12IFG15_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IFG15` writer - ADC12 Memory 15 Interrupt Flag"]
pub type ADC12IFG15_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg0(&self) -> ADC12IFG0_R {
        ADC12IFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg1(&self) -> ADC12IFG1_R {
        ADC12IFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg2(&self) -> ADC12IFG2_R {
        ADC12IFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg3(&self) -> ADC12IFG3_R {
        ADC12IFG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg4(&self) -> ADC12IFG4_R {
        ADC12IFG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg5(&self) -> ADC12IFG5_R {
        ADC12IFG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg6(&self) -> ADC12IFG6_R {
        ADC12IFG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg7(&self) -> ADC12IFG7_R {
        ADC12IFG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg8(&self) -> ADC12IFG8_R {
        ADC12IFG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg9(&self) -> ADC12IFG9_R {
        ADC12IFG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg10(&self) -> ADC12IFG10_R {
        ADC12IFG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg11(&self) -> ADC12IFG11_R {
        ADC12IFG11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg12(&self) -> ADC12IFG12_R {
        ADC12IFG12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg13(&self) -> ADC12IFG13_R {
        ADC12IFG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg14(&self) -> ADC12IFG14_R {
        ADC12IFG14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg15(&self) -> ADC12IFG15_R {
        ADC12IFG15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg0(&mut self) -> ADC12IFG0_W<0> {
        ADC12IFG0_W::new(self)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg1(&mut self) -> ADC12IFG1_W<1> {
        ADC12IFG1_W::new(self)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg2(&mut self) -> ADC12IFG2_W<2> {
        ADC12IFG2_W::new(self)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg3(&mut self) -> ADC12IFG3_W<3> {
        ADC12IFG3_W::new(self)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg4(&mut self) -> ADC12IFG4_W<4> {
        ADC12IFG4_W::new(self)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg5(&mut self) -> ADC12IFG5_W<5> {
        ADC12IFG5_W::new(self)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg6(&mut self) -> ADC12IFG6_W<6> {
        ADC12IFG6_W::new(self)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg7(&mut self) -> ADC12IFG7_W<7> {
        ADC12IFG7_W::new(self)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg8(&mut self) -> ADC12IFG8_W<8> {
        ADC12IFG8_W::new(self)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg9(&mut self) -> ADC12IFG9_W<9> {
        ADC12IFG9_W::new(self)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg10(&mut self) -> ADC12IFG10_W<10> {
        ADC12IFG10_W::new(self)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg11(&mut self) -> ADC12IFG11_W<11> {
        ADC12IFG11_W::new(self)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg12(&mut self) -> ADC12IFG12_W<12> {
        ADC12IFG12_W::new(self)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg13(&mut self) -> ADC12IFG13_W<13> {
        ADC12IFG13_W::new(self)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg14(&mut self) -> ADC12IFG14_W<14> {
        ADC12IFG14_W::new(self)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ifg15(&mut self) -> ADC12IFG15_W<15> {
        ADC12IFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "ADC12+ Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ifg](index.html) module"]
pub struct ADC12IFG_SPEC;
impl crate::RegisterSpec for ADC12IFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ifg::R](R) reader structure"]
impl crate::Readable for ADC12IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ifg::W](W) writer structure"]
impl crate::Writable for ADC12IFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12IFG to value 0"]
impl crate::Resettable for ADC12IFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
