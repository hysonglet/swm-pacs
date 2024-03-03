#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSTEN` reader - RSTEN field"]
pub type RSTEN_R = crate::BitReader<bool>;
#[doc = "Field `RSTEN` writer - RSTEN field"]
pub type RSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `INTEN` reader - INTEN field"]
pub type INTEN_R = crate::BitReader<bool>;
#[doc = "Field `INTEN` writer - INTEN field"]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WINEN` reader - WINEN field"]
pub type WINEN_R = crate::BitReader<bool>;
#[doc = "Field `WINEN` writer - WINEN field"]
pub type WINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CKDIV` reader - CKDIV field"]
pub type CKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKDIV` writer - CKDIV field"]
pub type CKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSTEN field"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INTEN field"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WINEN field"]
    #[inline(always)]
    pub fn winen(&self) -> WINEN_R {
        WINEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CKDIV field"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - RSTEN field"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RSTEN_W<1> {
        RSTEN_W::new(self)
    }
    #[doc = "Bit 2 - INTEN field"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W<2> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 3 - WINEN field"]
    #[inline(always)]
    pub fn winen(&mut self) -> WINEN_W<3> {
        WINEN_W::new(self)
    }
    #[doc = "Bits 8:11 - CKDIV field"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W<8> {
        CKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
