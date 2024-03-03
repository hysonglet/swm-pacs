#[doc = "Register `TXBUF6_[%s]` reader"]
pub struct R(crate::R<TXBUF6__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBUF6__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBUF6__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBUF6__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBUF6_[%s]` writer"]
pub struct W(crate::W<TXBUF6__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBUF6__SPEC>;
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
impl From<crate::W<TXBUF6__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBUF6__SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXBUF6_ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf6_](index.html) module"]
pub struct TXBUF6__SPEC;
impl crate::RegisterSpec for TXBUF6__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbuf6_::R](R) reader structure"]
impl crate::Readable for TXBUF6__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbuf6_::W](W) writer structure"]
impl crate::Writable for TXBUF6__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBUF6_[%s]
to value 0"]
impl crate::Resettable for TXBUF6__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
