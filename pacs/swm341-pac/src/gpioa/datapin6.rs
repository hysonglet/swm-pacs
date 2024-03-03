#[doc = "Register `DATAPIN6` reader"]
pub struct R(crate::R<DATAPIN6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAPIN6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAPIN6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAPIN6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAPIN6` writer"]
pub struct W(crate::W<DATAPIN6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAPIN6_SPEC>;
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
impl From<crate::W<DATAPIN6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAPIN6_SPEC>) -> Self {
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
#[doc = "DATAPIN6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datapin6](index.html) module"]
pub struct DATAPIN6_SPEC;
impl crate::RegisterSpec for DATAPIN6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datapin6::R](R) reader structure"]
impl crate::Readable for DATAPIN6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datapin6::W](W) writer structure"]
impl crate::Writable for DATAPIN6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAPIN6 to value 0"]
impl crate::Resettable for DATAPIN6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
