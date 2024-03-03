#[doc = "Register `REMAIN` reader"]
pub struct R(crate::R<REMAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMAIN` writer"]
pub struct W(crate::W<REMAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMAIN_SPEC>;
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
impl From<crate::W<REMAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMAIN_SPEC>) -> Self {
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
#[doc = "REMAIN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remain](index.html) module"]
pub struct REMAIN_SPEC;
impl crate::RegisterSpec for REMAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remain::R](R) reader structure"]
impl crate::Readable for REMAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remain::W](W) writer structure"]
impl crate::Writable for REMAIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REMAIN to value 0"]
impl crate::Resettable for REMAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
