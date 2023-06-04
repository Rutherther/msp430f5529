#[doc = "Register `PMMRIE` reader"]
pub struct R(crate::R<PMMRIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMRIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMRIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMRIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMRIE` writer"]
pub struct W(crate::W<PMMRIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMRIE_SPEC>;
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
impl From<crate::W<PMMRIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMRIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVSMLDLYIE` reader - SVS and SVM low side Delay expired interrupt enable"]
pub type SVSMLDLYIE_R = crate::BitReader<bool>;
#[doc = "Field `SVSMLDLYIE` writer - SVS and SVM low side Delay expired interrupt enable"]
pub type SVSMLDLYIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVMLIE` reader - SVM low side interrupt enable"]
pub type SVMLIE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLIE` writer - SVM low side interrupt enable"]
pub type SVMLIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVMLVLRIE` reader - SVM low side Voltage Level Reached interrupt enable"]
pub type SVMLVLRIE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLVLRIE` writer - SVM low side Voltage Level Reached interrupt enable"]
pub type SVMLVLRIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVSMHDLYIE` reader - SVS and SVM high side Delay expired interrupt enable"]
pub type SVSMHDLYIE_R = crate::BitReader<bool>;
#[doc = "Field `SVSMHDLYIE` writer - SVS and SVM high side Delay expired interrupt enable"]
pub type SVSMHDLYIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVMHIE` reader - SVM high side interrupt enable"]
pub type SVMHIE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHIE` writer - SVM high side interrupt enable"]
pub type SVMHIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVMHVLRIE` reader - SVM high side Voltage Level Reached interrupt enable"]
pub type SVMHVLRIE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHVLRIE` writer - SVM high side Voltage Level Reached interrupt enable"]
pub type SVMHVLRIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVSLPE` reader - SVS low side POR enable"]
pub type SVSLPE_R = crate::BitReader<bool>;
#[doc = "Field `SVSLPE` writer - SVS low side POR enable"]
pub type SVSLPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVMLVLRPE` reader - SVM low side Voltage Level reached POR enable"]
pub type SVMLVLRPE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLVLRPE` writer - SVM low side Voltage Level reached POR enable"]
pub type SVMLVLRPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVSHPE` reader - SVS high side POR enable"]
pub type SVSHPE_R = crate::BitReader<bool>;
#[doc = "Field `SVSHPE` writer - SVS high side POR enable"]
pub type SVSHPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
#[doc = "Field `SVMHVLRPE` reader - SVM high side Voltage Level reached POR enable"]
pub type SVMHVLRPE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHVLRPE` writer - SVM high side Voltage Level reached POR enable"]
pub type SVMHVLRPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmldlyie(&self) -> SVSMLDLYIE_R {
        SVSMLDLYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SVM low side interrupt enable"]
    #[inline(always)]
    pub fn svmlie(&self) -> SVMLIE_R {
        SVMLIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmlvlrie(&self) -> SVMLVLRIE_R {
        SVMLVLRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmhdlyie(&self) -> SVSMHDLYIE_R {
        SVSMHDLYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SVM high side interrupt enable"]
    #[inline(always)]
    pub fn svmhie(&self) -> SVMHIE_R {
        SVMHIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmhvlrie(&self) -> SVMHVLRIE_R {
        SVMHVLRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SVS low side POR enable"]
    #[inline(always)]
    pub fn svslpe(&self) -> SVSLPE_R {
        SVSLPE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SVM low side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmlvlrpe(&self) -> SVMLVLRPE_R {
        SVMLVLRPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SVS high side POR enable"]
    #[inline(always)]
    pub fn svshpe(&self) -> SVSHPE_R {
        SVSHPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SVM high side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmhvlrpe(&self) -> SVMHVLRPE_R {
        SVMHVLRPE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn svsmldlyie(&mut self) -> SVSMLDLYIE_W<0> {
        SVSMLDLYIE_W::new(self)
    }
    #[doc = "Bit 1 - SVM low side interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmlie(&mut self) -> SVMLIE_W<1> {
        SVMLIE_W::new(self)
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmlvlrie(&mut self) -> SVMLVLRIE_W<2> {
        SVMLVLRIE_W::new(self)
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn svsmhdlyie(&mut self) -> SVSMHDLYIE_W<4> {
        SVSMHDLYIE_W::new(self)
    }
    #[doc = "Bit 5 - SVM high side interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmhie(&mut self) -> SVMHIE_W<5> {
        SVMHIE_W::new(self)
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmhvlrie(&mut self) -> SVMHVLRIE_W<6> {
        SVMHVLRIE_W::new(self)
    }
    #[doc = "Bit 8 - SVS low side POR enable"]
    #[inline(always)]
    #[must_use]
    pub fn svslpe(&mut self) -> SVSLPE_W<8> {
        SVSLPE_W::new(self)
    }
    #[doc = "Bit 9 - SVM low side Voltage Level reached POR enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmlvlrpe(&mut self) -> SVMLVLRPE_W<9> {
        SVMLVLRPE_W::new(self)
    }
    #[doc = "Bit 12 - SVS high side POR enable"]
    #[inline(always)]
    #[must_use]
    pub fn svshpe(&mut self) -> SVSHPE_W<12> {
        SVSHPE_W::new(self)
    }
    #[doc = "Bit 13 - SVM high side Voltage Level reached POR enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmhvlrpe(&mut self) -> SVMHVLRPE_W<13> {
        SVMHVLRPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM and RESET Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmrie](index.html) module"]
pub struct PMMRIE_SPEC;
impl crate::RegisterSpec for PMMRIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmrie::R](R) reader structure"]
impl crate::Readable for PMMRIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmrie::W](W) writer structure"]
impl crate::Writable for PMMRIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMMRIE to value 0"]
impl crate::Resettable for PMMRIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
