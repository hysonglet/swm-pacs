#[doc = "Register `DATAPIN5` reader"]
pub struct R(crate::R<DATAPIN5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAPIN5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAPIN5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAPIN5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAPIN5` writer"]
pub struct W(crate::W<DATAPIN5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAPIN5_SPEC>;
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
impl From<crate::W<DATAPIN5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAPIN5_SPEC>) -> Self {
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
#[doc = "DATAPIN5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datapin5](index.html) module"]
pub struct DATAPIN5_SPEC;
impl crate::RegisterSpec for DATAPIN5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datapin5::R](R) reader structure"]
impl crate::Readable for DATAPIN5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datapin5::W](W) writer structure"]
impl crate::Writable for DATAPIN5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAPIN5 to value 0"]
impl crate::Resettable for DATAPIN5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
