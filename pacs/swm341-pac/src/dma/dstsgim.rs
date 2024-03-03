#[doc = "Register `DSTSGIM` reader"]
pub struct R(crate::R<DSTSGIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTSGIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTSGIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTSGIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSTSGIM` writer"]
pub struct W(crate::W<DSTSGIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSTSGIM_SPEC>;
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
impl From<crate::W<DSTSGIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSTSGIM_SPEC>) -> Self {
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
#[doc = "DSTSGIM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstsgim](index.html) module"]
pub struct DSTSGIM_SPEC;
impl crate::RegisterSpec for DSTSGIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dstsgim::R](R) reader structure"]
impl crate::Readable for DSTSGIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dstsgim::W](W) writer structure"]
impl crate::Writable for DSTSGIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSTSGIM to value 0"]
impl crate::Resettable for DSTSGIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
