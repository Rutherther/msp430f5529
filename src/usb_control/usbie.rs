#[doc = "Register `USBIE` reader"]
pub struct R(crate::R<USBIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIE` writer"]
pub struct W(crate::W<USBIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIE_SPEC>;
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
impl From<crate::W<USBIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPOWIE` reader - USB - Setup Overwrite Interrupt Enable"]
pub type STPOWIE_R = crate::BitReader<bool>;
#[doc = "Field `STPOWIE` writer - USB - Setup Overwrite Interrupt Enable"]
pub type STPOWIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIE_SPEC, bool, O>;
#[doc = "Field `SETUPIE` reader - USB - Setup Interrupt Enable"]
pub type SETUPIE_R = crate::BitReader<bool>;
#[doc = "Field `SETUPIE` writer - USB - Setup Interrupt Enable"]
pub type SETUPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIE_SPEC, bool, O>;
#[doc = "Field `RESRIE` reader - USB - Function Resume Request Interrupt Enable"]
pub type RESRIE_R = crate::BitReader<bool>;
#[doc = "Field `RESRIE` writer - USB - Function Resume Request Interrupt Enable"]
pub type RESRIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIE_SPEC, bool, O>;
#[doc = "Field `SUSRIE` reader - USB - Function Suspend Request Interrupt Enable"]
pub type SUSRIE_R = crate::BitReader<bool>;
#[doc = "Field `SUSRIE` writer - USB - Function Suspend Request Interrupt Enable"]
pub type SUSRIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIE_SPEC, bool, O>;
#[doc = "Field `RSTRIE` reader - USB - Function Reset Request Interrupt Enable"]
pub type RSTRIE_R = crate::BitReader<bool>;
#[doc = "Field `RSTRIE` writer - USB - Function Reset Request Interrupt Enable"]
pub type RSTRIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Enable"]
    #[inline(always)]
    pub fn stpowie(&self) -> STPOWIE_R {
        STPOWIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Enable"]
    #[inline(always)]
    pub fn setupie(&self) -> SETUPIE_R {
        SETUPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Enable"]
    #[inline(always)]
    pub fn resrie(&self) -> RESRIE_R {
        RESRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Enable"]
    #[inline(always)]
    pub fn susrie(&self) -> SUSRIE_R {
        SUSRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Enable"]
    #[inline(always)]
    pub fn rstrie(&self) -> RSTRIE_R {
        RSTRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Setup Overwrite Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stpowie(&mut self) -> STPOWIE_W<0> {
        STPOWIE_W::new(self)
    }
    #[doc = "Bit 2 - USB - Setup Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn setupie(&mut self) -> SETUPIE_W<2> {
        SETUPIE_W::new(self)
    }
    #[doc = "Bit 5 - USB - Function Resume Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resrie(&mut self) -> RESRIE_W<5> {
        RESRIE_W::new(self)
    }
    #[doc = "Bit 6 - USB - Function Suspend Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn susrie(&mut self) -> SUSRIE_W<6> {
        SUSRIE_W::new(self)
    }
    #[doc = "Bit 7 - USB - Function Reset Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstrie(&mut self) -> RSTRIE_W<7> {
        RSTRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbie](index.html) module"]
pub struct USBIE_SPEC;
impl crate::RegisterSpec for USBIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbie::R](R) reader structure"]
impl crate::Readable for USBIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbie::W](W) writer structure"]
impl crate::Writable for USBIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIE to value 0"]
impl crate::Resettable for USBIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
