#[doc = "Register `UCB1BR1` reader"]
pub struct R(crate::R<UCB1BR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1BR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB1BR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB1BR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1BR1` writer"]
pub struct W(crate::W<UCB1BR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1BR1_SPEC>;
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
impl From<crate::W<UCB1BR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB1BR1_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1br1](index.html) module"]
pub struct UCB1BR1_SPEC;
impl crate::RegisterSpec for UCB1BR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb1br1::R](R) reader structure"]
impl crate::Readable for UCB1BR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1br1::W](W) writer structure"]
impl crate::Writable for UCB1BR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1BR1 to value 0"]
impl crate::Resettable for UCB1BR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
