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
#[doc = "Field `MASTER` reader - MASTER field"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` writer - MASTER field"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HS` reader - HS field"]
pub type HS_R = crate::BitReader<bool>;
#[doc = "Field `HS` writer - HS field"]
pub type HS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DNF` reader - DNF field"]
pub type DNF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DNF` writer - DNF field"]
pub type DNF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MASTER field"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HS field"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - DNF field"]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - MASTER field"]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W<1> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 2 - HS field"]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W<2> {
        HS_W::new(self)
    }
    #[doc = "Bits 3:6 - DNF field"]
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W<3> {
        DNF_W::new(self)
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
