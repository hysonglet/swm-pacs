#[doc = "Register `LINCR` reader"]
pub struct R(crate::R<LINCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINCR` writer"]
pub struct W(crate::W<LINCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINCR_SPEC>;
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
impl From<crate::W<LINCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRKDETIE` reader - BRKDETIE field"]
pub type BRKDETIE_R = crate::BitReader<bool>;
#[doc = "Field `BRKDETIE` writer - BRKDETIE field"]
pub type BRKDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LINCR_SPEC, bool, O>;
#[doc = "Field `BRKDETIF` reader - BRKDETIF field"]
pub type BRKDETIF_R = crate::BitReader<bool>;
#[doc = "Field `BRKDETIF` writer - BRKDETIF field"]
pub type BRKDETIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LINCR_SPEC, bool, O>;
#[doc = "Field `GENBRKIE` reader - GENBRKIE field"]
pub type GENBRKIE_R = crate::BitReader<bool>;
#[doc = "Field `GENBRKIE` writer - GENBRKIE field"]
pub type GENBRKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LINCR_SPEC, bool, O>;
#[doc = "Field `GENBRKIF` reader - GENBRKIF field"]
pub type GENBRKIF_R = crate::BitReader<bool>;
#[doc = "Field `GENBRKIF` writer - GENBRKIF field"]
pub type GENBRKIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LINCR_SPEC, bool, O>;
#[doc = "Field `GENBRK` reader - GENBRK field"]
pub type GENBRK_R = crate::BitReader<bool>;
#[doc = "Field `GENBRK` writer - GENBRK field"]
pub type GENBRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LINCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BRKDETIE field"]
    #[inline(always)]
    pub fn brkdetie(&self) -> BRKDETIE_R {
        BRKDETIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRKDETIF field"]
    #[inline(always)]
    pub fn brkdetif(&self) -> BRKDETIF_R {
        BRKDETIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GENBRKIE field"]
    #[inline(always)]
    pub fn genbrkie(&self) -> GENBRKIE_R {
        GENBRKIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GENBRKIF field"]
    #[inline(always)]
    pub fn genbrkif(&self) -> GENBRKIF_R {
        GENBRKIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GENBRK field"]
    #[inline(always)]
    pub fn genbrk(&self) -> GENBRK_R {
        GENBRK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRKDETIE field"]
    #[inline(always)]
    pub fn brkdetie(&mut self) -> BRKDETIE_W<0> {
        BRKDETIE_W::new(self)
    }
    #[doc = "Bit 1 - BRKDETIF field"]
    #[inline(always)]
    pub fn brkdetif(&mut self) -> BRKDETIF_W<1> {
        BRKDETIF_W::new(self)
    }
    #[doc = "Bit 2 - GENBRKIE field"]
    #[inline(always)]
    pub fn genbrkie(&mut self) -> GENBRKIE_W<2> {
        GENBRKIE_W::new(self)
    }
    #[doc = "Bit 3 - GENBRKIF field"]
    #[inline(always)]
    pub fn genbrkif(&mut self) -> GENBRKIF_W<3> {
        GENBRKIF_W::new(self)
    }
    #[doc = "Bit 4 - GENBRK field"]
    #[inline(always)]
    pub fn genbrk(&mut self) -> GENBRK_W<4> {
        GENBRK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LINCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lincr](index.html) module"]
pub struct LINCR_SPEC;
impl crate::RegisterSpec for LINCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lincr::R](R) reader structure"]
impl crate::Readable for LINCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lincr::W](W) writer structure"]
impl crate::Writable for LINCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINCR to value 0"]
impl crate::Resettable for LINCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
