#[doc = "Register `ADC12CTL1` reader"]
pub struct R(crate::R<ADC12CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC12CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC12CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12CTL1` writer"]
pub struct W(crate::W<ADC12CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12CTL1_SPEC>;
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
impl From<crate::W<ADC12CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC12CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12BUSY` reader - ADC12 Busy"]
pub type ADC12BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ADC12BUSY` writer - ADC12 Busy"]
pub type ADC12BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL1_SPEC, bool, O>;
#[doc = "Field `ADC12CONSEQ` reader - ADC12 Conversion Sequence Select Bit: 0"]
pub type ADC12CONSEQ_R = crate::FieldReader<u8, ADC12CONSEQ_A>;
#[doc = "ADC12 Conversion Sequence Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12CONSEQ_A {
    #[doc = "0: ADC12 Conversion Sequence Select: 0"]
    ADC12CONSEQ_0 = 0,
    #[doc = "1: ADC12 Conversion Sequence Select: 1"]
    ADC12CONSEQ_1 = 1,
    #[doc = "2: ADC12 Conversion Sequence Select: 2"]
    ADC12CONSEQ_2 = 2,
    #[doc = "3: ADC12 Conversion Sequence Select: 3"]
    ADC12CONSEQ_3 = 3,
}
impl From<ADC12CONSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12CONSEQ_A) -> Self {
        variant as _
    }
}
impl ADC12CONSEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12CONSEQ_A {
        match self.bits {
            0 => ADC12CONSEQ_A::ADC12CONSEQ_0,
            1 => ADC12CONSEQ_A::ADC12CONSEQ_1,
            2 => ADC12CONSEQ_A::ADC12CONSEQ_2,
            3 => ADC12CONSEQ_A::ADC12CONSEQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_0`"]
    #[inline(always)]
    pub fn is_adc12conseq_0(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_0
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_1`"]
    #[inline(always)]
    pub fn is_adc12conseq_1(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_1
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_2`"]
    #[inline(always)]
    pub fn is_adc12conseq_2(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_2
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_3`"]
    #[inline(always)]
    pub fn is_adc12conseq_3(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_3
    }
}
#[doc = "Field `ADC12CONSEQ` writer - ADC12 Conversion Sequence Select Bit: 0"]
pub type ADC12CONSEQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC12CTL1_SPEC, u8, ADC12CONSEQ_A, 2, O>;
impl<'a, const O: u8> ADC12CONSEQ_W<'a, O> {
    #[doc = "ADC12 Conversion Sequence Select: 0"]
    #[inline(always)]
    pub fn adc12conseq_0(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_0)
    }
    #[doc = "ADC12 Conversion Sequence Select: 1"]
    #[inline(always)]
    pub fn adc12conseq_1(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_1)
    }
    #[doc = "ADC12 Conversion Sequence Select: 2"]
    #[inline(always)]
    pub fn adc12conseq_2(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_2)
    }
    #[doc = "ADC12 Conversion Sequence Select: 3"]
    #[inline(always)]
    pub fn adc12conseq_3(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_3)
    }
}
#[doc = "Field `ADC12SSEL` reader - ADC12 Clock Source Select Bit: 0"]
pub type ADC12SSEL_R = crate::FieldReader<u8, ADC12SSEL_A>;
#[doc = "ADC12 Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12SSEL_A {
    #[doc = "0: ADC12 Clock Source Select: 0"]
    ADC12SSEL_0 = 0,
    #[doc = "1: ADC12 Clock Source Select: 1"]
    ADC12SSEL_1 = 1,
    #[doc = "2: ADC12 Clock Source Select: 2"]
    ADC12SSEL_2 = 2,
    #[doc = "3: ADC12 Clock Source Select: 3"]
    ADC12SSEL_3 = 3,
}
impl From<ADC12SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SSEL_A) -> Self {
        variant as _
    }
}
impl ADC12SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SSEL_A {
        match self.bits {
            0 => ADC12SSEL_A::ADC12SSEL_0,
            1 => ADC12SSEL_A::ADC12SSEL_1,
            2 => ADC12SSEL_A::ADC12SSEL_2,
            3 => ADC12SSEL_A::ADC12SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_0`"]
    #[inline(always)]
    pub fn is_adc12ssel_0(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_1`"]
    #[inline(always)]
    pub fn is_adc12ssel_1(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_2`"]
    #[inline(always)]
    pub fn is_adc12ssel_2(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_2
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_3`"]
    #[inline(always)]
    pub fn is_adc12ssel_3(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_3
    }
}
#[doc = "Field `ADC12SSEL` writer - ADC12 Clock Source Select Bit: 0"]
pub type ADC12SSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC12CTL1_SPEC, u8, ADC12SSEL_A, 2, O>;
impl<'a, const O: u8> ADC12SSEL_W<'a, O> {
    #[doc = "ADC12 Clock Source Select: 0"]
    #[inline(always)]
    pub fn adc12ssel_0(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_0)
    }
    #[doc = "ADC12 Clock Source Select: 1"]
    #[inline(always)]
    pub fn adc12ssel_1(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_1)
    }
    #[doc = "ADC12 Clock Source Select: 2"]
    #[inline(always)]
    pub fn adc12ssel_2(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_2)
    }
    #[doc = "ADC12 Clock Source Select: 3"]
    #[inline(always)]
    pub fn adc12ssel_3(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_3)
    }
}
#[doc = "Field `ADC12DIV` reader - ADC12 Clock Divider Select Bit: 0"]
pub type ADC12DIV_R = crate::FieldReader<u8, ADC12DIV_A>;
#[doc = "ADC12 Clock Divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12DIV_A {
    #[doc = "0: ADC12 Clock Divider Select: 0"]
    ADC12DIV_0 = 0,
    #[doc = "1: ADC12 Clock Divider Select: 1"]
    ADC12DIV_1 = 1,
    #[doc = "2: ADC12 Clock Divider Select: 2"]
    ADC12DIV_2 = 2,
    #[doc = "3: ADC12 Clock Divider Select: 3"]
    ADC12DIV_3 = 3,
    #[doc = "4: ADC12 Clock Divider Select: 4"]
    ADC12DIV_4 = 4,
    #[doc = "5: ADC12 Clock Divider Select: 5"]
    ADC12DIV_5 = 5,
    #[doc = "6: ADC12 Clock Divider Select: 6"]
    ADC12DIV_6 = 6,
    #[doc = "7: ADC12 Clock Divider Select: 7"]
    ADC12DIV_7 = 7,
}
impl From<ADC12DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12DIV_A) -> Self {
        variant as _
    }
}
impl ADC12DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12DIV_A {
        match self.bits {
            0 => ADC12DIV_A::ADC12DIV_0,
            1 => ADC12DIV_A::ADC12DIV_1,
            2 => ADC12DIV_A::ADC12DIV_2,
            3 => ADC12DIV_A::ADC12DIV_3,
            4 => ADC12DIV_A::ADC12DIV_4,
            5 => ADC12DIV_A::ADC12DIV_5,
            6 => ADC12DIV_A::ADC12DIV_6,
            7 => ADC12DIV_A::ADC12DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_0`"]
    #[inline(always)]
    pub fn is_adc12div_0(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_0
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_1`"]
    #[inline(always)]
    pub fn is_adc12div_1(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_1
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_2`"]
    #[inline(always)]
    pub fn is_adc12div_2(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_2
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_3`"]
    #[inline(always)]
    pub fn is_adc12div_3(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_3
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_4`"]
    #[inline(always)]
    pub fn is_adc12div_4(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_4
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_5`"]
    #[inline(always)]
    pub fn is_adc12div_5(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_5
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_6`"]
    #[inline(always)]
    pub fn is_adc12div_6(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_6
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_7`"]
    #[inline(always)]
    pub fn is_adc12div_7(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_7
    }
}
#[doc = "Field `ADC12DIV` writer - ADC12 Clock Divider Select Bit: 0"]
pub type ADC12DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC12CTL1_SPEC, u8, ADC12DIV_A, 3, O>;
impl<'a, const O: u8> ADC12DIV_W<'a, O> {
    #[doc = "ADC12 Clock Divider Select: 0"]
    #[inline(always)]
    pub fn adc12div_0(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_0)
    }
    #[doc = "ADC12 Clock Divider Select: 1"]
    #[inline(always)]
    pub fn adc12div_1(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_1)
    }
    #[doc = "ADC12 Clock Divider Select: 2"]
    #[inline(always)]
    pub fn adc12div_2(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_2)
    }
    #[doc = "ADC12 Clock Divider Select: 3"]
    #[inline(always)]
    pub fn adc12div_3(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_3)
    }
    #[doc = "ADC12 Clock Divider Select: 4"]
    #[inline(always)]
    pub fn adc12div_4(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_4)
    }
    #[doc = "ADC12 Clock Divider Select: 5"]
    #[inline(always)]
    pub fn adc12div_5(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_5)
    }
    #[doc = "ADC12 Clock Divider Select: 6"]
    #[inline(always)]
    pub fn adc12div_6(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_6)
    }
    #[doc = "ADC12 Clock Divider Select: 7"]
    #[inline(always)]
    pub fn adc12div_7(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_7)
    }
}
#[doc = "Field `ADC12ISSH` reader - ADC12 Invert Sample Hold Signal"]
pub type ADC12ISSH_R = crate::BitReader<bool>;
#[doc = "Field `ADC12ISSH` writer - ADC12 Invert Sample Hold Signal"]
pub type ADC12ISSH_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL1_SPEC, bool, O>;
#[doc = "Field `ADC12SHP` reader - ADC12 Sample/Hold Pulse Mode"]
pub type ADC12SHP_R = crate::BitReader<bool>;
#[doc = "Field `ADC12SHP` writer - ADC12 Sample/Hold Pulse Mode"]
pub type ADC12SHP_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADC12CTL1_SPEC, bool, O>;
#[doc = "Field `ADC12SHS` reader - ADC12 Sample/Hold Source Bit: 0"]
pub type ADC12SHS_R = crate::FieldReader<u8, ADC12SHS_A>;
#[doc = "ADC12 Sample/Hold Source Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12SHS_A {
    #[doc = "0: ADC12 Sample/Hold Source: 0"]
    ADC12SHS_0 = 0,
    #[doc = "1: ADC12 Sample/Hold Source: 1"]
    ADC12SHS_1 = 1,
    #[doc = "2: ADC12 Sample/Hold Source: 2"]
    ADC12SHS_2 = 2,
    #[doc = "3: ADC12 Sample/Hold Source: 3"]
    ADC12SHS_3 = 3,
}
impl From<ADC12SHS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHS_A) -> Self {
        variant as _
    }
}
impl ADC12SHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHS_A {
        match self.bits {
            0 => ADC12SHS_A::ADC12SHS_0,
            1 => ADC12SHS_A::ADC12SHS_1,
            2 => ADC12SHS_A::ADC12SHS_2,
            3 => ADC12SHS_A::ADC12SHS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_0`"]
    #[inline(always)]
    pub fn is_adc12shs_0(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_1`"]
    #[inline(always)]
    pub fn is_adc12shs_1(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_2`"]
    #[inline(always)]
    pub fn is_adc12shs_2(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_3`"]
    #[inline(always)]
    pub fn is_adc12shs_3(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_3
    }
}
#[doc = "Field `ADC12SHS` writer - ADC12 Sample/Hold Source Bit: 0"]
pub type ADC12SHS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC12CTL1_SPEC, u8, ADC12SHS_A, 2, O>;
impl<'a, const O: u8> ADC12SHS_W<'a, O> {
    #[doc = "ADC12 Sample/Hold Source: 0"]
    #[inline(always)]
    pub fn adc12shs_0(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_0)
    }
    #[doc = "ADC12 Sample/Hold Source: 1"]
    #[inline(always)]
    pub fn adc12shs_1(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_1)
    }
    #[doc = "ADC12 Sample/Hold Source: 2"]
    #[inline(always)]
    pub fn adc12shs_2(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_2)
    }
    #[doc = "ADC12 Sample/Hold Source: 3"]
    #[inline(always)]
    pub fn adc12shs_3(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_3)
    }
}
#[doc = "Field `ADC12CSTARTADD` reader - ADC12 Conversion Start Address Bit: 0"]
pub type ADC12CSTARTADD_R = crate::FieldReader<u8, ADC12CSTARTADD_A>;
#[doc = "ADC12 Conversion Start Address Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12CSTARTADD_A {
    #[doc = "0: ADC12 Conversion Start Address: 0"]
    ADC12CSTARTADD_0 = 0,
    #[doc = "1: ADC12 Conversion Start Address: 1"]
    ADC12CSTARTADD_1 = 1,
    #[doc = "2: ADC12 Conversion Start Address: 2"]
    ADC12CSTARTADD_2 = 2,
    #[doc = "3: ADC12 Conversion Start Address: 3"]
    ADC12CSTARTADD_3 = 3,
    #[doc = "4: ADC12 Conversion Start Address: 4"]
    ADC12CSTARTADD_4 = 4,
    #[doc = "5: ADC12 Conversion Start Address: 5"]
    ADC12CSTARTADD_5 = 5,
    #[doc = "6: ADC12 Conversion Start Address: 6"]
    ADC12CSTARTADD_6 = 6,
    #[doc = "7: ADC12 Conversion Start Address: 7"]
    ADC12CSTARTADD_7 = 7,
    #[doc = "8: ADC12 Conversion Start Address: 8"]
    ADC12CSTARTADD_8 = 8,
    #[doc = "9: ADC12 Conversion Start Address: 9"]
    ADC12CSTARTADD_9 = 9,
    #[doc = "10: ADC12 Conversion Start Address: 10"]
    ADC12CSTARTADD_10 = 10,
    #[doc = "11: ADC12 Conversion Start Address: 11"]
    ADC12CSTARTADD_11 = 11,
    #[doc = "12: ADC12 Conversion Start Address: 12"]
    ADC12CSTARTADD_12 = 12,
    #[doc = "13: ADC12 Conversion Start Address: 13"]
    ADC12CSTARTADD_13 = 13,
    #[doc = "14: ADC12 Conversion Start Address: 14"]
    ADC12CSTARTADD_14 = 14,
    #[doc = "15: ADC12 Conversion Start Address: 15"]
    ADC12CSTARTADD_15 = 15,
}
impl From<ADC12CSTARTADD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12CSTARTADD_A) -> Self {
        variant as _
    }
}
impl ADC12CSTARTADD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12CSTARTADD_A {
        match self.bits {
            0 => ADC12CSTARTADD_A::ADC12CSTARTADD_0,
            1 => ADC12CSTARTADD_A::ADC12CSTARTADD_1,
            2 => ADC12CSTARTADD_A::ADC12CSTARTADD_2,
            3 => ADC12CSTARTADD_A::ADC12CSTARTADD_3,
            4 => ADC12CSTARTADD_A::ADC12CSTARTADD_4,
            5 => ADC12CSTARTADD_A::ADC12CSTARTADD_5,
            6 => ADC12CSTARTADD_A::ADC12CSTARTADD_6,
            7 => ADC12CSTARTADD_A::ADC12CSTARTADD_7,
            8 => ADC12CSTARTADD_A::ADC12CSTARTADD_8,
            9 => ADC12CSTARTADD_A::ADC12CSTARTADD_9,
            10 => ADC12CSTARTADD_A::ADC12CSTARTADD_10,
            11 => ADC12CSTARTADD_A::ADC12CSTARTADD_11,
            12 => ADC12CSTARTADD_A::ADC12CSTARTADD_12,
            13 => ADC12CSTARTADD_A::ADC12CSTARTADD_13,
            14 => ADC12CSTARTADD_A::ADC12CSTARTADD_14,
            15 => ADC12CSTARTADD_A::ADC12CSTARTADD_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_0`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_0(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_0
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_1`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_1(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_1
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_2`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_2(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_2
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_3`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_3(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_3
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_4`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_4(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_4
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_5`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_5(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_5
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_6`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_6(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_6
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_7`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_7(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_7
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_8`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_8(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_8
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_9`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_9(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_9
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_10`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_10(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_10
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_11`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_11(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_11
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_12`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_12(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_12
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_13`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_13(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_13
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_14`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_14(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_14
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_15`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_15(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_15
    }
}
#[doc = "Field `ADC12CSTARTADD` writer - ADC12 Conversion Start Address Bit: 0"]
pub type ADC12CSTARTADD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC12CTL1_SPEC, u8, ADC12CSTARTADD_A, 4, O>;
impl<'a, const O: u8> ADC12CSTARTADD_W<'a, O> {
    #[doc = "ADC12 Conversion Start Address: 0"]
    #[inline(always)]
    pub fn adc12cstartadd_0(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_0)
    }
    #[doc = "ADC12 Conversion Start Address: 1"]
    #[inline(always)]
    pub fn adc12cstartadd_1(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_1)
    }
    #[doc = "ADC12 Conversion Start Address: 2"]
    #[inline(always)]
    pub fn adc12cstartadd_2(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_2)
    }
    #[doc = "ADC12 Conversion Start Address: 3"]
    #[inline(always)]
    pub fn adc12cstartadd_3(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_3)
    }
    #[doc = "ADC12 Conversion Start Address: 4"]
    #[inline(always)]
    pub fn adc12cstartadd_4(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_4)
    }
    #[doc = "ADC12 Conversion Start Address: 5"]
    #[inline(always)]
    pub fn adc12cstartadd_5(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_5)
    }
    #[doc = "ADC12 Conversion Start Address: 6"]
    #[inline(always)]
    pub fn adc12cstartadd_6(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_6)
    }
    #[doc = "ADC12 Conversion Start Address: 7"]
    #[inline(always)]
    pub fn adc12cstartadd_7(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_7)
    }
    #[doc = "ADC12 Conversion Start Address: 8"]
    #[inline(always)]
    pub fn adc12cstartadd_8(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_8)
    }
    #[doc = "ADC12 Conversion Start Address: 9"]
    #[inline(always)]
    pub fn adc12cstartadd_9(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_9)
    }
    #[doc = "ADC12 Conversion Start Address: 10"]
    #[inline(always)]
    pub fn adc12cstartadd_10(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_10)
    }
    #[doc = "ADC12 Conversion Start Address: 11"]
    #[inline(always)]
    pub fn adc12cstartadd_11(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_11)
    }
    #[doc = "ADC12 Conversion Start Address: 12"]
    #[inline(always)]
    pub fn adc12cstartadd_12(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_12)
    }
    #[doc = "ADC12 Conversion Start Address: 13"]
    #[inline(always)]
    pub fn adc12cstartadd_13(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_13)
    }
    #[doc = "ADC12 Conversion Start Address: 14"]
    #[inline(always)]
    pub fn adc12cstartadd_14(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_14)
    }
    #[doc = "ADC12 Conversion Start Address: 15"]
    #[inline(always)]
    pub fn adc12cstartadd_15(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Busy"]
    #[inline(always)]
    pub fn adc12busy(&self) -> ADC12BUSY_R {
        ADC12BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ADC12 Conversion Sequence Select Bit: 0"]
    #[inline(always)]
    pub fn adc12conseq(&self) -> ADC12CONSEQ_R {
        ADC12CONSEQ_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - ADC12 Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn adc12ssel(&self) -> ADC12SSEL_R {
        ADC12SSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - ADC12 Clock Divider Select Bit: 0"]
    #[inline(always)]
    pub fn adc12div(&self) -> ADC12DIV_R {
        ADC12DIV_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC12 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn adc12issh(&self) -> ADC12ISSH_R {
        ADC12ISSH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC12 Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn adc12shp(&self) -> ADC12SHP_R {
        ADC12SHP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ADC12 Sample/Hold Source Bit: 0"]
    #[inline(always)]
    pub fn adc12shs(&self) -> ADC12SHS_R {
        ADC12SHS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - ADC12 Conversion Start Address Bit: 0"]
    #[inline(always)]
    pub fn adc12cstartadd(&self) -> ADC12CSTARTADD_R {
        ADC12CSTARTADD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Busy"]
    #[inline(always)]
    #[must_use]
    pub fn adc12busy(&mut self) -> ADC12BUSY_W<0> {
        ADC12BUSY_W::new(self)
    }
    #[doc = "Bits 1:2 - ADC12 Conversion Sequence Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12conseq(&mut self) -> ADC12CONSEQ_W<1> {
        ADC12CONSEQ_W::new(self)
    }
    #[doc = "Bits 3:4 - ADC12 Clock Source Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ssel(&mut self) -> ADC12SSEL_W<3> {
        ADC12SSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - ADC12 Clock Divider Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12div(&mut self) -> ADC12DIV_W<5> {
        ADC12DIV_W::new(self)
    }
    #[doc = "Bit 8 - ADC12 Invert Sample Hold Signal"]
    #[inline(always)]
    #[must_use]
    pub fn adc12issh(&mut self) -> ADC12ISSH_W<8> {
        ADC12ISSH_W::new(self)
    }
    #[doc = "Bit 9 - ADC12 Sample/Hold Pulse Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc12shp(&mut self) -> ADC12SHP_W<9> {
        ADC12SHP_W::new(self)
    }
    #[doc = "Bits 10:11 - ADC12 Sample/Hold Source Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12shs(&mut self) -> ADC12SHS_W<10> {
        ADC12SHS_W::new(self)
    }
    #[doc = "Bits 12:15 - ADC12 Conversion Start Address Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc12cstartadd(&mut self) -> ADC12CSTARTADD_W<12> {
        ADC12CSTARTADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12+ Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl1](index.html) module"]
pub struct ADC12CTL1_SPEC;
impl crate::RegisterSpec for ADC12CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ctl1::R](R) reader structure"]
impl crate::Readable for ADC12CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ctl1::W](W) writer structure"]
impl crate::Writable for ADC12CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC12CTL1 to value 0"]
impl crate::Resettable for ADC12CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
