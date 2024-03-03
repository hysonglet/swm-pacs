#[doc = "Register `IMGSIZ` reader"]
pub struct R(crate::R<IMGSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMGSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMGSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMGSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMGSIZ` writer"]
pub struct W(crate::W<IMGSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMGSIZ_SPEC>;
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
impl From<crate::W<IMGSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMGSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPIX` reader - HPIX field"]
pub type HPIX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPIX` writer - HPIX field"]
pub type HPIX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMGSIZ_SPEC, u16, u16, 10, O>;
#[doc = "Field `VPIX` reader - VPIX field"]
pub type VPIX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VPIX` writer - VPIX field"]
pub type VPIX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMGSIZ_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - HPIX field"]
    #[inline(always)]
    pub fn hpix(&self) -> HPIX_R {
        HPIX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - VPIX field"]
    #[inline(always)]
    pub fn vpix(&self) -> VPIX_R {
        VPIX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - HPIX field"]
    #[inline(always)]
    pub fn hpix(&mut self) -> HPIX_W<0> {
        HPIX_W::new(self)
    }
    #[doc = "Bits 16:25 - VPIX field"]
    #[inline(always)]
    pub fn vpix(&mut self) -> VPIX_W<16> {
        VPIX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMGSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imgsiz](index.html) module"]
pub struct IMGSIZ_SPEC;
impl crate::RegisterSpec for IMGSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imgsiz::R](R) reader structure"]
impl crate::Readable for IMGSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imgsiz::W](W) writer structure"]
impl crate::Writable for IMGSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMGSIZ to value 0"]
impl crate::Resettable for IMGSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
