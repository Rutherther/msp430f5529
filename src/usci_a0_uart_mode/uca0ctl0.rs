#[doc = "Register `UCA0CTL0` reader"]
pub struct R(crate::R<UCA0CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0CTL0` writer"]
pub struct W(crate::W<UCA0CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0CTL0_SPEC>;
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
impl From<crate::W<UCA0CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub struct UCSYNC_R(crate::FieldReader<bool, bool>);
impl UCSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub struct UCSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSYNC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Async. Mode: USCI Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: Sync. Mode: USCI Mode: 0"]
    UCMODE_0 = 0,
    #[doc = "1: Sync. Mode: USCI Mode: 1"]
    UCMODE_1 = 1,
    #[doc = "2: Sync. Mode: USCI Mode: 2"]
    UCMODE_2 = 2,
    #[doc = "3: Sync. Mode: USCI Mode: 3"]
    UCMODE_3 = 3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCMODE` reader - Async. Mode: USCI Mode 1"]
pub struct UCMODE_R(crate::FieldReader<u8, UCMODE_A>);
impl UCMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UCMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCMODE_0`"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        **self == UCMODE_A::UCMODE_0
    }
    #[doc = "Checks if the value of the field is `UCMODE_1`"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        **self == UCMODE_A::UCMODE_1
    }
    #[doc = "Checks if the value of the field is `UCMODE_2`"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        **self == UCMODE_A::UCMODE_2
    }
    #[doc = "Checks if the value of the field is `UCMODE_3`"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        **self == UCMODE_A::UCMODE_3
    }
}
impl core::ops::Deref for UCMODE_R {
    type Target = crate::FieldReader<u8, UCMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCMODE` writer - Async. Mode: USCI Mode 1"]
pub struct UCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u8 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `UCSPB` reader - Async. Mode: Stop Bits 0:one / 1: two"]
pub struct UCSPB_R(crate::FieldReader<bool, bool>);
impl UCSPB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSPB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSPB` writer - Async. Mode: Stop Bits 0:one / 1: two"]
pub struct UCSPB_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSPB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `UC7BIT` reader - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub struct UC7BIT_R(crate::FieldReader<bool, bool>);
impl UC7BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UC7BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UC7BIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UC7BIT` writer - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub struct UC7BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UC7BIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UCMSB` reader - Async. Mode: MSB first 0:LSB / 1:MSB"]
pub struct UCMSB_R(crate::FieldReader<bool, bool>);
impl UCMSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCMSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCMSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCMSB` writer - Async. Mode: MSB first 0:LSB / 1:MSB"]
pub struct UCMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMSB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UCPAR` reader - Async. Mode: Parity 0:odd / 1:even"]
pub struct UCPAR_R(crate::FieldReader<bool, bool>);
impl UCPAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCPAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPAR` writer - Async. Mode: Parity 0:odd / 1:even"]
pub struct UCPAR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPAR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `UCPEN` reader - Async. Mode: Parity enable"]
pub struct UCPEN_R(crate::FieldReader<bool, bool>);
impl UCPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPEN` writer - Async. Mode: Parity enable"]
pub struct UCPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Async. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Async. Mode: Stop Bits 0:one / 1: two"]
    #[inline(always)]
    pub fn ucspb(&self) -> UCSPB_R {
        UCSPB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Async. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Async. Mode: Parity 0:odd / 1:even"]
    #[inline(always)]
    pub fn ucpar(&self) -> UCPAR_R {
        UCPAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Async. Mode: Parity enable"]
    #[inline(always)]
    pub fn ucpen(&self) -> UCPEN_R {
        UCPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W {
        UCSYNC_W { w: self }
    }
    #[doc = "Bits 1:2 - Async. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W {
        UCMODE_W { w: self }
    }
    #[doc = "Bit 3 - Async. Mode: Stop Bits 0:one / 1: two"]
    #[inline(always)]
    pub fn ucspb(&mut self) -> UCSPB_W {
        UCSPB_W { w: self }
    }
    #[doc = "Bit 4 - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> UC7BIT_W {
        UC7BIT_W { w: self }
    }
    #[doc = "Bit 5 - Async. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UCMSB_W {
        UCMSB_W { w: self }
    }
    #[doc = "Bit 6 - Async. Mode: Parity 0:odd / 1:even"]
    #[inline(always)]
    pub fn ucpar(&mut self) -> UCPAR_W {
        UCPAR_W { w: self }
    }
    #[doc = "Bit 7 - Async. Mode: Parity enable"]
    #[inline(always)]
    pub fn ucpen(&mut self) -> UCPEN_W {
        UCPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ctl0](index.html) module"]
pub struct UCA0CTL0_SPEC;
impl crate::RegisterSpec for UCA0CTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0ctl0::R](R) reader structure"]
impl crate::Readable for UCA0CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ctl0::W](W) writer structure"]
impl crate::Writable for UCA0CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0CTL0 to value 0"]
impl crate::Resettable for UCA0CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
