#[doc = "Register `MINSECAL` reader"]
pub struct R(crate::R<MINSECAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MINSECAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MINSECAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MINSECAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MINSECAL` writer"]
pub struct W(crate::W<MINSECAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MINSECAL_SPEC>;
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
impl From<crate::W<MINSECAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MINSECAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - SEC field"]
pub type SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC` writer - SEC field"]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MINSECAL_SPEC, u8, u8, 6, O>;
#[doc = "Field `MIN` reader - MIN field"]
pub type MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN` writer - MIN field"]
pub type MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MINSECAL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - SEC field"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - MIN field"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SEC field"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
    }
    #[doc = "Bits 6:11 - MIN field"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W<6> {
        MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MINSECAL register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [minsecal](index.html) module"]
pub struct MINSECAL_SPEC;
impl crate::RegisterSpec for MINSECAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [minsecal::R](R) reader structure"]
impl crate::Readable for MINSECAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [minsecal::W](W) writer structure"]
impl crate::Writable for MINSECAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MINSECAL to value 0"]
impl crate::Resettable for MINSECAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
