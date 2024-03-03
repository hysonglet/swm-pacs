#[doc = "Register `OCR` reader"]
pub struct R(crate::R<OCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCR` writer"]
pub struct W(crate::W<OCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCR_SPEC>;
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
impl From<crate::W<OCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDLEA` reader - IDLEA field"]
pub type IDLEA_R = crate::BitReader<bool>;
#[doc = "Field `IDLEA` writer - IDLEA field"]
pub type IDLEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCR_SPEC, bool, O>;
#[doc = "Field `IDLEB` reader - IDLEB field"]
pub type IDLEB_R = crate::BitReader<bool>;
#[doc = "Field `IDLEB` writer - IDLEB field"]
pub type IDLEB_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCR_SPEC, bool, O>;
#[doc = "Field `IDLEAN` reader - IDLEAN field"]
pub type IDLEAN_R = crate::BitReader<bool>;
#[doc = "Field `IDLEAN` writer - IDLEAN field"]
pub type IDLEAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCR_SPEC, bool, O>;
#[doc = "Field `IDLEBN` reader - IDLEBN field"]
pub type IDLEBN_R = crate::BitReader<bool>;
#[doc = "Field `IDLEBN` writer - IDLEBN field"]
pub type IDLEBN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCR_SPEC, bool, O>;
#[doc = "Field `INVA` reader - INVA field"]
pub type INVA_R = crate::BitReader<bool>;
#[doc = "Field `INVA` writer - INVA field"]
pub type INVA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCR_SPEC, bool, O>;
#[doc = "Field `INVB` reader - INVB field"]
pub type INVB_R = crate::BitReader<bool>;
#[doc = "Field `INVB` writer - INVB field"]
pub type INVB_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCR_SPEC, bool, O>;
#[doc = "Field `INVAN` reader - INVAN field"]
pub type INVAN_R = crate::BitReader<bool>;
#[doc = "Field `INVAN` writer - INVAN field"]
pub type INVAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCR_SPEC, bool, O>;
#[doc = "Field `INVBN` reader - INVBN field"]
pub type INVBN_R = crate::BitReader<bool>;
#[doc = "Field `INVBN` writer - INVBN field"]
pub type INVBN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IDLEA field"]
    #[inline(always)]
    pub fn idlea(&self) -> IDLEA_R {
        IDLEA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IDLEB field"]
    #[inline(always)]
    pub fn idleb(&self) -> IDLEB_R {
        IDLEB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDLEAN field"]
    #[inline(always)]
    pub fn idlean(&self) -> IDLEAN_R {
        IDLEAN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IDLEBN field"]
    #[inline(always)]
    pub fn idlebn(&self) -> IDLEBN_R {
        IDLEBN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INVA field"]
    #[inline(always)]
    pub fn inva(&self) -> INVA_R {
        INVA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INVB field"]
    #[inline(always)]
    pub fn invb(&self) -> INVB_R {
        INVB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INVAN field"]
    #[inline(always)]
    pub fn invan(&self) -> INVAN_R {
        INVAN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INVBN field"]
    #[inline(always)]
    pub fn invbn(&self) -> INVBN_R {
        INVBN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDLEA field"]
    #[inline(always)]
    pub fn idlea(&mut self) -> IDLEA_W<0> {
        IDLEA_W::new(self)
    }
    #[doc = "Bit 1 - IDLEB field"]
    #[inline(always)]
    pub fn idleb(&mut self) -> IDLEB_W<1> {
        IDLEB_W::new(self)
    }
    #[doc = "Bit 2 - IDLEAN field"]
    #[inline(always)]
    pub fn idlean(&mut self) -> IDLEAN_W<2> {
        IDLEAN_W::new(self)
    }
    #[doc = "Bit 3 - IDLEBN field"]
    #[inline(always)]
    pub fn idlebn(&mut self) -> IDLEBN_W<3> {
        IDLEBN_W::new(self)
    }
    #[doc = "Bit 4 - INVA field"]
    #[inline(always)]
    pub fn inva(&mut self) -> INVA_W<4> {
        INVA_W::new(self)
    }
    #[doc = "Bit 5 - INVB field"]
    #[inline(always)]
    pub fn invb(&mut self) -> INVB_W<5> {
        INVB_W::new(self)
    }
    #[doc = "Bit 6 - INVAN field"]
    #[inline(always)]
    pub fn invan(&mut self) -> INVAN_W<6> {
        INVAN_W::new(self)
    }
    #[doc = "Bit 7 - INVBN field"]
    #[inline(always)]
    pub fn invbn(&mut self) -> INVBN_W<7> {
        INVBN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr](index.html) module"]
pub struct OCR_SPEC;
impl crate::RegisterSpec for OCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocr::R](R) reader structure"]
impl crate::Readable for OCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocr::W](W) writer structure"]
impl crate::Writable for OCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCR to value 0"]
impl crate::Resettable for OCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
