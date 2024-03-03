#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENLDO` reader - ENLDO field"]
pub type ENLDO_R = crate::BitReader<bool>;
#[doc = "Field `ENLDO` writer - ENLDO field"]
pub type ENLDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `BITS` reader - BITS field"]
pub type BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITS` writer - BITS field"]
pub type BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 2 - ENLDO field"]
    #[inline(always)]
    pub fn enldo(&self) -> ENLDO_R {
        ENLDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 6:7 - BITS field"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ENLDO field"]
    #[inline(always)]
    pub fn enldo(&mut self) -> ENLDO_W<2> {
        ENLDO_W::new(self)
    }
    #[doc = "Bits 6:7 - BITS field"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BITS_W<6> {
        BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
