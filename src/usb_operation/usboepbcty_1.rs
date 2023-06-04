#[doc = "Register `USBOEPBCTY_1` reader"]
pub struct R(crate::R<USBOEPBCTY_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBOEPBCTY_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBOEPBCTY_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBOEPBCTY_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBOEPBCTY_1` writer"]
pub struct W(crate::W<USBOEPBCTY_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBOEPBCTY_1_SPEC>;
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
impl From<crate::W<USBOEPBCTY_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBOEPBCTY_1_SPEC>) -> Self {
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
#[doc = "Output Endpoint_1: Y-byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbcty_1](index.html) module"]
pub struct USBOEPBCTY_1_SPEC;
impl crate::RegisterSpec for USBOEPBCTY_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usboepbcty_1::R](R) reader structure"]
impl crate::Readable for USBOEPBCTY_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usboepbcty_1::W](W) writer structure"]
impl crate::Writable for USBOEPBCTY_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBOEPBCTY_1 to value 0"]
impl crate::Resettable for USBOEPBCTY_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
