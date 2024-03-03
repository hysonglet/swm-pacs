#[doc = "Register `EWLIM` reader"]
pub struct R(crate::R<EWLIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EWLIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EWLIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EWLIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EWLIM` writer"]
pub struct W(crate::W<EWLIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EWLIM_SPEC>;
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
impl From<crate::W<EWLIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EWLIM_SPEC>) -> Self {
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
#[doc = "EWLIM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ewlim](index.html) module"]
pub struct EWLIM_SPEC;
impl crate::RegisterSpec for EWLIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ewlim::R](R) reader structure"]
impl crate::Readable for EWLIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ewlim::W](W) writer structure"]
impl crate::Writable for EWLIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EWLIM to value 0"]
impl crate::Resettable for EWLIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
