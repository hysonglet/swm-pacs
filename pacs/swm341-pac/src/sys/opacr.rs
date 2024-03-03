#[doc = "Register `OPACR` reader"]
pub struct R(crate::R<OPACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR` writer"]
pub struct W(crate::W<OPACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR_SPEC>;
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
impl From<crate::W<OPACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA0ON` reader - OPA0ON field"]
pub type OPA0ON_R = crate::BitReader<bool>;
#[doc = "Field `OPA0ON` writer - OPA0ON field"]
pub type OPA0ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPACR_SPEC, bool, O>;
#[doc = "Field `OPA1ON` reader - OPA1ON field"]
pub type OPA1ON_R = crate::BitReader<bool>;
#[doc = "Field `OPA1ON` writer - OPA1ON field"]
pub type OPA1ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPACR_SPEC, bool, O>;
#[doc = "Field `OPA2ON` reader - OPA2ON field"]
pub type OPA2ON_R = crate::BitReader<bool>;
#[doc = "Field `OPA2ON` writer - OPA2ON field"]
pub type OPA2ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPACR_SPEC, bool, O>;
#[doc = "Field `OPA3ON` reader - OPA3ON field"]
pub type OPA3ON_R = crate::BitReader<bool>;
#[doc = "Field `OPA3ON` writer - OPA3ON field"]
pub type OPA3ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPACR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OPA0ON field"]
    #[inline(always)]
    pub fn opa0on(&self) -> OPA0ON_R {
        OPA0ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPA1ON field"]
    #[inline(always)]
    pub fn opa1on(&self) -> OPA1ON_R {
        OPA1ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPA2ON field"]
    #[inline(always)]
    pub fn opa2on(&self) -> OPA2ON_R {
        OPA2ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPA3ON field"]
    #[inline(always)]
    pub fn opa3on(&self) -> OPA3ON_R {
        OPA3ON_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA0ON field"]
    #[inline(always)]
    pub fn opa0on(&mut self) -> OPA0ON_W<0> {
        OPA0ON_W::new(self)
    }
    #[doc = "Bit 1 - OPA1ON field"]
    #[inline(always)]
    pub fn opa1on(&mut self) -> OPA1ON_W<1> {
        OPA1ON_W::new(self)
    }
    #[doc = "Bit 2 - OPA2ON field"]
    #[inline(always)]
    pub fn opa2on(&mut self) -> OPA2ON_W<2> {
        OPA2ON_W::new(self)
    }
    #[doc = "Bit 3 - OPA3ON field"]
    #[inline(always)]
    pub fn opa3on(&mut self) -> OPA3ON_W<3> {
        OPA3ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPACR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr](index.html) module"]
pub struct OPACR_SPEC;
impl crate::RegisterSpec for OPACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr::R](R) reader structure"]
impl crate::Readable for OPACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr::W](W) writer structure"]
impl crate::Writable for OPACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPACR to value 0"]
impl crate::Resettable for OPACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
