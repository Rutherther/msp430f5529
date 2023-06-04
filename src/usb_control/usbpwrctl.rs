#[doc = "Register `USBPWRCTL` reader"]
pub struct R(crate::R<USBPWRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPWRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPWRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPWRCTL` writer"]
pub struct W(crate::W<USBPWRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPWRCTL_SPEC>;
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
impl From<crate::W<USBPWRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPWRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VUOVLIFG` reader - USB - VUSB Overload Interrupt Flag"]
pub type VUOVLIFG_R = crate::BitReader<bool>;
#[doc = "Field `VUOVLIFG` writer - USB - VUSB Overload Interrupt Flag"]
pub type VUOVLIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `VBONIFG` reader - USB - VBUS \"Coming ON\" Interrupt Flag"]
pub type VBONIFG_R = crate::BitReader<bool>;
#[doc = "Field `VBONIFG` writer - USB - VBUS \"Coming ON\" Interrupt Flag"]
pub type VBONIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `VBOFFIFG` reader - USB - VBUS \"Going OFF\" Interrupt Flag"]
pub type VBOFFIFG_R = crate::BitReader<bool>;
#[doc = "Field `VBOFFIFG` writer - USB - VBUS \"Going OFF\" Interrupt Flag"]
pub type VBOFFIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `USBBGVBV` reader - USB - USB Bandgap and VBUS valid"]
pub type USBBGVBV_R = crate::BitReader<bool>;
#[doc = "Field `USBBGVBV` writer - USB - USB Bandgap and VBUS valid"]
pub type USBBGVBV_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `USBDETEN` reader - USB - VBUS on/off events enable"]
pub type USBDETEN_R = crate::BitReader<bool>;
#[doc = "Field `USBDETEN` writer - USB - VBUS on/off events enable"]
pub type USBDETEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `OVLAOFF` reader - USB - LDO overload auto off enable"]
pub type OVLAOFF_R = crate::BitReader<bool>;
#[doc = "Field `OVLAOFF` writer - USB - LDO overload auto off enable"]
pub type OVLAOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `SLDOAON` reader - USB - Secondary LDO auto on enable"]
pub type SLDOAON_R = crate::BitReader<bool>;
#[doc = "Field `SLDOAON` writer - USB - Secondary LDO auto on enable"]
pub type SLDOAON_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `VUOVLIE` reader - USB - Overload indication Interrupt Enable"]
pub type VUOVLIE_R = crate::BitReader<bool>;
#[doc = "Field `VUOVLIE` writer - USB - Overload indication Interrupt Enable"]
pub type VUOVLIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `VBONIE` reader - USB - VBUS \"Coming ON\" Interrupt Enable"]
pub type VBONIE_R = crate::BitReader<bool>;
#[doc = "Field `VBONIE` writer - USB - VBUS \"Coming ON\" Interrupt Enable"]
pub type VBONIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `VBOFFIE` reader - USB - VBUS \"Going OFF\" Interrupt Enable"]
pub type VBOFFIE_R = crate::BitReader<bool>;
#[doc = "Field `VBOFFIE` writer - USB - VBUS \"Going OFF\" Interrupt Enable"]
pub type VBOFFIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `VUSBEN` reader - USB - LDO Enable (3.3V)"]
pub type VUSBEN_R = crate::BitReader<bool>;
#[doc = "Field `VUSBEN` writer - USB - LDO Enable (3.3V)"]
pub type VUSBEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
#[doc = "Field `SLDOEN` reader - USB - Secondary LDO Enable (1.8V)"]
pub type SLDOEN_R = crate::BitReader<bool>;
#[doc = "Field `SLDOEN` writer - USB - Secondary LDO Enable (1.8V)"]
pub type SLDOEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, USBPWRCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB - VUSB Overload Interrupt Flag"]
    #[inline(always)]
    pub fn vuovlifg(&self) -> VUOVLIFG_R {
        VUOVLIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB - VBUS \"Coming ON\" Interrupt Flag"]
    #[inline(always)]
    pub fn vbonifg(&self) -> VBONIFG_R {
        VBONIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB - VBUS \"Going OFF\" Interrupt Flag"]
    #[inline(always)]
    pub fn vboffifg(&self) -> VBOFFIFG_R {
        VBOFFIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB - USB Bandgap and VBUS valid"]
    #[inline(always)]
    pub fn usbbgvbv(&self) -> USBBGVBV_R {
        USBBGVBV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB - VBUS on/off events enable"]
    #[inline(always)]
    pub fn usbdeten(&self) -> USBDETEN_R {
        USBDETEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB - LDO overload auto off enable"]
    #[inline(always)]
    pub fn ovlaoff(&self) -> OVLAOFF_R {
        OVLAOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB - Secondary LDO auto on enable"]
    #[inline(always)]
    pub fn sldoaon(&self) -> SLDOAON_R {
        SLDOAON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - USB - Overload indication Interrupt Enable"]
    #[inline(always)]
    pub fn vuovlie(&self) -> VUOVLIE_R {
        VUOVLIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB - VBUS \"Coming ON\" Interrupt Enable"]
    #[inline(always)]
    pub fn vbonie(&self) -> VBONIE_R {
        VBONIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB - VBUS \"Going OFF\" Interrupt Enable"]
    #[inline(always)]
    pub fn vboffie(&self) -> VBOFFIE_R {
        VBOFFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB - LDO Enable (3.3V)"]
    #[inline(always)]
    pub fn vusben(&self) -> VUSBEN_R {
        VUSBEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB - Secondary LDO Enable (1.8V)"]
    #[inline(always)]
    pub fn sldoen(&self) -> SLDOEN_R {
        SLDOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - VUSB Overload Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vuovlifg(&mut self) -> VUOVLIFG_W<0> {
        VUOVLIFG_W::new(self)
    }
    #[doc = "Bit 1 - USB - VBUS \"Coming ON\" Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vbonifg(&mut self) -> VBONIFG_W<1> {
        VBONIFG_W::new(self)
    }
    #[doc = "Bit 2 - USB - VBUS \"Going OFF\" Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vboffifg(&mut self) -> VBOFFIFG_W<2> {
        VBOFFIFG_W::new(self)
    }
    #[doc = "Bit 3 - USB - USB Bandgap and VBUS valid"]
    #[inline(always)]
    #[must_use]
    pub fn usbbgvbv(&mut self) -> USBBGVBV_W<3> {
        USBBGVBV_W::new(self)
    }
    #[doc = "Bit 4 - USB - VBUS on/off events enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbdeten(&mut self) -> USBDETEN_W<4> {
        USBDETEN_W::new(self)
    }
    #[doc = "Bit 5 - USB - LDO overload auto off enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovlaoff(&mut self) -> OVLAOFF_W<5> {
        OVLAOFF_W::new(self)
    }
    #[doc = "Bit 6 - USB - Secondary LDO auto on enable"]
    #[inline(always)]
    #[must_use]
    pub fn sldoaon(&mut self) -> SLDOAON_W<6> {
        SLDOAON_W::new(self)
    }
    #[doc = "Bit 8 - USB - Overload indication Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vuovlie(&mut self) -> VUOVLIE_W<8> {
        VUOVLIE_W::new(self)
    }
    #[doc = "Bit 9 - USB - VBUS \"Coming ON\" Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbonie(&mut self) -> VBONIE_W<9> {
        VBONIE_W::new(self)
    }
    #[doc = "Bit 10 - USB - VBUS \"Going OFF\" Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vboffie(&mut self) -> VBOFFIE_W<10> {
        VBOFFIE_W::new(self)
    }
    #[doc = "Bit 11 - USB - LDO Enable (3.3V)"]
    #[inline(always)]
    #[must_use]
    pub fn vusben(&mut self) -> VUSBEN_W<11> {
        VUSBEN_W::new(self)
    }
    #[doc = "Bit 12 - USB - Secondary LDO Enable (1.8V)"]
    #[inline(always)]
    #[must_use]
    pub fn sldoen(&mut self) -> SLDOEN_W<12> {
        SLDOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpwrctl](index.html) module"]
pub struct USBPWRCTL_SPEC;
impl crate::RegisterSpec for USBPWRCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbpwrctl::R](R) reader structure"]
impl crate::Readable for USBPWRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpwrctl::W](W) writer structure"]
impl crate::Writable for USBPWRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPWRCTL to value 0"]
impl crate::Resettable for USBPWRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
