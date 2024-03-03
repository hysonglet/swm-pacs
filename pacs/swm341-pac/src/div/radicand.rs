#[doc = "Register `RADICAND` reader"]
pub struct R(crate::R<RADICAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RADICAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RADICAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RADICAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RADICAND` writer"]
pub struct W(crate::W<RADICAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RADICAND_SPEC>;
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
impl From<crate::W<RADICAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RADICAND_SPEC>) -> Self {
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
#[doc = "RADICAND register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radicand](index.html) module"]
pub struct RADICAND_SPEC;
impl crate::RegisterSpec for RADICAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [radicand::R](R) reader structure"]
impl crate::Readable for RADICAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [radicand::W](W) writer structure"]
impl crate::Writable for RADICAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RADICAND to value 0"]
impl crate::Resettable for RADICAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
