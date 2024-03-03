#[doc = "Register `CRH` reader"]
pub struct R(crate::R<CRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRH` writer"]
pub struct W(crate::W<CRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRH_SPEC>;
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
impl From<crate::W<CRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSW` reader - HSW field"]
pub type HSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSW` writer - HSW field"]
pub type HSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRH_SPEC, u8, u8, 8, O>;
#[doc = "Field `HBP` reader - HBP field"]
pub type HBP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HBP` writer - HBP field"]
pub type HBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRH_SPEC, u8, u8, 8, O>;
#[doc = "Field `PIX` reader - PIX field"]
pub type PIX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PIX` writer - PIX field"]
pub type PIX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRH_SPEC, u16, u16, 10, O>;
#[doc = "Field `HFP` reader - HFP field"]
pub type HFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HFP` writer - HFP field"]
pub type HFP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRH_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:7 - HSW field"]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - HBP field"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - PIX field"]
    #[inline(always)]
    pub fn pix(&self) -> PIX_R {
        PIX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - HFP field"]
    #[inline(always)]
    pub fn hfp(&self) -> HFP_R {
        HFP_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HSW field"]
    #[inline(always)]
    pub fn hsw(&mut self) -> HSW_W<0> {
        HSW_W::new(self)
    }
    #[doc = "Bits 8:15 - HBP field"]
    #[inline(always)]
    pub fn hbp(&mut self) -> HBP_W<8> {
        HBP_W::new(self)
    }
    #[doc = "Bits 16:25 - PIX field"]
    #[inline(always)]
    pub fn pix(&mut self) -> PIX_W<16> {
        PIX_W::new(self)
    }
    #[doc = "Bits 26:31 - HFP field"]
    #[inline(always)]
    pub fn hfp(&mut self) -> HFP_W<26> {
        HFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRH register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crh](index.html) module"]
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crh::R](R) reader structure"]
impl crate::Readable for CRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crh::W](W) writer structure"]
impl crate::Writable for CRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRH to value 0"]
impl crate::Resettable for CRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
