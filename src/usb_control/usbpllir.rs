#[doc = "Register `USBPLLIR` reader"]
pub struct R(crate::R<USBPLLIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLIR` writer"]
pub struct W(crate::W<USBPLLIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLIR_SPEC>;
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
impl From<crate::W<USBPLLIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBOOLIFG` reader - USB - PLL out of lock Interrupt Flag"]
pub type USBOOLIFG_R = crate::BitReader<bool>;
#[doc = "Field `USBOOLIFG` writer - USB - PLL out of lock Interrupt Flag"]
pub type USBOOLIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLIR_SPEC, bool, O>;
#[doc = "Field `USBLOSIFG` reader - USB - PLL loss of signal Interrupt Flag"]
pub type USBLOSIFG_R = crate::BitReader<bool>;
#[doc = "Field `USBLOSIFG` writer - USB - PLL loss of signal Interrupt Flag"]
pub type USBLOSIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLIR_SPEC, bool, O>;
#[doc = "Field `USBOORIFG` reader - USB - PLL out of range Interrupt Flag"]
pub type USBOORIFG_R = crate::BitReader<bool>;
#[doc = "Field `USBOORIFG` writer - USB - PLL out of range Interrupt Flag"]
pub type USBOORIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLIR_SPEC, bool, O>;
#[doc = "Field `USBOOLIE` reader - USB - PLL out of lock Interrupt enable"]
pub type USBOOLIE_R = crate::BitReader<bool>;
#[doc = "Field `USBOOLIE` writer - USB - PLL out of lock Interrupt enable"]
pub type USBOOLIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLIR_SPEC, bool, O>;
#[doc = "Field `USBLOSIE` reader - USB - PLL loss of signal Interrupt enable"]
pub type USBLOSIE_R = crate::BitReader<bool>;
#[doc = "Field `USBLOSIE` writer - USB - PLL loss of signal Interrupt enable"]
pub type USBLOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLIR_SPEC, bool, O>;
#[doc = "Field `USBOORIE` reader - USB - PLL out of range Interrupt enable"]
pub type USBOORIE_R = crate::BitReader<bool>;
#[doc = "Field `USBOORIE` writer - USB - PLL out of range Interrupt enable"]
pub type USBOORIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPLLIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB - PLL out of lock Interrupt Flag"]
    #[inline(always)]
    pub fn usboolifg(&self) -> USBOOLIFG_R {
        USBOOLIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - PLL loss of signal Interrupt Flag"]
    #[inline(always)]
    pub fn usblosifg(&self) -> USBLOSIFG_R {
        USBLOSIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - PLL out of range Interrupt Flag"]
    #[inline(always)]
    pub fn usboorifg(&self) -> USBOORIFG_R {
        USBOORIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - PLL out of lock Interrupt enable"]
    #[inline(always)]
    pub fn usboolie(&self) -> USBOOLIE_R {
        USBOOLIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB - PLL loss of signal Interrupt enable"]
    #[inline(always)]
    pub fn usblosie(&self) -> USBLOSIE_R {
        USBLOSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB - PLL out of range Interrupt enable"]
    #[inline(always)]
    pub fn usboorie(&self) -> USBOORIE_R {
        USBOORIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - PLL out of lock Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usboolifg(&mut self) -> USBOOLIFG_W<0> {
        USBOOLIFG_W::new(self)
    }
    #[doc = "Bit 1 - USB - PLL loss of signal Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usblosifg(&mut self) -> USBLOSIFG_W<1> {
        USBLOSIFG_W::new(self)
    }
    #[doc = "Bit 2 - USB - PLL out of range Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usboorifg(&mut self) -> USBOORIFG_W<2> {
        USBOORIFG_W::new(self)
    }
    #[doc = "Bit 8 - USB - PLL out of lock Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usboolie(&mut self) -> USBOOLIE_W<8> {
        USBOOLIE_W::new(self)
    }
    #[doc = "Bit 9 - USB - PLL loss of signal Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usblosie(&mut self) -> USBLOSIE_W<9> {
        USBLOSIE_W::new(self)
    }
    #[doc = "Bit 10 - USB - PLL out of range Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usboorie(&mut self) -> USBOORIE_W<10> {
        USBOORIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL Interrupt control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllir](index.html) module"]
pub struct USBPLLIR_SPEC;
impl crate::RegisterSpec for USBPLLIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbpllir::R](R) reader structure"]
impl crate::Readable for USBPLLIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpllir::W](W) writer structure"]
impl crate::Writable for USBPLLIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPLLIR to value 0"]
impl crate::Resettable for USBPLLIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
