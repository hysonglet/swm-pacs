#[doc = "Register `PRNGCR` reader"]
pub struct R(crate::R<PRNGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRNGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRNGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRNGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRNGCR` writer"]
pub struct W(crate::W<PRNGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRNGCR_SPEC>;
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
impl From<crate::W<PRNGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRNGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` reader - CLR field"]
pub type CLR_R = crate::BitReader<bool>;
#[doc = "Field `CLR` writer - CLR field"]
pub type CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRNGCR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - MODE field"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - MODE field"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRNGCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RDY` reader - RDY field"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - RDY field"]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRNGCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CLR field"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MODE field"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - RDY field"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLR field"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Bits 1:2 - MODE field"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 8 - RDY field"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W<8> {
        RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRNGCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prngcr](index.html) module"]
pub struct PRNGCR_SPEC;
impl crate::RegisterSpec for PRNGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prngcr::R](R) reader structure"]
impl crate::Readable for PRNGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prngcr::W](W) writer structure"]
impl crate::Writable for PRNGCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRNGCR to value 0"]
impl crate::Resettable for PRNGCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
