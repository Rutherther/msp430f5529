#[doc = "Register `ADC12IV` reader"]
pub struct R(crate::R<ADC12IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IV` writer"]
pub struct W(crate::W<ADC12IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IV_SPEC>;
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
impl From<crate::W<ADC12IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12IV_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12+ Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12iv](index.html) module"]
pub struct ADC12IV_SPEC;
impl crate::RegisterSpec for ADC12IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12iv::R](R) reader structure"]
impl crate::Readable for ADC12IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12iv::W](W) writer structure"]
impl crate::Writable for ADC12IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12IV to value 0"]
impl crate::Resettable for ADC12IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
