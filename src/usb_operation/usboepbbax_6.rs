#[doc = "Register `USBOEPBBAX_6` reader"]
pub struct R(crate::R<USBOEPBBAX_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBOEPBBAX_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBOEPBBAX_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBOEPBBAX_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBOEPBBAX_6` writer"]
pub struct W(crate::W<USBOEPBBAX_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBOEPBBAX_6_SPEC>;
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
impl From<crate::W<USBOEPBBAX_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBOEPBBAX_6_SPEC>) -> Self {
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
#[doc = "Output Endpoint_6: X-buffer base addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepbbax_6](index.html) module"]
pub struct USBOEPBBAX_6_SPEC;
impl crate::RegisterSpec for USBOEPBBAX_6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usboepbbax_6::R](R) reader structure"]
impl crate::Readable for USBOEPBBAX_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usboepbbax_6::W](W) writer structure"]
impl crate::Writable for USBOEPBBAX_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBOEPBBAX_6 to value 0"]
impl crate::Resettable for USBOEPBBAX_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
