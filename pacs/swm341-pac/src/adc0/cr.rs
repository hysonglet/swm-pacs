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
#[doc = "Field `AVG` reader - AVG field"]
pub type AVG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVG` writer - AVG field"]
pub type AVG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESET` reader - RESET field"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET field"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMAEN field"]
pub type DMAEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMAEN` writer - DMAEN field"]
pub type DMAEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FFCLR` reader - FFCLR field"]
pub type FFCLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FFCLR` writer - FFCLR field"]
pub type FFCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - AVG field"]
    #[inline(always)]
    pub fn avg(&self) -> AVG_R {
        AVG_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - RESET field"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DMAEN field"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - FFCLR field"]
    #[inline(always)]
    pub fn ffclr(&self) -> FFCLR_R {
        FFCLR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - AVG field"]
    #[inline(always)]
    pub fn avg(&mut self) -> AVG_W<1> {
        AVG_W::new(self)
    }
    #[doc = "Bit 3 - RESET field"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<3> {
        RESET_W::new(self)
    }
    #[doc = "Bits 4:7 - DMAEN field"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<4> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 8:11 - FFCLR field"]
    #[inline(always)]
    pub fn ffclr(&mut self) -> FFCLR_W<8> {
        FFCLR_W::new(self)
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
