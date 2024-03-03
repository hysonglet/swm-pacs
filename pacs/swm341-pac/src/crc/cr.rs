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
#[doc = "Field `IREV` reader - IREV field"]
pub type IREV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREV` writer - IREV field"]
pub type IREV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `INOT` reader - INOT field"]
pub type INOT_R = crate::BitReader<bool>;
#[doc = "Field `INOT` writer - INOT field"]
pub type INOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OREV` reader - OREV field"]
pub type OREV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OREV` writer - OREV field"]
pub type OREV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ONOT` reader - ONOT field"]
pub type ONOT_R = crate::BitReader<bool>;
#[doc = "Field `ONOT` writer - ONOT field"]
pub type ONOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `POLY` reader - POLY field"]
pub type POLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POLY` writer - POLY field"]
pub type POLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IBIT` reader - IBIT field"]
pub type IBIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIT` writer - IBIT field"]
pub type IBIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - IREV field"]
    #[inline(always)]
    pub fn irev(&self) -> IREV_R {
        IREV_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - INOT field"]
    #[inline(always)]
    pub fn inot(&self) -> INOT_R {
        INOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - OREV field"]
    #[inline(always)]
    pub fn orev(&self) -> OREV_R {
        OREV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ONOT field"]
    #[inline(always)]
    pub fn onot(&self) -> ONOT_R {
        ONOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - POLY field"]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - IBIT field"]
    #[inline(always)]
    pub fn ibit(&self) -> IBIT_R {
        IBIT_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - IREV field"]
    #[inline(always)]
    pub fn irev(&mut self) -> IREV_W<1> {
        IREV_W::new(self)
    }
    #[doc = "Bit 3 - INOT field"]
    #[inline(always)]
    pub fn inot(&mut self) -> INOT_W<3> {
        INOT_W::new(self)
    }
    #[doc = "Bits 4:5 - OREV field"]
    #[inline(always)]
    pub fn orev(&mut self) -> OREV_W<4> {
        OREV_W::new(self)
    }
    #[doc = "Bit 6 - ONOT field"]
    #[inline(always)]
    pub fn onot(&mut self) -> ONOT_W<6> {
        ONOT_W::new(self)
    }
    #[doc = "Bits 7:8 - POLY field"]
    #[inline(always)]
    pub fn poly(&mut self) -> POLY_W<7> {
        POLY_W::new(self)
    }
    #[doc = "Bits 9:10 - IBIT field"]
    #[inline(always)]
    pub fn ibit(&mut self) -> IBIT_W<9> {
        IBIT_W::new(self)
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
