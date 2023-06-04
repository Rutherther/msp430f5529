#[doc = "Register `RTCPS1CTL` reader"]
pub struct R(crate::R<RTCPS1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCPS1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCPS1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCPS1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCPS1CTL` writer"]
pub struct W(crate::W<RTCPS1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCPS1CTL_SPEC>;
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
impl From<crate::W<RTCPS1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCPS1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT1PSIFG` reader - RTC Prescale Timer 1 Interrupt Flag"]
pub type RT1PSIFG_R = crate::BitReader<bool>;
#[doc = "Field `RT1PSIFG` writer - RTC Prescale Timer 1 Interrupt Flag"]
pub type RT1PSIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCPS1CTL_SPEC, bool, O>;
#[doc = "Field `RT1PSIE` reader - RTC Prescale Timer 1 Interrupt Enable Flag"]
pub type RT1PSIE_R = crate::BitReader<bool>;
#[doc = "Field `RT1PSIE` writer - RTC Prescale Timer 1 Interrupt Enable Flag"]
pub type RT1PSIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCPS1CTL_SPEC, bool, O>;
#[doc = "Field `RT1IP` reader - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
pub type RT1IP_R = crate::FieldReader<u8, RT1IP_A>;
#[doc = "RTC Prescale Timer 1 Interrupt Interval Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT1IP_A {
    #[doc = "0: RTC Prescale Timer 1 Interrupt Interval /2"]
    RT1IP_0 = 0,
    #[doc = "1: RTC Prescale Timer 1 Interrupt Interval /4"]
    RT1IP_1 = 1,
    #[doc = "2: RTC Prescale Timer 1 Interrupt Interval /8"]
    RT1IP_2 = 2,
    #[doc = "3: RTC Prescale Timer 1 Interrupt Interval /16"]
    RT1IP_3 = 3,
    #[doc = "4: RTC Prescale Timer 1 Interrupt Interval /32"]
    RT1IP_4 = 4,
    #[doc = "5: RTC Prescale Timer 1 Interrupt Interval /64"]
    RT1IP_5 = 5,
    #[doc = "6: RTC Prescale Timer 1 Interrupt Interval /128"]
    RT1IP_6 = 6,
    #[doc = "7: RTC Prescale Timer 1 Interrupt Interval /256"]
    RT1IP_7 = 7,
}
impl From<RT1IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1IP_A) -> Self {
        variant as _
    }
}
impl RT1IP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1IP_A {
        match self.bits {
            0 => RT1IP_A::RT1IP_0,
            1 => RT1IP_A::RT1IP_1,
            2 => RT1IP_A::RT1IP_2,
            3 => RT1IP_A::RT1IP_3,
            4 => RT1IP_A::RT1IP_4,
            5 => RT1IP_A::RT1IP_5,
            6 => RT1IP_A::RT1IP_6,
            7 => RT1IP_A::RT1IP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT1IP_0`"]
    #[inline(always)]
    pub fn is_rt1ip_0(&self) -> bool {
        *self == RT1IP_A::RT1IP_0
    }
    #[doc = "Checks if the value of the field is `RT1IP_1`"]
    #[inline(always)]
    pub fn is_rt1ip_1(&self) -> bool {
        *self == RT1IP_A::RT1IP_1
    }
    #[doc = "Checks if the value of the field is `RT1IP_2`"]
    #[inline(always)]
    pub fn is_rt1ip_2(&self) -> bool {
        *self == RT1IP_A::RT1IP_2
    }
    #[doc = "Checks if the value of the field is `RT1IP_3`"]
    #[inline(always)]
    pub fn is_rt1ip_3(&self) -> bool {
        *self == RT1IP_A::RT1IP_3
    }
    #[doc = "Checks if the value of the field is `RT1IP_4`"]
    #[inline(always)]
    pub fn is_rt1ip_4(&self) -> bool {
        *self == RT1IP_A::RT1IP_4
    }
    #[doc = "Checks if the value of the field is `RT1IP_5`"]
    #[inline(always)]
    pub fn is_rt1ip_5(&self) -> bool {
        *self == RT1IP_A::RT1IP_5
    }
    #[doc = "Checks if the value of the field is `RT1IP_6`"]
    #[inline(always)]
    pub fn is_rt1ip_6(&self) -> bool {
        *self == RT1IP_A::RT1IP_6
    }
    #[doc = "Checks if the value of the field is `RT1IP_7`"]
    #[inline(always)]
    pub fn is_rt1ip_7(&self) -> bool {
        *self == RT1IP_A::RT1IP_7
    }
}
#[doc = "Field `RT1IP` writer - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
pub type RT1IP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, RTCPS1CTL_SPEC, u8, RT1IP_A, 3, O>;
impl<'a, const O: u8> RT1IP_W<'a, O> {
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /2"]
    #[inline(always)]
    pub fn rt1ip_0(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_0)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /4"]
    #[inline(always)]
    pub fn rt1ip_1(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_1)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /8"]
    #[inline(always)]
    pub fn rt1ip_2(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_2)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /16"]
    #[inline(always)]
    pub fn rt1ip_3(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_3)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /32"]
    #[inline(always)]
    pub fn rt1ip_4(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_4)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /64"]
    #[inline(always)]
    pub fn rt1ip_5(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_5)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /128"]
    #[inline(always)]
    pub fn rt1ip_6(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_6)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /256"]
    #[inline(always)]
    pub fn rt1ip_7(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_7)
    }
}
#[doc = "Field `RT1PSHOLD` reader - RTC Prescale Timer 1 Hold"]
pub type RT1PSHOLD_R = crate::BitReader<bool>;
#[doc = "Field `RT1PSHOLD` writer - RTC Prescale Timer 1 Hold"]
pub type RT1PSHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCPS1CTL_SPEC, bool, O>;
#[doc = "Field `RT1PSDIV` reader - RTC Prescale Timer 1 Clock Divide Bit: 2"]
pub type RT1PSDIV_R = crate::FieldReader<u8, RT1PSDIV_A>;
#[doc = "RTC Prescale Timer 1 Clock Divide Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT1PSDIV_A {
    #[doc = "0: RTC Prescale Timer 1 Clock Divide /2"]
    RT1PSDIV_0 = 0,
    #[doc = "1: RTC Prescale Timer 1 Clock Divide /4"]
    RT1PSDIV_1 = 1,
    #[doc = "2: RTC Prescale Timer 1 Clock Divide /8"]
    RT1PSDIV_2 = 2,
    #[doc = "3: RTC Prescale Timer 1 Clock Divide /16"]
    RT1PSDIV_3 = 3,
    #[doc = "4: RTC Prescale Timer 1 Clock Divide /32"]
    RT1PSDIV_4 = 4,
    #[doc = "5: RTC Prescale Timer 1 Clock Divide /64"]
    RT1PSDIV_5 = 5,
    #[doc = "6: RTC Prescale Timer 1 Clock Divide /128"]
    RT1PSDIV_6 = 6,
    #[doc = "7: RTC Prescale Timer 1 Clock Divide /256"]
    RT1PSDIV_7 = 7,
}
impl From<RT1PSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1PSDIV_A) -> Self {
        variant as _
    }
}
impl RT1PSDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1PSDIV_A {
        match self.bits {
            0 => RT1PSDIV_A::RT1PSDIV_0,
            1 => RT1PSDIV_A::RT1PSDIV_1,
            2 => RT1PSDIV_A::RT1PSDIV_2,
            3 => RT1PSDIV_A::RT1PSDIV_3,
            4 => RT1PSDIV_A::RT1PSDIV_4,
            5 => RT1PSDIV_A::RT1PSDIV_5,
            6 => RT1PSDIV_A::RT1PSDIV_6,
            7 => RT1PSDIV_A::RT1PSDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_0`"]
    #[inline(always)]
    pub fn is_rt1psdiv_0(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_0
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_1`"]
    #[inline(always)]
    pub fn is_rt1psdiv_1(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_1
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_2`"]
    #[inline(always)]
    pub fn is_rt1psdiv_2(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_2
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_3`"]
    #[inline(always)]
    pub fn is_rt1psdiv_3(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_3
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_4`"]
    #[inline(always)]
    pub fn is_rt1psdiv_4(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_4
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_5`"]
    #[inline(always)]
    pub fn is_rt1psdiv_5(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_5
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_6`"]
    #[inline(always)]
    pub fn is_rt1psdiv_6(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_6
    }
    #[doc = "Checks if the value of the field is `RT1PSDIV_7`"]
    #[inline(always)]
    pub fn is_rt1psdiv_7(&self) -> bool {
        *self == RT1PSDIV_A::RT1PSDIV_7
    }
}
#[doc = "Field `RT1PSDIV` writer - RTC Prescale Timer 1 Clock Divide Bit: 2"]
pub type RT1PSDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, RTCPS1CTL_SPEC, u8, RT1PSDIV_A, 3, O>;
impl<'a, const O: u8> RT1PSDIV_W<'a, O> {
    #[doc = "RTC Prescale Timer 1 Clock Divide /2"]
    #[inline(always)]
    pub fn rt1psdiv_0(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_0)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /4"]
    #[inline(always)]
    pub fn rt1psdiv_1(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_1)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /8"]
    #[inline(always)]
    pub fn rt1psdiv_2(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_2)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /16"]
    #[inline(always)]
    pub fn rt1psdiv_3(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_3)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /32"]
    #[inline(always)]
    pub fn rt1psdiv_4(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_4)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /64"]
    #[inline(always)]
    pub fn rt1psdiv_5(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_5)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /128"]
    #[inline(always)]
    pub fn rt1psdiv_6(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_6)
    }
    #[doc = "RTC Prescale Timer 1 Clock Divide /256"]
    #[inline(always)]
    pub fn rt1psdiv_7(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::RT1PSDIV_7)
    }
}
#[doc = "Field `RT1SSEL` reader - RTC Prescale Timer 1 Source Select Bit 1"]
pub type RT1SSEL_R = crate::FieldReader<u8, RT1SSEL_A>;
#[doc = "RTC Prescale Timer 1 Source Select Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT1SSEL_A {
    #[doc = "0: RTC Prescale Timer Source Select ACLK"]
    RT1SSEL_0 = 0,
    #[doc = "1: RTC Prescale Timer Source Select SMCLK"]
    RT1SSEL_1 = 1,
    #[doc = "2: RTC Prescale Timer Source Select RT0PS"]
    RT1SSEL_2 = 2,
    #[doc = "3: RTC Prescale Timer Source Select RT0PS"]
    RT1SSEL_3 = 3,
}
impl From<RT1SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1SSEL_A) -> Self {
        variant as _
    }
}
impl RT1SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1SSEL_A {
        match self.bits {
            0 => RT1SSEL_A::RT1SSEL_0,
            1 => RT1SSEL_A::RT1SSEL_1,
            2 => RT1SSEL_A::RT1SSEL_2,
            3 => RT1SSEL_A::RT1SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_0`"]
    #[inline(always)]
    pub fn is_rt1ssel_0(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_0
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_1`"]
    #[inline(always)]
    pub fn is_rt1ssel_1(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_1
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_2`"]
    #[inline(always)]
    pub fn is_rt1ssel_2(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_2
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_3`"]
    #[inline(always)]
    pub fn is_rt1ssel_3(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_3
    }
}
#[doc = "Field `RT1SSEL` writer - RTC Prescale Timer 1 Source Select Bit 1"]
pub type RT1SSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, RTCPS1CTL_SPEC, u8, RT1SSEL_A, 2, O>;
impl<'a, const O: u8> RT1SSEL_W<'a, O> {
    #[doc = "RTC Prescale Timer Source Select ACLK"]
    #[inline(always)]
    pub fn rt1ssel_0(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_0)
    }
    #[doc = "RTC Prescale Timer Source Select SMCLK"]
    #[inline(always)]
    pub fn rt1ssel_1(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_1)
    }
    #[doc = "RTC Prescale Timer Source Select RT0PS"]
    #[inline(always)]
    pub fn rt1ssel_2(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_2)
    }
    #[doc = "RTC Prescale Timer Source Select RT0PS"]
    #[inline(always)]
    pub fn rt1ssel_3(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - RTC Prescale Timer 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rt1psifg(&self) -> RT1PSIFG_R {
        RT1PSIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 1 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt1psie(&self) -> RT1PSIE_R {
        RT1PSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt1ip(&self) -> RT1IP_R {
        RT1IP_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 8 - RTC Prescale Timer 1 Hold"]
    #[inline(always)]
    pub fn rt1pshold(&self) -> RT1PSHOLD_R {
        RT1PSHOLD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 1 Clock Divide Bit: 2"]
    #[inline(always)]
    pub fn rt1psdiv(&self) -> RT1PSDIV_R {
        RT1PSDIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - RTC Prescale Timer 1 Source Select Bit 1"]
    #[inline(always)]
    pub fn rt1ssel(&self) -> RT1SSEL_R {
        RT1SSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Prescale Timer 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rt1psifg(&mut self) -> RT1PSIFG_W<0> {
        RT1PSIFG_W::new(self)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 1 Interrupt Enable Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rt1psie(&mut self) -> RT1PSIE_W<1> {
        RT1PSIE_W::new(self)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn rt1ip(&mut self) -> RT1IP_W<2> {
        RT1IP_W::new(self)
    }
    #[doc = "Bit 8 - RTC Prescale Timer 1 Hold"]
    #[inline(always)]
    #[must_use]
    pub fn rt1pshold(&mut self) -> RT1PSHOLD_W<8> {
        RT1PSHOLD_W::new(self)
    }
    #[doc = "Bits 11:13 - RTC Prescale Timer 1 Clock Divide Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn rt1psdiv(&mut self) -> RT1PSDIV_W<11> {
        RT1PSDIV_W::new(self)
    }
    #[doc = "Bits 14:15 - RTC Prescale Timer 1 Source Select Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rt1ssel(&mut self) -> RT1SSEL_W<14> {
        RT1SSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Prescale Timer 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps1ctl](index.html) module"]
pub struct RTCPS1CTL_SPEC;
impl crate::RegisterSpec for RTCPS1CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcps1ctl::R](R) reader structure"]
impl crate::Readable for RTCPS1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcps1ctl::W](W) writer structure"]
impl crate::Writable for RTCPS1CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCPS1CTL to value 0"]
impl crate::Resettable for RTCPS1CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
