#[doc = "Register `UCB1STAT` reader"]
pub struct R(crate::R<UCB1STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB1STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB1STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1STAT` writer"]
pub struct W(crate::W<UCB1STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1STAT_SPEC>;
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
impl From<crate::W<UCB1STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB1STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBBUSY` reader - Bus Busy Flag"]
pub type UCBBUSY_R = crate::BitReader<bool>;
#[doc = "Field `UCBBUSY` writer - Bus Busy Flag"]
pub type UCBBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB1STAT_SPEC, bool, O>;
#[doc = "Field `UCGC` reader - General Call address received Flag"]
pub type UCGC_R = crate::BitReader<bool>;
#[doc = "Field `UCGC` writer - General Call address received Flag"]
pub type UCGC_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB1STAT_SPEC, bool, O>;
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UCSCLLOW_R = crate::BitReader<bool>;
#[doc = "Field `UCSCLLOW` writer - SCL low"]
pub type UCSCLLOW_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB1STAT_SPEC, bool, O>;
#[doc = "Field `UCLISTEN` reader - USCI Listen mode"]
pub type UCLISTEN_R = crate::BitReader<bool>;
#[doc = "Field `UCLISTEN` writer - USCI Listen mode"]
pub type UCLISTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, UCB1STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucbbusy(&mut self) -> UCBBUSY_W<4> {
        UCBBUSY_W::new(self)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucgc(&mut self) -> UCGC_W<5> {
        UCGC_W::new(self)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    #[must_use]
    pub fn ucscllow(&mut self) -> UCSCLLOW_W<6> {
        UCSCLLOW_W::new(self)
    }
    #[doc = "Bit 7 - USCI Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn uclisten(&mut self) -> UCLISTEN_W<7> {
        UCLISTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1stat](index.html) module"]
pub struct UCB1STAT_SPEC;
impl crate::RegisterSpec for UCB1STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb1stat::R](R) reader structure"]
impl crate::Readable for UCB1STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1stat::W](W) writer structure"]
impl crate::Writable for UCB1STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB1STAT to value 0"]
impl crate::Resettable for UCB1STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
