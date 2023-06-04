#[doc = "Register `USBMAINT` reader"]
pub struct R(crate::R<USBMAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBMAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBMAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBMAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBMAINT` writer"]
pub struct W(crate::W<USBMAINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBMAINT_SPEC>;
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
impl From<crate::W<USBMAINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBMAINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTIFG` reader - USB - Timer Interrupt Flag"]
pub type UTIFG_R = crate::BitReader<bool>;
#[doc = "Field `UTIFG` writer - USB - Timer Interrupt Flag"]
pub type UTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBMAINT_SPEC, bool, O>;
#[doc = "Field `UTIE` reader - USB - Timer Interrupt Enable"]
pub type UTIE_R = crate::BitReader<bool>;
#[doc = "Field `UTIE` writer - USB - Timer Interrupt Enable"]
pub type UTIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBMAINT_SPEC, bool, O>;
#[doc = "Field `TSGEN` reader - USB - Time Stamp Generator Enable"]
pub type TSGEN_R = crate::BitReader<bool>;
#[doc = "Field `TSGEN` writer - USB - Time Stamp Generator Enable"]
pub type TSGEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBMAINT_SPEC, bool, O>;
#[doc = "Field `TSESEL` reader - USB - Time Stamp Event Select Bit 0"]
pub type TSESEL_R = crate::FieldReader<u8, TSESEL_A>;
#[doc = "USB - Time Stamp Event Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSESEL_A {
    #[doc = "0: USB - Time Stamp Event Select: 0"]
    TSESEL_0 = 0,
    #[doc = "1: USB - Time Stamp Event Select: 1"]
    TSESEL_1 = 1,
    #[doc = "2: USB - Time Stamp Event Select: 2"]
    TSESEL_2 = 2,
    #[doc = "3: USB - Time Stamp Event Select: 3"]
    TSESEL_3 = 3,
}
impl From<TSESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSESEL_A) -> Self {
        variant as _
    }
}
impl TSESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSESEL_A {
        match self.bits {
            0 => TSESEL_A::TSESEL_0,
            1 => TSESEL_A::TSESEL_1,
            2 => TSESEL_A::TSESEL_2,
            3 => TSESEL_A::TSESEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TSESEL_0`"]
    #[inline(always)]
    pub fn is_tsesel_0(&self) -> bool {
        *self == TSESEL_A::TSESEL_0
    }
    #[doc = "Checks if the value of the field is `TSESEL_1`"]
    #[inline(always)]
    pub fn is_tsesel_1(&self) -> bool {
        *self == TSESEL_A::TSESEL_1
    }
    #[doc = "Checks if the value of the field is `TSESEL_2`"]
    #[inline(always)]
    pub fn is_tsesel_2(&self) -> bool {
        *self == TSESEL_A::TSESEL_2
    }
    #[doc = "Checks if the value of the field is `TSESEL_3`"]
    #[inline(always)]
    pub fn is_tsesel_3(&self) -> bool {
        *self == TSESEL_A::TSESEL_3
    }
}
#[doc = "Field `TSESEL` writer - USB - Time Stamp Event Select Bit 0"]
pub type TSESEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, USBMAINT_SPEC, u8, TSESEL_A, 2, O>;
impl<'a, const O: u8> TSESEL_W<'a, O> {
    #[doc = "USB - Time Stamp Event Select: 0"]
    #[inline(always)]
    pub fn tsesel_0(self) -> &'a mut W {
        self.variant(TSESEL_A::TSESEL_0)
    }
    #[doc = "USB - Time Stamp Event Select: 1"]
    #[inline(always)]
    pub fn tsesel_1(self) -> &'a mut W {
        self.variant(TSESEL_A::TSESEL_1)
    }
    #[doc = "USB - Time Stamp Event Select: 2"]
    #[inline(always)]
    pub fn tsesel_2(self) -> &'a mut W {
        self.variant(TSESEL_A::TSESEL_2)
    }
    #[doc = "USB - Time Stamp Event Select: 3"]
    #[inline(always)]
    pub fn tsesel_3(self) -> &'a mut W {
        self.variant(TSESEL_A::TSESEL_3)
    }
}
#[doc = "Field `TSE3` reader - USB - Time Stamp Event #3 Bit"]
pub type TSE3_R = crate::BitReader<bool>;
#[doc = "Field `TSE3` writer - USB - Time Stamp Event #3 Bit"]
pub type TSE3_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBMAINT_SPEC, bool, O>;
#[doc = "Field `UTSEL` reader - USB - Timer Select Bit 0"]
pub type UTSEL_R = crate::FieldReader<u8, UTSEL_A>;
#[doc = "USB - Timer Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UTSEL_A {
    #[doc = "0: USB - Timer Select: 0"]
    UTSEL_0 = 0,
    #[doc = "1: USB - Timer Select: 1"]
    UTSEL_1 = 1,
    #[doc = "2: USB - Timer Select: 2"]
    UTSEL_2 = 2,
    #[doc = "3: USB - Timer Select: 3"]
    UTSEL_3 = 3,
    #[doc = "4: USB - Timer Select: 4"]
    UTSEL_4 = 4,
    #[doc = "5: USB - Timer Select: 5"]
    UTSEL_5 = 5,
    #[doc = "6: USB - Timer Select: 6"]
    UTSEL_6 = 6,
    #[doc = "7: USB - Timer Select: 7"]
    UTSEL_7 = 7,
}
impl From<UTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UTSEL_A) -> Self {
        variant as _
    }
}
impl UTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTSEL_A {
        match self.bits {
            0 => UTSEL_A::UTSEL_0,
            1 => UTSEL_A::UTSEL_1,
            2 => UTSEL_A::UTSEL_2,
            3 => UTSEL_A::UTSEL_3,
            4 => UTSEL_A::UTSEL_4,
            5 => UTSEL_A::UTSEL_5,
            6 => UTSEL_A::UTSEL_6,
            7 => UTSEL_A::UTSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UTSEL_0`"]
    #[inline(always)]
    pub fn is_utsel_0(&self) -> bool {
        *self == UTSEL_A::UTSEL_0
    }
    #[doc = "Checks if the value of the field is `UTSEL_1`"]
    #[inline(always)]
    pub fn is_utsel_1(&self) -> bool {
        *self == UTSEL_A::UTSEL_1
    }
    #[doc = "Checks if the value of the field is `UTSEL_2`"]
    #[inline(always)]
    pub fn is_utsel_2(&self) -> bool {
        *self == UTSEL_A::UTSEL_2
    }
    #[doc = "Checks if the value of the field is `UTSEL_3`"]
    #[inline(always)]
    pub fn is_utsel_3(&self) -> bool {
        *self == UTSEL_A::UTSEL_3
    }
    #[doc = "Checks if the value of the field is `UTSEL_4`"]
    #[inline(always)]
    pub fn is_utsel_4(&self) -> bool {
        *self == UTSEL_A::UTSEL_4
    }
    #[doc = "Checks if the value of the field is `UTSEL_5`"]
    #[inline(always)]
    pub fn is_utsel_5(&self) -> bool {
        *self == UTSEL_A::UTSEL_5
    }
    #[doc = "Checks if the value of the field is `UTSEL_6`"]
    #[inline(always)]
    pub fn is_utsel_6(&self) -> bool {
        *self == UTSEL_A::UTSEL_6
    }
    #[doc = "Checks if the value of the field is `UTSEL_7`"]
    #[inline(always)]
    pub fn is_utsel_7(&self) -> bool {
        *self == UTSEL_A::UTSEL_7
    }
}
#[doc = "Field `UTSEL` writer - USB - Timer Select Bit 0"]
pub type UTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, USBMAINT_SPEC, u8, UTSEL_A, 3, O>;
impl<'a, const O: u8> UTSEL_W<'a, O> {
    #[doc = "USB - Timer Select: 0"]
    #[inline(always)]
    pub fn utsel_0(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_0)
    }
    #[doc = "USB - Timer Select: 1"]
    #[inline(always)]
    pub fn utsel_1(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_1)
    }
    #[doc = "USB - Timer Select: 2"]
    #[inline(always)]
    pub fn utsel_2(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_2)
    }
    #[doc = "USB - Timer Select: 3"]
    #[inline(always)]
    pub fn utsel_3(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_3)
    }
    #[doc = "USB - Timer Select: 4"]
    #[inline(always)]
    pub fn utsel_4(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_4)
    }
    #[doc = "USB - Timer Select: 5"]
    #[inline(always)]
    pub fn utsel_5(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_5)
    }
    #[doc = "USB - Timer Select: 6"]
    #[inline(always)]
    pub fn utsel_6(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_6)
    }
    #[doc = "USB - Timer Select: 7"]
    #[inline(always)]
    pub fn utsel_7(self) -> &'a mut W {
        self.variant(UTSEL_A::UTSEL_7)
    }
}
impl R {
    #[doc = "Bit 0 - USB - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn utifg(&self) -> UTIFG_R {
        UTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn utie(&self) -> UTIE_R {
        UTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - Time Stamp Generator Enable"]
    #[inline(always)]
    pub fn tsgen(&self) -> TSGEN_R {
        TSGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - USB - Time Stamp Event Select Bit 0"]
    #[inline(always)]
    pub fn tsesel(&self) -> TSESEL_R {
        TSESEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - USB - Time Stamp Event #3 Bit"]
    #[inline(always)]
    pub fn tse3(&self) -> TSE3_R {
        TSE3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 13:15 - USB - Timer Select Bit 0"]
    #[inline(always)]
    pub fn utsel(&self) -> UTSEL_R {
        UTSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Timer Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn utifg(&mut self) -> UTIFG_W<0> {
        UTIFG_W::new(self)
    }
    #[doc = "Bit 1 - USB - Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn utie(&mut self) -> UTIE_W<1> {
        UTIE_W::new(self)
    }
    #[doc = "Bit 8 - USB - Time Stamp Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsgen(&mut self) -> TSGEN_W<8> {
        TSGEN_W::new(self)
    }
    #[doc = "Bits 9:10 - USB - Time Stamp Event Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tsesel(&mut self) -> TSESEL_W<9> {
        TSESEL_W::new(self)
    }
    #[doc = "Bit 11 - USB - Time Stamp Event #3 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tse3(&mut self) -> TSE3_W<11> {
        TSE3_W::new(self)
    }
    #[doc = "Bits 13:15 - USB - Timer Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn utsel(&mut self) -> UTSEL_W<13> {
        UTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB maintenance register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbmaint](index.html) module"]
pub struct USBMAINT_SPEC;
impl crate::RegisterSpec for USBMAINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbmaint::R](R) reader structure"]
impl crate::Readable for USBMAINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbmaint::W](W) writer structure"]
impl crate::Writable for USBMAINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBMAINT to value 0"]
impl crate::Resettable for USBMAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
