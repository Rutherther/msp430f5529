#[doc = "Register `CBCTL1` reader"]
pub struct R(crate::R<CBCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBCTL1` writer"]
pub struct W(crate::W<CBCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBCTL1_SPEC>;
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
impl From<crate::W<CBCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBOUT` reader - Comp. B Output"]
pub type CBOUT_R = crate::BitReader<bool>;
#[doc = "Field `CBOUT` writer - Comp. B Output"]
pub type CBOUT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
#[doc = "Field `CBOUTPOL` reader - Comp. B Output Polarity"]
pub type CBOUTPOL_R = crate::BitReader<bool>;
#[doc = "Field `CBOUTPOL` writer - Comp. B Output Polarity"]
pub type CBOUTPOL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
#[doc = "Field `CBF` reader - Comp. B Enable Output Filter"]
pub type CBF_R = crate::BitReader<bool>;
#[doc = "Field `CBF` writer - Comp. B Enable Output Filter"]
pub type CBF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
#[doc = "Field `CBIES` reader - Comp. B Interrupt Edge Select"]
pub type CBIES_R = crate::BitReader<bool>;
#[doc = "Field `CBIES` writer - Comp. B Interrupt Edge Select"]
pub type CBIES_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
#[doc = "Field `CBSHORT` reader - Comp. B Input Short"]
pub type CBSHORT_R = crate::BitReader<bool>;
#[doc = "Field `CBSHORT` writer - Comp. B Input Short"]
pub type CBSHORT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
#[doc = "Field `CBEX` reader - Comp. B Exchange Inputs"]
pub type CBEX_R = crate::BitReader<bool>;
#[doc = "Field `CBEX` writer - Comp. B Exchange Inputs"]
pub type CBEX_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
#[doc = "Field `CBFDLY` reader - Comp. B Filter delay Bit 0"]
pub type CBFDLY_R = crate::FieldReader<u8, CBFDLY_A>;
#[doc = "Comp. B Filter delay Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CBFDLY_A {
    #[doc = "0: Comp. B Filter delay 0 : 450ns"]
    CBFDLY_0 = 0,
    #[doc = "1: Comp. B Filter delay 1 : 900ns"]
    CBFDLY_1 = 1,
    #[doc = "2: Comp. B Filter delay 2 : 1800ns"]
    CBFDLY_2 = 2,
    #[doc = "3: Comp. B Filter delay 3 : 3600ns"]
    CBFDLY_3 = 3,
}
impl From<CBFDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CBFDLY_A) -> Self {
        variant as _
    }
}
impl CBFDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBFDLY_A {
        match self.bits {
            0 => CBFDLY_A::CBFDLY_0,
            1 => CBFDLY_A::CBFDLY_1,
            2 => CBFDLY_A::CBFDLY_2,
            3 => CBFDLY_A::CBFDLY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBFDLY_0`"]
    #[inline(always)]
    pub fn is_cbfdly_0(&self) -> bool {
        *self == CBFDLY_A::CBFDLY_0
    }
    #[doc = "Checks if the value of the field is `CBFDLY_1`"]
    #[inline(always)]
    pub fn is_cbfdly_1(&self) -> bool {
        *self == CBFDLY_A::CBFDLY_1
    }
    #[doc = "Checks if the value of the field is `CBFDLY_2`"]
    #[inline(always)]
    pub fn is_cbfdly_2(&self) -> bool {
        *self == CBFDLY_A::CBFDLY_2
    }
    #[doc = "Checks if the value of the field is `CBFDLY_3`"]
    #[inline(always)]
    pub fn is_cbfdly_3(&self) -> bool {
        *self == CBFDLY_A::CBFDLY_3
    }
}
#[doc = "Field `CBFDLY` writer - Comp. B Filter delay Bit 0"]
pub type CBFDLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CBCTL1_SPEC, u8, CBFDLY_A, 2, O>;
impl<'a, const O: u8> CBFDLY_W<'a, O> {
    #[doc = "Comp. B Filter delay 0 : 450ns"]
    #[inline(always)]
    pub fn cbfdly_0(self) -> &'a mut W {
        self.variant(CBFDLY_A::CBFDLY_0)
    }
    #[doc = "Comp. B Filter delay 1 : 900ns"]
    #[inline(always)]
    pub fn cbfdly_1(self) -> &'a mut W {
        self.variant(CBFDLY_A::CBFDLY_1)
    }
    #[doc = "Comp. B Filter delay 2 : 1800ns"]
    #[inline(always)]
    pub fn cbfdly_2(self) -> &'a mut W {
        self.variant(CBFDLY_A::CBFDLY_2)
    }
    #[doc = "Comp. B Filter delay 3 : 3600ns"]
    #[inline(always)]
    pub fn cbfdly_3(self) -> &'a mut W {
        self.variant(CBFDLY_A::CBFDLY_3)
    }
}
#[doc = "Field `CBPWRMD` reader - Comp. B Power Mode Bit 0"]
pub type CBPWRMD_R = crate::FieldReader<u8, CBPWRMD_A>;
#[doc = "Comp. B Power Mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CBPWRMD_A {
    #[doc = "0: Comp. B Power Mode 0 : High speed"]
    CBPWRMD_0 = 0,
    #[doc = "1: Comp. B Power Mode 1 : Normal"]
    CBPWRMD_1 = 1,
    #[doc = "2: Comp. B Power Mode 2 : Ultra-Low"]
    CBPWRMD_2 = 2,
    #[doc = "3: Comp. B Power Mode 3 : Reserved"]
    CBPWRMD_3 = 3,
}
impl From<CBPWRMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CBPWRMD_A) -> Self {
        variant as _
    }
}
impl CBPWRMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBPWRMD_A {
        match self.bits {
            0 => CBPWRMD_A::CBPWRMD_0,
            1 => CBPWRMD_A::CBPWRMD_1,
            2 => CBPWRMD_A::CBPWRMD_2,
            3 => CBPWRMD_A::CBPWRMD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_0`"]
    #[inline(always)]
    pub fn is_cbpwrmd_0(&self) -> bool {
        *self == CBPWRMD_A::CBPWRMD_0
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_1`"]
    #[inline(always)]
    pub fn is_cbpwrmd_1(&self) -> bool {
        *self == CBPWRMD_A::CBPWRMD_1
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_2`"]
    #[inline(always)]
    pub fn is_cbpwrmd_2(&self) -> bool {
        *self == CBPWRMD_A::CBPWRMD_2
    }
    #[doc = "Checks if the value of the field is `CBPWRMD_3`"]
    #[inline(always)]
    pub fn is_cbpwrmd_3(&self) -> bool {
        *self == CBPWRMD_A::CBPWRMD_3
    }
}
#[doc = "Field `CBPWRMD` writer - Comp. B Power Mode Bit 0"]
pub type CBPWRMD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CBCTL1_SPEC, u8, CBPWRMD_A, 2, O>;
impl<'a, const O: u8> CBPWRMD_W<'a, O> {
    #[doc = "Comp. B Power Mode 0 : High speed"]
    #[inline(always)]
    pub fn cbpwrmd_0(self) -> &'a mut W {
        self.variant(CBPWRMD_A::CBPWRMD_0)
    }
    #[doc = "Comp. B Power Mode 1 : Normal"]
    #[inline(always)]
    pub fn cbpwrmd_1(self) -> &'a mut W {
        self.variant(CBPWRMD_A::CBPWRMD_1)
    }
    #[doc = "Comp. B Power Mode 2 : Ultra-Low"]
    #[inline(always)]
    pub fn cbpwrmd_2(self) -> &'a mut W {
        self.variant(CBPWRMD_A::CBPWRMD_2)
    }
    #[doc = "Comp. B Power Mode 3 : Reserved"]
    #[inline(always)]
    pub fn cbpwrmd_3(self) -> &'a mut W {
        self.variant(CBPWRMD_A::CBPWRMD_3)
    }
}
#[doc = "Field `CBON` reader - Comp. B enable"]
pub type CBON_R = crate::BitReader<bool>;
#[doc = "Field `CBON` writer - Comp. B enable"]
pub type CBON_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
#[doc = "Field `CBMRVL` reader - Comp. B CBMRV Level"]
pub type CBMRVL_R = crate::BitReader<bool>;
#[doc = "Field `CBMRVL` writer - Comp. B CBMRV Level"]
pub type CBMRVL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
#[doc = "Field `CBMRVS` reader - Comp. B Output selects between VREF0 or VREF1"]
pub type CBMRVS_R = crate::BitReader<bool>;
#[doc = "Field `CBMRVS` writer - Comp. B Output selects between VREF0 or VREF1"]
pub type CBMRVS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comp. B Output"]
    #[inline(always)]
    pub fn cbout(&self) -> CBOUT_R {
        CBOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. B Output Polarity"]
    #[inline(always)]
    pub fn cboutpol(&self) -> CBOUTPOL_R {
        CBOUTPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. B Enable Output Filter"]
    #[inline(always)]
    pub fn cbf(&self) -> CBF_R {
        CBF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. B Interrupt Edge Select"]
    #[inline(always)]
    pub fn cbies(&self) -> CBIES_R {
        CBIES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comp. B Input Short"]
    #[inline(always)]
    pub fn cbshort(&self) -> CBSHORT_R {
        CBSHORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comp. B Exchange Inputs"]
    #[inline(always)]
    pub fn cbex(&self) -> CBEX_R {
        CBEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Comp. B Filter delay Bit 0"]
    #[inline(always)]
    pub fn cbfdly(&self) -> CBFDLY_R {
        CBFDLY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Comp. B Power Mode Bit 0"]
    #[inline(always)]
    pub fn cbpwrmd(&self) -> CBPWRMD_R {
        CBPWRMD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Comp. B enable"]
    #[inline(always)]
    pub fn cbon(&self) -> CBON_R {
        CBON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comp. B CBMRV Level"]
    #[inline(always)]
    pub fn cbmrvl(&self) -> CBMRVL_R {
        CBMRVL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Comp. B Output selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cbmrvs(&self) -> CBMRVS_R {
        CBMRVS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. B Output"]
    #[inline(always)]
    #[must_use]
    pub fn cbout(&mut self) -> CBOUT_W<0> {
        CBOUT_W::new(self)
    }
    #[doc = "Bit 1 - Comp. B Output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cboutpol(&mut self) -> CBOUTPOL_W<1> {
        CBOUTPOL_W::new(self)
    }
    #[doc = "Bit 2 - Comp. B Enable Output Filter"]
    #[inline(always)]
    #[must_use]
    pub fn cbf(&mut self) -> CBF_W<2> {
        CBF_W::new(self)
    }
    #[doc = "Bit 3 - Comp. B Interrupt Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn cbies(&mut self) -> CBIES_W<3> {
        CBIES_W::new(self)
    }
    #[doc = "Bit 4 - Comp. B Input Short"]
    #[inline(always)]
    #[must_use]
    pub fn cbshort(&mut self) -> CBSHORT_W<4> {
        CBSHORT_W::new(self)
    }
    #[doc = "Bit 5 - Comp. B Exchange Inputs"]
    #[inline(always)]
    #[must_use]
    pub fn cbex(&mut self) -> CBEX_W<5> {
        CBEX_W::new(self)
    }
    #[doc = "Bits 6:7 - Comp. B Filter delay Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cbfdly(&mut self) -> CBFDLY_W<6> {
        CBFDLY_W::new(self)
    }
    #[doc = "Bits 8:9 - Comp. B Power Mode Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cbpwrmd(&mut self) -> CBPWRMD_W<8> {
        CBPWRMD_W::new(self)
    }
    #[doc = "Bit 10 - Comp. B enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbon(&mut self) -> CBON_W<10> {
        CBON_W::new(self)
    }
    #[doc = "Bit 11 - Comp. B CBMRV Level"]
    #[inline(always)]
    #[must_use]
    pub fn cbmrvl(&mut self) -> CBMRVL_W<11> {
        CBMRVL_W::new(self)
    }
    #[doc = "Bit 12 - Comp. B Output selects between VREF0 or VREF1"]
    #[inline(always)]
    #[must_use]
    pub fn cbmrvs(&mut self) -> CBMRVS_W<12> {
        CBMRVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator B Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl1](index.html) module"]
pub struct CBCTL1_SPEC;
impl crate::RegisterSpec for CBCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cbctl1::R](R) reader structure"]
impl crate::Readable for CBCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbctl1::W](W) writer structure"]
impl crate::Writable for CBCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBCTL1 to value 0"]
impl crate::Resettable for CBCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
