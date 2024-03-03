#[doc = "Register `TRIM` reader"]
pub struct R(crate::R<TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM` writer"]
pub struct W(crate::W<TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_SPEC>;
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
impl From<crate::W<TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADJ` reader - ADJ field"]
pub type ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADJ` writer - ADJ field"]
pub type ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 8, O>;
#[doc = "Field `DEC` reader - DEC field"]
pub type DEC_R = crate::BitReader<bool>;
#[doc = "Field `DEC` writer - DEC field"]
pub type DEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - ADJ field"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - DEC field"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADJ field"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W<0> {
        ADJ_W::new(self)
    }
    #[doc = "Bit 8 - DEC field"]
    #[inline(always)]
    pub fn dec(&mut self) -> DEC_W<8> {
        DEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRIM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim](index.html) module"]
pub struct TRIM_SPEC;
impl crate::RegisterSpec for TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim::R](R) reader structure"]
impl crate::Readable for TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim::W](W) writer structure"]
impl crate::Writable for TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM to value 0"]
impl crate::Resettable for TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
