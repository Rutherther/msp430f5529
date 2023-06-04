#[doc = "Register `ADC12IE` reader"]
pub struct R(crate::R<ADC12IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IE` writer"]
pub struct W(crate::W<ADC12IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IE_SPEC>;
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
impl From<crate::W<ADC12IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12IE0` reader - ADC12 Memory 0 Interrupt Enable"]
pub type ADC12IE0_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE0` writer - ADC12 Memory 0 Interrupt Enable"]
pub type ADC12IE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE1` reader - ADC12 Memory 1 Interrupt Enable"]
pub type ADC12IE1_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE1` writer - ADC12 Memory 1 Interrupt Enable"]
pub type ADC12IE1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE2` reader - ADC12 Memory 2 Interrupt Enable"]
pub type ADC12IE2_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE2` writer - ADC12 Memory 2 Interrupt Enable"]
pub type ADC12IE2_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE3` reader - ADC12 Memory 3 Interrupt Enable"]
pub type ADC12IE3_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE3` writer - ADC12 Memory 3 Interrupt Enable"]
pub type ADC12IE3_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE4` reader - ADC12 Memory 4 Interrupt Enable"]
pub type ADC12IE4_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE4` writer - ADC12 Memory 4 Interrupt Enable"]
pub type ADC12IE4_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE5` reader - ADC12 Memory 5 Interrupt Enable"]
pub type ADC12IE5_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE5` writer - ADC12 Memory 5 Interrupt Enable"]
pub type ADC12IE5_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE6` reader - ADC12 Memory 6 Interrupt Enable"]
pub type ADC12IE6_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE6` writer - ADC12 Memory 6 Interrupt Enable"]
pub type ADC12IE6_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE7` reader - ADC12 Memory 7 Interrupt Enable"]
pub type ADC12IE7_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE7` writer - ADC12 Memory 7 Interrupt Enable"]
pub type ADC12IE7_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE8` reader - ADC12 Memory 8 Interrupt Enable"]
pub type ADC12IE8_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE8` writer - ADC12 Memory 8 Interrupt Enable"]
pub type ADC12IE8_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE9` reader - ADC12 Memory 9 Interrupt Enable"]
pub type ADC12IE9_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE9` writer - ADC12 Memory 9 Interrupt Enable"]
pub type ADC12IE9_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE10` reader - ADC12 Memory 10 Interrupt Enable"]
pub type ADC12IE10_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE10` writer - ADC12 Memory 10 Interrupt Enable"]
pub type ADC12IE10_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE11` reader - ADC12 Memory 11 Interrupt Enable"]
pub type ADC12IE11_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE11` writer - ADC12 Memory 11 Interrupt Enable"]
pub type ADC12IE11_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE12` reader - ADC12 Memory 12 Interrupt Enable"]
pub type ADC12IE12_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE12` writer - ADC12 Memory 12 Interrupt Enable"]
pub type ADC12IE12_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE13` reader - ADC12 Memory 13 Interrupt Enable"]
pub type ADC12IE13_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE13` writer - ADC12 Memory 13 Interrupt Enable"]
pub type ADC12IE13_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE14` reader - ADC12 Memory 14 Interrupt Enable"]
pub type ADC12IE14_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE14` writer - ADC12 Memory 14 Interrupt Enable"]
pub type ADC12IE14_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
#[doc = "Field `ADC12IE15` reader - ADC12 Memory 15 Interrupt Enable"]
pub type ADC12IE15_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE15` writer - ADC12 Memory 15 Interrupt Enable"]
pub type ADC12IE15_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie0(&self) -> ADC12IE0_R {
        ADC12IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie1(&self) -> ADC12IE1_R {
        ADC12IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie2(&self) -> ADC12IE2_R {
        ADC12IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie3(&self) -> ADC12IE3_R {
        ADC12IE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie4(&self) -> ADC12IE4_R {
        ADC12IE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie5(&self) -> ADC12IE5_R {
        ADC12IE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie6(&self) -> ADC12IE6_R {
        ADC12IE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie7(&self) -> ADC12IE7_R {
        ADC12IE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie8(&self) -> ADC12IE8_R {
        ADC12IE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie9(&self) -> ADC12IE9_R {
        ADC12IE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie10(&self) -> ADC12IE10_R {
        ADC12IE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie11(&self) -> ADC12IE11_R {
        ADC12IE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie12(&self) -> ADC12IE12_R {
        ADC12IE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie13(&self) -> ADC12IE13_R {
        ADC12IE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie14(&self) -> ADC12IE14_R {
        ADC12IE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie15(&self) -> ADC12IE15_R {
        ADC12IE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie0(&mut self) -> ADC12IE0_W<0> {
        ADC12IE0_W::new(self)
    }
    #[doc = "Bit 1 - ADC12 Memory 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie1(&mut self) -> ADC12IE1_W<1> {
        ADC12IE1_W::new(self)
    }
    #[doc = "Bit 2 - ADC12 Memory 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie2(&mut self) -> ADC12IE2_W<2> {
        ADC12IE2_W::new(self)
    }
    #[doc = "Bit 3 - ADC12 Memory 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie3(&mut self) -> ADC12IE3_W<3> {
        ADC12IE3_W::new(self)
    }
    #[doc = "Bit 4 - ADC12 Memory 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie4(&mut self) -> ADC12IE4_W<4> {
        ADC12IE4_W::new(self)
    }
    #[doc = "Bit 5 - ADC12 Memory 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie5(&mut self) -> ADC12IE5_W<5> {
        ADC12IE5_W::new(self)
    }
    #[doc = "Bit 6 - ADC12 Memory 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie6(&mut self) -> ADC12IE6_W<6> {
        ADC12IE6_W::new(self)
    }
    #[doc = "Bit 7 - ADC12 Memory 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie7(&mut self) -> ADC12IE7_W<7> {
        ADC12IE7_W::new(self)
    }
    #[doc = "Bit 8 - ADC12 Memory 8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie8(&mut self) -> ADC12IE8_W<8> {
        ADC12IE8_W::new(self)
    }
    #[doc = "Bit 9 - ADC12 Memory 9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie9(&mut self) -> ADC12IE9_W<9> {
        ADC12IE9_W::new(self)
    }
    #[doc = "Bit 10 - ADC12 Memory 10 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie10(&mut self) -> ADC12IE10_W<10> {
        ADC12IE10_W::new(self)
    }
    #[doc = "Bit 11 - ADC12 Memory 11 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie11(&mut self) -> ADC12IE11_W<11> {
        ADC12IE11_W::new(self)
    }
    #[doc = "Bit 12 - ADC12 Memory 12 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie12(&mut self) -> ADC12IE12_W<12> {
        ADC12IE12_W::new(self)
    }
    #[doc = "Bit 13 - ADC12 Memory 13 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie13(&mut self) -> ADC12IE13_W<13> {
        ADC12IE13_W::new(self)
    }
    #[doc = "Bit 14 - ADC12 Memory 14 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie14(&mut self) -> ADC12IE14_W<14> {
        ADC12IE14_W::new(self)
    }
    #[doc = "Bit 15 - ADC12 Memory 15 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie15(&mut self) -> ADC12IE15_W<15> {
        ADC12IE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "ADC12+ Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ie](index.html) module"]
pub struct ADC12IE_SPEC;
impl crate::RegisterSpec for ADC12IE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ie::R](R) reader structure"]
impl crate::Readable for ADC12IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ie::W](W) writer structure"]
impl crate::Writable for ADC12IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12IE to value 0"]
impl crate::Resettable for ADC12IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
