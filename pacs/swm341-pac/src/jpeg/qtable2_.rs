#[doc = "Register `QTABLE2_[%s]` writer"]
pub struct W(crate::W<QTABLE2__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QTABLE2__SPEC>;
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
impl From<crate::W<QTABLE2__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QTABLE2__SPEC>) -> Self {
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
#[doc = "QTABLE2_ register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qtable2_](index.html) module"]
pub struct QTABLE2__SPEC;
impl crate::RegisterSpec for QTABLE2__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qtable2_::W](W) writer structure"]
impl crate::Writable for QTABLE2__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QTABLE2_[%s]
to value 0"]
impl crate::Resettable for QTABLE2__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
