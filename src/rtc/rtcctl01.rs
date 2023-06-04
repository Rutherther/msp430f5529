#[doc = "Register `RTCCTL01` reader"]
pub struct R(crate::R<RTCCTL01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTL01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTL01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL01` writer"]
pub struct W(crate::W<RTCCTL01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL01_SPEC>;
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
impl From<crate::W<RTCCTL01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTL01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCRDYIFG` reader - RTC Ready Interrupt Flag"]
pub type RTCRDYIFG_R = crate::BitReader<bool>;
#[doc = "Field `RTCRDYIFG` writer - RTC Ready Interrupt Flag"]
pub type RTCRDYIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCAIFG` reader - RTC Alarm Interrupt Flag"]
pub type RTCAIFG_R = crate::BitReader<bool>;
#[doc = "Field `RTCAIFG` writer - RTC Alarm Interrupt Flag"]
pub type RTCAIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCTEVIFG` reader - RTC Time Event Interrupt Flag"]
pub type RTCTEVIFG_R = crate::BitReader<bool>;
#[doc = "Field `RTCTEVIFG` writer - RTC Time Event Interrupt Flag"]
pub type RTCTEVIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCRDYIE` reader - RTC Ready Interrupt Enable Flag"]
pub type RTCRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCRDYIE` writer - RTC Ready Interrupt Enable Flag"]
pub type RTCRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCAIE` reader - RTC Alarm Interrupt Enable Flag"]
pub type RTCAIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCAIE` writer - RTC Alarm Interrupt Enable Flag"]
pub type RTCAIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCTEVIE` reader - RTC Time Event Interrupt Enable Flag"]
pub type RTCTEVIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCTEVIE` writer - RTC Time Event Interrupt Enable Flag"]
pub type RTCTEVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCTEV` reader - RTC Time Event 1"]
pub type RTCTEV_R = crate::FieldReader<u8, RTCTEV_A>;
#[doc = "RTC Time Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCTEV_A {
    #[doc = "0: RTC Time Event: 0 (Min. changed)"]
    RTCTEV_0 = 0,
    #[doc = "1: RTC Time Event: 1 (Hour changed)"]
    RTCTEV_1 = 1,
    #[doc = "2: RTC Time Event: 2 (12:00 changed)"]
    RTCTEV_2 = 2,
    #[doc = "3: RTC Time Event: 3 (00:00 changed)"]
    RTCTEV_3 = 3,
}
impl From<RTCTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCTEV_A) -> Self {
        variant as _
    }
}
impl RTCTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEV_A {
        match self.bits {
            0 => RTCTEV_A::RTCTEV_0,
            1 => RTCTEV_A::RTCTEV_1,
            2 => RTCTEV_A::RTCTEV_2,
            3 => RTCTEV_A::RTCTEV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCTEV_0`"]
    #[inline(always)]
    pub fn is_rtctev_0(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_0
    }
    #[doc = "Checks if the value of the field is `RTCTEV_1`"]
    #[inline(always)]
    pub fn is_rtctev_1(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_1
    }
    #[doc = "Checks if the value of the field is `RTCTEV_2`"]
    #[inline(always)]
    pub fn is_rtctev_2(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_2
    }
    #[doc = "Checks if the value of the field is `RTCTEV_3`"]
    #[inline(always)]
    pub fn is_rtctev_3(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_3
    }
}
#[doc = "Field `RTCTEV` writer - RTC Time Event 1"]
pub type RTCTEV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, RTCCTL01_SPEC, u8, RTCTEV_A, 2, O>;
impl<'a, const O: u8> RTCTEV_W<'a, O> {
    #[doc = "RTC Time Event: 0 (Min. changed)"]
    #[inline(always)]
    pub fn rtctev_0(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_0)
    }
    #[doc = "RTC Time Event: 1 (Hour changed)"]
    #[inline(always)]
    pub fn rtctev_1(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_1)
    }
    #[doc = "RTC Time Event: 2 (12:00 changed)"]
    #[inline(always)]
    pub fn rtctev_2(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_2)
    }
    #[doc = "RTC Time Event: 3 (00:00 changed)"]
    #[inline(always)]
    pub fn rtctev_3(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_3)
    }
}
#[doc = "Field `RTCSSEL` reader - RTC Source Select 1"]
pub type RTCSSEL_R = crate::FieldReader<u8, RTCSSEL_A>;
#[doc = "RTC Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSSEL_A {
    #[doc = "0: RTC Source Select ACLK"]
    RTCSSEL_0 = 0,
    #[doc = "1: RTC Source Select SMCLK"]
    RTCSSEL_1 = 1,
    #[doc = "2: RTC Source Select RT1PS"]
    RTCSSEL_2 = 2,
    #[doc = "3: RTC Source Select RT1PS"]
    RTCSSEL_3 = 3,
}
impl From<RTCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSSEL_A) -> Self {
        variant as _
    }
}
impl RTCSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSSEL_A {
        match self.bits {
            0 => RTCSSEL_A::RTCSSEL_0,
            1 => RTCSSEL_A::RTCSSEL_1,
            2 => RTCSSEL_A::RTCSSEL_2,
            3 => RTCSSEL_A::RTCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_0`"]
    #[inline(always)]
    pub fn is_rtcssel_0(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_0
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_1`"]
    #[inline(always)]
    pub fn is_rtcssel_1(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_1
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_2`"]
    #[inline(always)]
    pub fn is_rtcssel_2(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_2
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_3`"]
    #[inline(always)]
    pub fn is_rtcssel_3(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_3
    }
}
#[doc = "Field `RTCSSEL` writer - RTC Source Select 1"]
pub type RTCSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, RTCCTL01_SPEC, u8, RTCSSEL_A, 2, O>;
impl<'a, const O: u8> RTCSSEL_W<'a, O> {
    #[doc = "RTC Source Select ACLK"]
    #[inline(always)]
    pub fn rtcssel_0(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_0)
    }
    #[doc = "RTC Source Select SMCLK"]
    #[inline(always)]
    pub fn rtcssel_1(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_1)
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn rtcssel_2(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_2)
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn rtcssel_3(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_3)
    }
}
#[doc = "Field `RTCRDY` reader - RTC Ready"]
pub type RTCRDY_R = crate::BitReader<bool>;
#[doc = "Field `RTCRDY` writer - RTC Ready"]
pub type RTCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCMODE` reader - RTC Mode 0:Counter / 1: Calendar"]
pub type RTCMODE_R = crate::BitReader<bool>;
#[doc = "Field `RTCMODE` writer - RTC Mode 0:Counter / 1: Calendar"]
pub type RTCMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCHOLD` reader - RTC Hold"]
pub type RTCHOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTCHOLD` writer - RTC Hold"]
pub type RTCHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
#[doc = "Field `RTCBCD` reader - RTC BCD 0:Binary / 1:BCD"]
pub type RTCBCD_R = crate::BitReader<bool>;
#[doc = "Field `RTCBCD` writer - RTC BCD 0:Binary / 1:BCD"]
pub type RTCBCD_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL01_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RTC Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RTCRDYIFG_R {
        RTCRDYIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Alarm Interrupt Flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RTCAIFG_R {
        RTCAIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Time Event Interrupt Flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RTCTEVIFG_R {
        RTCTEVIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Ready Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RTCRDYIE_R {
        RTCRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RTCAIE_R {
        RTCAIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC Time Event Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RTCTEVIE_R {
        RTCTEVIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC Time Event 1"]
    #[inline(always)]
    pub fn rtctev(&self) -> RTCTEV_R {
        RTCTEV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - RTC Source Select 1"]
    #[inline(always)]
    pub fn rtcssel(&self) -> RTCSSEL_R {
        RTCSSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - RTC Ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RTCRDY_R {
        RTCRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC Mode 0:Counter / 1: Calendar"]
    #[inline(always)]
    pub fn rtcmode(&self) -> RTCMODE_R {
        RTCMODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC Hold"]
    #[inline(always)]
    pub fn rtchold(&self) -> RTCHOLD_R {
        RTCHOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC BCD 0:Binary / 1:BCD"]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RTCBCD_R {
        RTCBCD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrdyifg(&mut self) -> RTCRDYIFG_W<0> {
        RTCRDYIFG_W::new(self)
    }
    #[doc = "Bit 1 - RTC Alarm Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcaifg(&mut self) -> RTCAIFG_W<1> {
        RTCAIFG_W::new(self)
    }
    #[doc = "Bit 2 - RTC Time Event Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtctevifg(&mut self) -> RTCTEVIFG_W<2> {
        RTCTEVIFG_W::new(self)
    }
    #[doc = "Bit 4 - RTC Ready Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrdyie(&mut self) -> RTCRDYIE_W<4> {
        RTCRDYIE_W::new(self)
    }
    #[doc = "Bit 5 - RTC Alarm Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcaie(&mut self) -> RTCAIE_W<5> {
        RTCAIE_W::new(self)
    }
    #[doc = "Bit 6 - RTC Time Event Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtctevie(&mut self) -> RTCTEVIE_W<6> {
        RTCTEVIE_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC Time Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtctev(&mut self) -> RTCTEV_W<8> {
        RTCTEV_W::new(self)
    }
    #[doc = "Bits 10:11 - RTC Source Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtcssel(&mut self) -> RTCSSEL_W<10> {
        RTCSSEL_W::new(self)
    }
    #[doc = "Bit 12 - RTC Ready"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrdy(&mut self) -> RTCRDY_W<12> {
        RTCRDY_W::new(self)
    }
    #[doc = "Bit 13 - RTC Mode 0:Counter / 1: Calendar"]
    #[inline(always)]
    #[must_use]
    pub fn rtcmode(&mut self) -> RTCMODE_W<13> {
        RTCMODE_W::new(self)
    }
    #[doc = "Bit 14 - RTC Hold"]
    #[inline(always)]
    #[must_use]
    pub fn rtchold(&mut self) -> RTCHOLD_W<14> {
        RTCHOLD_W::new(self)
    }
    #[doc = "Bit 15 - RTC BCD 0:Binary / 1:BCD"]
    #[inline(always)]
    #[must_use]
    pub fn rtcbcd(&mut self) -> RTCBCD_W<15> {
        RTCBCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Control 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl01](index.html) module"]
pub struct RTCCTL01_SPEC;
impl crate::RegisterSpec for RTCCTL01_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl01::R](R) reader structure"]
impl crate::Readable for RTCCTL01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl01::W](W) writer structure"]
impl crate::Writable for RTCCTL01_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCTL01 to value 0"]
impl crate::Resettable for RTCCTL01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
