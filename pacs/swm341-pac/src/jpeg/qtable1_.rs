#[doc = "Register `QTABLE1_[%s]` writer"]
pub struct W(crate::W<QTABLE1__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QTABLE1__SPEC>;
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
impl From<crate::W<QTABLE1__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QTABLE1__SPEC>) -> Self {
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
#[doc = "QTABLE1_ register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qtable1_](index.html) module"]
pub struct QTABLE1__SPEC;
impl crate::RegisterSpec for QTABLE1__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qtable1_::W](W) writer structure"]
impl crate::Writable for QTABLE1__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QTABLE1_[%s]
to value 0"]
impl crate::Resettable for QTABLE1__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
