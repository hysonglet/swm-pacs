#[doc = "Register `TEMPCR` reader"]
pub struct R(crate::R<TEMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPCR` writer"]
pub struct W(crate::W<TEMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPCR_SPEC>;
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
impl From<crate::W<TEMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEMPCR_SPEC, bool, O>;
#[doc = "Field `TRIM` reader - TRIM field"]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM` writer - TRIM field"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `AD0CH7` reader - AD0CH7 field"]
pub type AD0CH7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD0CH7` writer - AD0CH7 field"]
pub type AD0CH7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:9 - TRIM field"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - AD0CH7 field"]
    #[inline(always)]
    pub fn ad0ch7(&self) -> AD0CH7_R {
        AD0CH7_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 4:9 - TRIM field"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<4> {
        TRIM_W::new(self)
    }
    #[doc = "Bits 16:17 - AD0CH7 field"]
    #[inline(always)]
    pub fn ad0ch7(&mut self) -> AD0CH7_W<16> {
        AD0CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TEMPCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempcr](index.html) module"]
pub struct TEMPCR_SPEC;
impl crate::RegisterSpec for TEMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempcr::R](R) reader structure"]
impl crate::Readable for TEMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempcr::W](W) writer structure"]
impl crate::Writable for TEMPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEMPCR to value 0"]
impl crate::Resettable for TEMPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
