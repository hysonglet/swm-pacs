#[doc = "Register `DACCR` reader"]
pub struct R(crate::R<DACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACCR` writer"]
pub struct W(crate::W<DACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACCR_SPEC>;
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
impl From<crate::W<DACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VRADJ` reader - VRADJ field"]
pub type VRADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VRADJ` writer - VRADJ field"]
pub type VRADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - VRADJ field"]
    #[inline(always)]
    pub fn vradj(&self) -> VRADJ_R {
        VRADJ_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - VRADJ field"]
    #[inline(always)]
    pub fn vradj(&mut self) -> VRADJ_W<0> {
        VRADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DACCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daccr](index.html) module"]
pub struct DACCR_SPEC;
impl crate::RegisterSpec for DACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daccr::R](R) reader structure"]
impl crate::Readable for DACCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daccr::W](W) writer structure"]
impl crate::Writable for DACCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACCR to value 0"]
impl crate::Resettable for DACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
