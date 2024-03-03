#[doc = "Register `SEQCHN1` reader"]
pub struct R(crate::R<SEQCHN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCHN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCHN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCHN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCHN1` writer"]
pub struct W(crate::W<SEQCHN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCHN1_SPEC>;
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
impl From<crate::W<SEQCHN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCHN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ2` reader - SEQ2 field"]
pub type SEQ2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEQ2` writer - SEQ2 field"]
pub type SEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCHN1_SPEC, u16, u16, 12, O>;
#[doc = "Field `SEQ3` reader - SEQ3 field"]
pub type SEQ3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEQ3` writer - SEQ3 field"]
pub type SEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCHN1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - SEQ2 field"]
    #[inline(always)]
    pub fn seq2(&self) -> SEQ2_R {
        SEQ2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - SEQ3 field"]
    #[inline(always)]
    pub fn seq3(&self) -> SEQ3_R {
        SEQ3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - SEQ2 field"]
    #[inline(always)]
    pub fn seq2(&mut self) -> SEQ2_W<0> {
        SEQ2_W::new(self)
    }
    #[doc = "Bits 16:27 - SEQ3 field"]
    #[inline(always)]
    pub fn seq3(&mut self) -> SEQ3_W<16> {
        SEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SEQCHN1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqchn1](index.html) module"]
pub struct SEQCHN1_SPEC;
impl crate::RegisterSpec for SEQCHN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqchn1::R](R) reader structure"]
impl crate::Readable for SEQCHN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqchn1::W](W) writer structure"]
impl crate::Writable for SEQCHN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQCHN1 to value 0"]
impl crate::Resettable for SEQCHN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
