#[doc = "Register `PREDIV` reader"]
pub struct R(crate::R<PREDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREDIV` writer"]
pub struct W(crate::W<PREDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREDIV_SPEC>;
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
impl From<crate::W<PREDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREDIV_SPEC>) -> Self {
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
#[doc = "PREDIV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prediv](index.html) module"]
pub struct PREDIV_SPEC;
impl crate::RegisterSpec for PREDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prediv::R](R) reader structure"]
impl crate::Readable for PREDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prediv::W](W) writer structure"]
impl crate::Writable for PREDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREDIV to value 0"]
impl crate::Resettable for PREDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
