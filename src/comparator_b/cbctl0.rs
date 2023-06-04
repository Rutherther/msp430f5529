#[doc = "Register `CBCTL0` reader"]
pub struct R(crate::R<CBCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBCTL0` writer"]
pub struct W(crate::W<CBCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBCTL0_SPEC>;
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
impl From<crate::W<CBCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBIPSEL` reader - Comp. B Pos. Channel Input Select 0"]
pub type CBIPSEL_R = crate::FieldReader<u8, CBIPSEL_A>;
#[doc = "Comp. B Pos. Channel Input Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CBIPSEL_A {
    #[doc = "0: Comp. B V+ terminal Input Select: Channel 0"]
    CBIPSEL_0 = 0,
    #[doc = "1: Comp. B V+ terminal Input Select: Channel 1"]
    CBIPSEL_1 = 1,
    #[doc = "2: Comp. B V+ terminal Input Select: Channel 2"]
    CBIPSEL_2 = 2,
    #[doc = "3: Comp. B V+ terminal Input Select: Channel 3"]
    CBIPSEL_3 = 3,
    #[doc = "4: Comp. B V+ terminal Input Select: Channel 4"]
    CBIPSEL_4 = 4,
    #[doc = "5: Comp. B V+ terminal Input Select: Channel 5"]
    CBIPSEL_5 = 5,
    #[doc = "6: Comp. B V+ terminal Input Select: Channel 6"]
    CBIPSEL_6 = 6,
    #[doc = "7: Comp. B V+ terminal Input Select: Channel 7"]
    CBIPSEL_7 = 7,
    #[doc = "8: Comp. B V+ terminal Input Select: Channel 8"]
    CBIPSEL_8 = 8,
    #[doc = "9: Comp. B V+ terminal Input Select: Channel 9"]
    CBIPSEL_9 = 9,
    #[doc = "10: Comp. B V+ terminal Input Select: Channel 10"]
    CBIPSEL_10 = 10,
    #[doc = "11: Comp. B V+ terminal Input Select: Channel 11"]
    CBIPSEL_11 = 11,
    #[doc = "12: Comp. B V+ terminal Input Select: Channel 12"]
    CBIPSEL_12 = 12,
    #[doc = "13: Comp. B V+ terminal Input Select: Channel 13"]
    CBIPSEL_13 = 13,
    #[doc = "14: Comp. B V+ terminal Input Select: Channel 14"]
    CBIPSEL_14 = 14,
    #[doc = "15: Comp. B V+ terminal Input Select: Channel 15"]
    CBIPSEL_15 = 15,
}
impl From<CBIPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CBIPSEL_A) -> Self {
        variant as _
    }
}
impl CBIPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBIPSEL_A {
        match self.bits {
            0 => CBIPSEL_A::CBIPSEL_0,
            1 => CBIPSEL_A::CBIPSEL_1,
            2 => CBIPSEL_A::CBIPSEL_2,
            3 => CBIPSEL_A::CBIPSEL_3,
            4 => CBIPSEL_A::CBIPSEL_4,
            5 => CBIPSEL_A::CBIPSEL_5,
            6 => CBIPSEL_A::CBIPSEL_6,
            7 => CBIPSEL_A::CBIPSEL_7,
            8 => CBIPSEL_A::CBIPSEL_8,
            9 => CBIPSEL_A::CBIPSEL_9,
            10 => CBIPSEL_A::CBIPSEL_10,
            11 => CBIPSEL_A::CBIPSEL_11,
            12 => CBIPSEL_A::CBIPSEL_12,
            13 => CBIPSEL_A::CBIPSEL_13,
            14 => CBIPSEL_A::CBIPSEL_14,
            15 => CBIPSEL_A::CBIPSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_0`"]
    #[inline(always)]
    pub fn is_cbipsel_0(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_0
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_1`"]
    #[inline(always)]
    pub fn is_cbipsel_1(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_1
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_2`"]
    #[inline(always)]
    pub fn is_cbipsel_2(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_2
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_3`"]
    #[inline(always)]
    pub fn is_cbipsel_3(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_3
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_4`"]
    #[inline(always)]
    pub fn is_cbipsel_4(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_4
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_5`"]
    #[inline(always)]
    pub fn is_cbipsel_5(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_5
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_6`"]
    #[inline(always)]
    pub fn is_cbipsel_6(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_6
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_7`"]
    #[inline(always)]
    pub fn is_cbipsel_7(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_7
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_8`"]
    #[inline(always)]
    pub fn is_cbipsel_8(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_8
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_9`"]
    #[inline(always)]
    pub fn is_cbipsel_9(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_9
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_10`"]
    #[inline(always)]
    pub fn is_cbipsel_10(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_10
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_11`"]
    #[inline(always)]
    pub fn is_cbipsel_11(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_11
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_12`"]
    #[inline(always)]
    pub fn is_cbipsel_12(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_12
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_13`"]
    #[inline(always)]
    pub fn is_cbipsel_13(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_13
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_14`"]
    #[inline(always)]
    pub fn is_cbipsel_14(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_14
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_15`"]
    #[inline(always)]
    pub fn is_cbipsel_15(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_15
    }
}
#[doc = "Field `CBIPSEL` writer - Comp. B Pos. Channel Input Select 0"]
pub type CBIPSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CBCTL0_SPEC, u8, CBIPSEL_A, 4, O>;
impl<'a, const O: u8> CBIPSEL_W<'a, O> {
    #[doc = "Comp. B V+ terminal Input Select: Channel 0"]
    #[inline(always)]
    pub fn cbipsel_0(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_0)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 1"]
    #[inline(always)]
    pub fn cbipsel_1(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_1)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 2"]
    #[inline(always)]
    pub fn cbipsel_2(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_2)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 3"]
    #[inline(always)]
    pub fn cbipsel_3(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_3)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 4"]
    #[inline(always)]
    pub fn cbipsel_4(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_4)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 5"]
    #[inline(always)]
    pub fn cbipsel_5(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_5)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 6"]
    #[inline(always)]
    pub fn cbipsel_6(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_6)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 7"]
    #[inline(always)]
    pub fn cbipsel_7(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_7)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 8"]
    #[inline(always)]
    pub fn cbipsel_8(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_8)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 9"]
    #[inline(always)]
    pub fn cbipsel_9(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_9)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 10"]
    #[inline(always)]
    pub fn cbipsel_10(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_10)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 11"]
    #[inline(always)]
    pub fn cbipsel_11(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_11)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 12"]
    #[inline(always)]
    pub fn cbipsel_12(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_12)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 13"]
    #[inline(always)]
    pub fn cbipsel_13(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_13)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 14"]
    #[inline(always)]
    pub fn cbipsel_14(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_14)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 15"]
    #[inline(always)]
    pub fn cbipsel_15(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_15)
    }
}
#[doc = "Field `CBIPEN` reader - Comp. B Pos. Channel Input Enable"]
pub type CBIPEN_R = crate::BitReader<bool>;
#[doc = "Field `CBIPEN` writer - Comp. B Pos. Channel Input Enable"]
pub type CBIPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL0_SPEC, bool, O>;
#[doc = "Field `CBIMSEL` reader - Comp. B Neg. Channel Input Select 0"]
pub type CBIMSEL_R = crate::FieldReader<u8, CBIMSEL_A>;
#[doc = "Comp. B Neg. Channel Input Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CBIMSEL_A {
    #[doc = "0: Comp. B V- Terminal Input Select: Channel 0"]
    CBIMSEL_0 = 0,
    #[doc = "1: Comp. B V- Terminal Input Select: Channel 1"]
    CBIMSEL_1 = 1,
    #[doc = "2: Comp. B V- Terminal Input Select: Channel 2"]
    CBIMSEL_2 = 2,
    #[doc = "3: Comp. B V- Terminal Input Select: Channel 3"]
    CBIMSEL_3 = 3,
    #[doc = "4: Comp. B V- Terminal Input Select: Channel 4"]
    CBIMSEL_4 = 4,
    #[doc = "5: Comp. B V- Terminal Input Select: Channel 5"]
    CBIMSEL_5 = 5,
    #[doc = "6: Comp. B V- Terminal Input Select: Channel 6"]
    CBIMSEL_6 = 6,
    #[doc = "7: Comp. B V- Terminal Input Select: Channel 7"]
    CBIMSEL_7 = 7,
    #[doc = "8: Comp. B V- terminal Input Select: Channel 8"]
    CBIMSEL_8 = 8,
    #[doc = "9: Comp. B V- terminal Input Select: Channel 9"]
    CBIMSEL_9 = 9,
    #[doc = "10: Comp. B V- terminal Input Select: Channel 10"]
    CBIMSEL_10 = 10,
    #[doc = "11: Comp. B V- terminal Input Select: Channel 11"]
    CBIMSEL_11 = 11,
    #[doc = "12: Comp. B V- terminal Input Select: Channel 12"]
    CBIMSEL_12 = 12,
    #[doc = "13: Comp. B V- terminal Input Select: Channel 13"]
    CBIMSEL_13 = 13,
    #[doc = "14: Comp. B V- terminal Input Select: Channel 14"]
    CBIMSEL_14 = 14,
    #[doc = "15: Comp. B V- terminal Input Select: Channel 15"]
    CBIMSEL_15 = 15,
}
impl From<CBIMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CBIMSEL_A) -> Self {
        variant as _
    }
}
impl CBIMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBIMSEL_A {
        match self.bits {
            0 => CBIMSEL_A::CBIMSEL_0,
            1 => CBIMSEL_A::CBIMSEL_1,
            2 => CBIMSEL_A::CBIMSEL_2,
            3 => CBIMSEL_A::CBIMSEL_3,
            4 => CBIMSEL_A::CBIMSEL_4,
            5 => CBIMSEL_A::CBIMSEL_5,
            6 => CBIMSEL_A::CBIMSEL_6,
            7 => CBIMSEL_A::CBIMSEL_7,
            8 => CBIMSEL_A::CBIMSEL_8,
            9 => CBIMSEL_A::CBIMSEL_9,
            10 => CBIMSEL_A::CBIMSEL_10,
            11 => CBIMSEL_A::CBIMSEL_11,
            12 => CBIMSEL_A::CBIMSEL_12,
            13 => CBIMSEL_A::CBIMSEL_13,
            14 => CBIMSEL_A::CBIMSEL_14,
            15 => CBIMSEL_A::CBIMSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_0`"]
    #[inline(always)]
    pub fn is_cbimsel_0(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_0
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_1`"]
    #[inline(always)]
    pub fn is_cbimsel_1(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_1
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_2`"]
    #[inline(always)]
    pub fn is_cbimsel_2(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_2
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_3`"]
    #[inline(always)]
    pub fn is_cbimsel_3(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_3
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_4`"]
    #[inline(always)]
    pub fn is_cbimsel_4(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_4
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_5`"]
    #[inline(always)]
    pub fn is_cbimsel_5(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_5
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_6`"]
    #[inline(always)]
    pub fn is_cbimsel_6(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_6
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_7`"]
    #[inline(always)]
    pub fn is_cbimsel_7(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_7
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_8`"]
    #[inline(always)]
    pub fn is_cbimsel_8(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_8
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_9`"]
    #[inline(always)]
    pub fn is_cbimsel_9(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_9
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_10`"]
    #[inline(always)]
    pub fn is_cbimsel_10(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_10
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_11`"]
    #[inline(always)]
    pub fn is_cbimsel_11(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_11
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_12`"]
    #[inline(always)]
    pub fn is_cbimsel_12(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_12
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_13`"]
    #[inline(always)]
    pub fn is_cbimsel_13(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_13
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_14`"]
    #[inline(always)]
    pub fn is_cbimsel_14(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_14
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_15`"]
    #[inline(always)]
    pub fn is_cbimsel_15(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_15
    }
}
#[doc = "Field `CBIMSEL` writer - Comp. B Neg. Channel Input Select 0"]
pub type CBIMSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CBCTL0_SPEC, u8, CBIMSEL_A, 4, O>;
impl<'a, const O: u8> CBIMSEL_W<'a, O> {
    #[doc = "Comp. B V- Terminal Input Select: Channel 0"]
    #[inline(always)]
    pub fn cbimsel_0(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_0)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 1"]
    #[inline(always)]
    pub fn cbimsel_1(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_1)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 2"]
    #[inline(always)]
    pub fn cbimsel_2(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_2)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 3"]
    #[inline(always)]
    pub fn cbimsel_3(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_3)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 4"]
    #[inline(always)]
    pub fn cbimsel_4(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_4)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 5"]
    #[inline(always)]
    pub fn cbimsel_5(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_5)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 6"]
    #[inline(always)]
    pub fn cbimsel_6(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_6)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 7"]
    #[inline(always)]
    pub fn cbimsel_7(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_7)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 8"]
    #[inline(always)]
    pub fn cbimsel_8(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_8)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 9"]
    #[inline(always)]
    pub fn cbimsel_9(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_9)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 10"]
    #[inline(always)]
    pub fn cbimsel_10(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_10)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 11"]
    #[inline(always)]
    pub fn cbimsel_11(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_11)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 12"]
    #[inline(always)]
    pub fn cbimsel_12(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_12)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 13"]
    #[inline(always)]
    pub fn cbimsel_13(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_13)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 14"]
    #[inline(always)]
    pub fn cbimsel_14(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_14)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 15"]
    #[inline(always)]
    pub fn cbimsel_15(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_15)
    }
}
#[doc = "Field `CBIMEN` reader - Comp. B Neg. Channel Input Enable"]
pub type CBIMEN_R = crate::BitReader<bool>;
#[doc = "Field `CBIMEN` writer - Comp. B Neg. Channel Input Enable"]
pub type CBIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CBCTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Comp. B Pos. Channel Input Select 0"]
    #[inline(always)]
    pub fn cbipsel(&self) -> CBIPSEL_R {
        CBIPSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Comp. B Pos. Channel Input Enable"]
    #[inline(always)]
    pub fn cbipen(&self) -> CBIPEN_R {
        CBIPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Comp. B Neg. Channel Input Select 0"]
    #[inline(always)]
    pub fn cbimsel(&self) -> CBIMSEL_R {
        CBIMSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comp. B Neg. Channel Input Enable"]
    #[inline(always)]
    pub fn cbimen(&self) -> CBIMEN_R {
        CBIMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comp. B Pos. Channel Input Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn cbipsel(&mut self) -> CBIPSEL_W<0> {
        CBIPSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comp. B Pos. Channel Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbipen(&mut self) -> CBIPEN_W<7> {
        CBIPEN_W::new(self)
    }
    #[doc = "Bits 8:11 - Comp. B Neg. Channel Input Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn cbimsel(&mut self) -> CBIMSEL_W<8> {
        CBIMSEL_W::new(self)
    }
    #[doc = "Bit 15 - Comp. B Neg. Channel Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbimen(&mut self) -> CBIMEN_W<15> {
        CBIMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator B Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbctl0](index.html) module"]
pub struct CBCTL0_SPEC;
impl crate::RegisterSpec for CBCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cbctl0::R](R) reader structure"]
impl crate::Readable for CBCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbctl0::W](W) writer structure"]
impl crate::Writable for CBCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBCTL0 to value 0"]
impl crate::Resettable for CBCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
