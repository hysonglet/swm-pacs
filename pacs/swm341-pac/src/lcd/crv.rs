#[doc = "Register `CRV` reader"]
pub struct R(crate::R<CRV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRV` writer"]
pub struct W(crate::W<CRV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRV_SPEC>;
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
impl From<crate::W<CRV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSW` reader - VSW field"]
pub type VSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSW` writer - VSW field"]
pub type VSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRV_SPEC, u8, u8, 8, O>;
#[doc = "Field `VBP` reader - VBP field"]
pub type VBP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBP` writer - VBP field"]
pub type VBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRV_SPEC, u8, u8, 8, O>;
#[doc = "Field `PIX` reader - PIX field"]
pub type PIX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PIX` writer - PIX field"]
pub type PIX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRV_SPEC, u16, u16, 10, O>;
#[doc = "Field `VFP` reader - VFP field"]
pub type VFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VFP` writer - VFP field"]
pub type VFP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRV_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:7 - VSW field"]
    #[inline(always)]
    pub fn vsw(&self) -> VSW_R {
        VSW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VBP field"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - PIX field"]
    #[inline(always)]
    pub fn pix(&self) -> PIX_R {
        PIX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - VFP field"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VSW field"]
    #[inline(always)]
    pub fn vsw(&mut self) -> VSW_W<0> {
        VSW_W::new(self)
    }
    #[doc = "Bits 8:15 - VBP field"]
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W<8> {
        VBP_W::new(self)
    }
    #[doc = "Bits 16:25 - PIX field"]
    #[inline(always)]
    pub fn pix(&mut self) -> PIX_W<16> {
        PIX_W::new(self)
    }
    #[doc = "Bits 26:31 - VFP field"]
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W<26> {
        VFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crv](index.html) module"]
pub struct CRV_SPEC;
impl crate::RegisterSpec for CRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crv::R](R) reader structure"]
impl crate::Readable for CRV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crv::W](W) writer structure"]
impl crate::Writable for CRV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRV to value 0"]
impl crate::Resettable for CRV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
