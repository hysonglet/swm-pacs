#[doc = "Register `HALL3V` reader"]
pub struct R(crate::R<HALL3V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALL3V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HALL3V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HALL3V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HALL3V` writer"]
pub struct W(crate::W<HALL3V_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HALL3V_SPEC>;
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
impl From<crate::W<HALL3V_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HALL3V_SPEC>) -> Self {
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
#[doc = "HALL3V register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hall3v](index.html) module"]
pub struct HALL3V_SPEC;
impl crate::RegisterSpec for HALL3V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hall3v::R](R) reader structure"]
impl crate::Readable for HALL3V_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hall3v::W](W) writer structure"]
impl crate::Writable for HALL3V_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HALL3V to value 0"]
impl crate::Resettable for HALL3V_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
