#[doc = "Register `AFE` reader"]
pub struct R(crate::R<AFE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFE` writer"]
pub struct W(crate::W<AFE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFE_SPEC>;
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
impl From<crate::W<AFE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFE_SPEC>) -> Self {
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
#[doc = "AFE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afe](index.html) module"]
pub struct AFE_SPEC;
impl crate::RegisterSpec for AFE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afe::R](R) reader structure"]
impl crate::Readable for AFE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afe::W](W) writer structure"]
impl crate::Writable for AFE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFE to value 0"]
impl crate::Resettable for AFE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
