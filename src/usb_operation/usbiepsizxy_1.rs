#[doc = "Register `USBIEPSIZXY_1` reader"]
pub struct R(crate::R<USBIEPSIZXY_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIEPSIZXY_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIEPSIZXY_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIEPSIZXY_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIEPSIZXY_1` writer"]
pub struct W(crate::W<USBIEPSIZXY_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIEPSIZXY_1_SPEC>;
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
impl From<crate::W<USBIEPSIZXY_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIEPSIZXY_1_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Endpoint_1: X/Y-buffer size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepsizxy_1](index.html) module"]
pub struct USBIEPSIZXY_1_SPEC;
impl crate::RegisterSpec for USBIEPSIZXY_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbiepsizxy_1::R](R) reader structure"]
impl crate::Readable for USBIEPSIZXY_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbiepsizxy_1::W](W) writer structure"]
impl crate::Writable for USBIEPSIZXY_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIEPSIZXY_1 to value 0"]
impl crate::Resettable for USBIEPSIZXY_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
