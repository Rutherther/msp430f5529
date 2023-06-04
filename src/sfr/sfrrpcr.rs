#[doc = "Register `SFRRPCR` reader"]
pub struct R(crate::R<SFRRPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFRRPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFRRPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFRRPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFRRPCR` writer"]
pub struct W(crate::W<SFRRPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFRRPCR_SPEC>;
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
impl From<crate::W<SFRRPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFRRPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSNMI` reader - NMI select"]
pub type SYSNMI_R = crate::BitReader<bool>;
#[doc = "Field `SYSNMI` writer - NMI select"]
pub type SYSNMI_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRRPCR_SPEC, bool, O>;
#[doc = "Field `SYSNMIIES` reader - NMI edge select"]
pub type SYSNMIIES_R = crate::BitReader<bool>;
#[doc = "Field `SYSNMIIES` writer - NMI edge select"]
pub type SYSNMIIES_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRRPCR_SPEC, bool, O>;
#[doc = "Field `SYSRSTUP` reader - RESET Pin pull down/up select"]
pub type SYSRSTUP_R = crate::BitReader<bool>;
#[doc = "Field `SYSRSTUP` writer - RESET Pin pull down/up select"]
pub type SYSRSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRRPCR_SPEC, bool, O>;
#[doc = "Field `SYSRSTRE` reader - RESET Pin Resistor enable"]
pub type SYSRSTRE_R = crate::BitReader<bool>;
#[doc = "Field `SYSRSTRE` writer - RESET Pin Resistor enable"]
pub type SYSRSTRE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SFRRPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&self) -> SYSNMI_R {
        SYSNMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&self) -> SYSNMIIES_R {
        SYSNMIIES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    pub fn sysrstup(&self) -> SYSRSTUP_R {
        SYSRSTUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&self) -> SYSRSTRE_R {
        SYSRSTRE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    #[must_use]
    pub fn sysnmi(&mut self) -> SYSNMI_W<0> {
        SYSNMI_W::new(self)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    #[must_use]
    pub fn sysnmiies(&mut self) -> SYSNMIIES_W<1> {
        SYSNMIIES_W::new(self)
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    #[must_use]
    pub fn sysrstup(&mut self) -> SYSRSTUP_W<2> {
        SYSRSTUP_W::new(self)
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysrstre(&mut self) -> SYSRSTRE_W<3> {
        SYSRSTRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RESET Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrrpcr](index.html) module"]
pub struct SFRRPCR_SPEC;
impl crate::RegisterSpec for SFRRPCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sfrrpcr::R](R) reader structure"]
impl crate::Readable for SFRRPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfrrpcr::W](W) writer structure"]
impl crate::Writable for SFRRPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFRRPCR to value 0"]
impl crate::Resettable for SFRRPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
