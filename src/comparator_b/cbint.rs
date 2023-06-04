#[doc = "Register `CBINT` reader"]
pub struct R(crate::R<CBINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBINT` writer"]
pub struct W(crate::W<CBINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBINT_SPEC>;
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
impl From<crate::W<CBINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBIFG` reader - Comp. B Interrupt Flag"]
pub type CBIFG_R = crate::BitReader<bool>;
#[doc = "Field `CBIFG` writer - Comp. B Interrupt Flag"]
pub type CBIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBINT_SPEC, bool, O>;
#[doc = "Field `CBIIFG` reader - Comp. B Interrupt Flag Inverted Polarity"]
pub type CBIIFG_R = crate::BitReader<bool>;
#[doc = "Field `CBIIFG` writer - Comp. B Interrupt Flag Inverted Polarity"]
pub type CBIIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBINT_SPEC, bool, O>;
#[doc = "Field `CBIE` reader - Comp. B Interrupt Enable"]
pub type CBIE_R = crate::BitReader<bool>;
#[doc = "Field `CBIE` writer - Comp. B Interrupt Enable"]
pub type CBIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBINT_SPEC, bool, O>;
#[doc = "Field `CBIIE` reader - Comp. B Interrupt Enable Inverted Polarity"]
pub type CBIIE_R = crate::BitReader<bool>;
#[doc = "Field `CBIIE` writer - Comp. B Interrupt Enable Inverted Polarity"]
pub type CBIIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comp. B Interrupt Flag"]
    #[inline(always)]
    pub fn cbifg(&self) -> CBIFG_R {
        CBIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. B Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn cbiifg(&self) -> CBIIFG_R {
        CBIIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Comp. B Interrupt Enable"]
    #[inline(always)]
    pub fn cbie(&self) -> CBIE_R {
        CBIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comp. B Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn cbiie(&self) -> CBIIE_R {
        CBIIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cbifg(&mut self) -> CBIFG_W<0> {
        CBIFG_W::new(self)
    }
    #[doc = "Bit 1 - Comp. B Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cbiifg(&mut self) -> CBIIFG_W<1> {
        CBIIFG_W::new(self)
    }
    #[doc = "Bit 8 - Comp. B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbie(&mut self) -> CBIE_W<8> {
        CBIE_W::new(self)
    }
    #[doc = "Bit 9 - Comp. B Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cbiie(&mut self) -> CBIIE_W<9> {
        CBIIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator B Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbint](index.html) module"]
pub struct CBINT_SPEC;
impl crate::RegisterSpec for CBINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cbint::R](R) reader structure"]
impl crate::Readable for CBINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbint::W](W) writer structure"]
impl crate::Writable for CBINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBINT to value 0"]
impl crate::Resettable for CBINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
