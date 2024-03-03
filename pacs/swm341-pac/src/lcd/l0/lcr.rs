#[doc = "Register `LCR` reader"]
pub struct R(crate::R<LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR` writer"]
pub struct W(crate::W<LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_SPEC>;
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
impl From<crate::W<LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALPHA` reader - ALPHA field"]
pub type ALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALPHA` writer - ALPHA field"]
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `CKEN` reader - CKEN field"]
pub type CKEN_R = crate::BitReader<bool>;
#[doc = "Field `CKEN` writer - CKEN field"]
pub type CKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - ALPHA field"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CKEN field"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ALPHA field"]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<0> {
        ALPHA_W::new(self)
    }
    #[doc = "Bit 8 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<8> {
        EN_W::new(self)
    }
    #[doc = "Bit 9 - CKEN field"]
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W<9> {
        CKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](index.html) module"]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr::R](R) reader structure"]
impl crate::Readable for LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr::W](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
