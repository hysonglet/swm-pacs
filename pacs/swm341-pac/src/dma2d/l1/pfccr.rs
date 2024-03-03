#[doc = "Register `PFCCR` reader"]
pub struct R(crate::R<PFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFCCR` writer"]
pub struct W(crate::W<PFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFCCR_SPEC>;
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
impl From<crate::W<PFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFMT` reader - CFMT field"]
pub type CFMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFMT` writer - CFMT field"]
pub type CFMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFCCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AINV` reader - AINV field"]
pub type AINV_R = crate::BitReader<bool>;
#[doc = "Field `AINV` writer - AINV field"]
pub type AINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFCCR_SPEC, bool, O>;
#[doc = "Field `RBSWAP` reader - RBSWAP field"]
pub type RBSWAP_R = crate::BitReader<bool>;
#[doc = "Field `RBSWAP` writer - RBSWAP field"]
pub type RBSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFCCR_SPEC, bool, O>;
#[doc = "Field `ALPHA` reader - ALPHA field"]
pub type ALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALPHA` writer - ALPHA field"]
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFCCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - CFMT field"]
    #[inline(always)]
    pub fn cfmt(&self) -> CFMT_R {
        CFMT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - AINV field"]
    #[inline(always)]
    pub fn ainv(&self) -> AINV_R {
        AINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RBSWAP field"]
    #[inline(always)]
    pub fn rbswap(&self) -> RBSWAP_R {
        RBSWAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 24:31 - ALPHA field"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CFMT field"]
    #[inline(always)]
    pub fn cfmt(&mut self) -> CFMT_W<0> {
        CFMT_W::new(self)
    }
    #[doc = "Bit 3 - AINV field"]
    #[inline(always)]
    pub fn ainv(&mut self) -> AINV_W<3> {
        AINV_W::new(self)
    }
    #[doc = "Bit 4 - RBSWAP field"]
    #[inline(always)]
    pub fn rbswap(&mut self) -> RBSWAP_W<4> {
        RBSWAP_W::new(self)
    }
    #[doc = "Bits 24:31 - ALPHA field"]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<24> {
        ALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFCCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfccr](index.html) module"]
pub struct PFCCR_SPEC;
impl crate::RegisterSpec for PFCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfccr::R](R) reader structure"]
impl crate::Readable for PFCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfccr::W](W) writer structure"]
impl crate::Writable for PFCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFCCR to value 0"]
impl crate::Resettable for PFCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
