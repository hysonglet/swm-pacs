#[doc = "Register `CACHE` reader"]
pub struct R(crate::R<CACHE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE` writer"]
pub struct W(crate::W<CACHE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_SPEC>;
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
impl From<crate::W<CACHE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROGEN` reader - PROGEN field"]
pub type PROGEN_R = crate::BitReader<bool>;
#[doc = "Field `PROGEN` writer - PROGEN field"]
pub type PROGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_SPEC, bool, O>;
#[doc = "Field `CEN` reader - CEN field"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - CEN field"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_SPEC, bool, O>;
#[doc = "Field `CPEN` reader - CPEN field"]
pub type CPEN_R = crate::BitReader<bool>;
#[doc = "Field `CPEN` writer - CPEN field"]
pub type CPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_SPEC, bool, O>;
#[doc = "Field `CCLR` reader - CCLR field"]
pub type CCLR_R = crate::BitReader<bool>;
#[doc = "Field `CCLR` writer - CCLR field"]
pub type CCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PROGEN field"]
    #[inline(always)]
    pub fn progen(&self) -> PROGEN_R {
        PROGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - CEN field"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPEN field"]
    #[inline(always)]
    pub fn cpen(&self) -> CPEN_R {
        CPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CCLR field"]
    #[inline(always)]
    pub fn cclr(&self) -> CCLR_R {
        CCLR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PROGEN field"]
    #[inline(always)]
    pub fn progen(&mut self) -> PROGEN_W<0> {
        PROGEN_W::new(self)
    }
    #[doc = "Bit 16 - CEN field"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<16> {
        CEN_W::new(self)
    }
    #[doc = "Bit 17 - CPEN field"]
    #[inline(always)]
    pub fn cpen(&mut self) -> CPEN_W<17> {
        CPEN_W::new(self)
    }
    #[doc = "Bit 18 - CCLR field"]
    #[inline(always)]
    pub fn cclr(&mut self) -> CCLR_W<18> {
        CCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CACHE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache](index.html) module"]
pub struct CACHE_SPEC;
impl crate::RegisterSpec for CACHE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache::R](R) reader structure"]
impl crate::Readable for CACHE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache::W](W) writer structure"]
impl crate::Writable for CACHE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE to value 0"]
impl crate::Resettable for CACHE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
