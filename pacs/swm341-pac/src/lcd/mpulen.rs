#[doc = "Register `MPULEN` reader"]
pub struct R(crate::R<MPULEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPULEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPULEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPULEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPULEN` writer"]
pub struct W(crate::W<MPULEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPULEN_SPEC>;
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
impl From<crate::W<MPULEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPULEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VPIX` reader - VPIX field"]
pub type VPIX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VPIX` writer - VPIX field"]
pub type VPIX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPULEN_SPEC, u16, u16, 10, O>;
#[doc = "Field `HPIX` reader - HPIX field"]
pub type HPIX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPIX` writer - HPIX field"]
pub type HPIX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPULEN_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - VPIX field"]
    #[inline(always)]
    pub fn vpix(&self) -> VPIX_R {
        VPIX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - HPIX field"]
    #[inline(always)]
    pub fn hpix(&self) -> HPIX_R {
        HPIX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VPIX field"]
    #[inline(always)]
    pub fn vpix(&mut self) -> VPIX_W<0> {
        VPIX_W::new(self)
    }
    #[doc = "Bits 16:25 - HPIX field"]
    #[inline(always)]
    pub fn hpix(&mut self) -> HPIX_W<16> {
        HPIX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPULEN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpulen](index.html) module"]
pub struct MPULEN_SPEC;
impl crate::RegisterSpec for MPULEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpulen::R](R) reader structure"]
impl crate::Readable for MPULEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpulen::W](W) writer structure"]
impl crate::Writable for MPULEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPULEN to value 0"]
impl crate::Resettable for MPULEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
