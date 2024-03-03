#[doc = "Register `SEQCHN0` reader"]
pub struct R(crate::R<SEQCHN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCHN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCHN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCHN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCHN0` writer"]
pub struct W(crate::W<SEQCHN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCHN0_SPEC>;
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
impl From<crate::W<SEQCHN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCHN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ0` reader - SEQ0 field"]
pub type SEQ0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEQ0` writer - SEQ0 field"]
pub type SEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCHN0_SPEC, u16, u16, 12, O>;
#[doc = "Field `SEQ1` reader - SEQ1 field"]
pub type SEQ1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEQ1` writer - SEQ1 field"]
pub type SEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCHN0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - SEQ0 field"]
    #[inline(always)]
    pub fn seq0(&self) -> SEQ0_R {
        SEQ0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - SEQ1 field"]
    #[inline(always)]
    pub fn seq1(&self) -> SEQ1_R {
        SEQ1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - SEQ0 field"]
    #[inline(always)]
    pub fn seq0(&mut self) -> SEQ0_W<0> {
        SEQ0_W::new(self)
    }
    #[doc = "Bits 16:27 - SEQ1 field"]
    #[inline(always)]
    pub fn seq1(&mut self) -> SEQ1_W<16> {
        SEQ1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SEQCHN0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqchn0](index.html) module"]
pub struct SEQCHN0_SPEC;
impl crate::RegisterSpec for SEQCHN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqchn0::R](R) reader structure"]
impl crate::Readable for SEQCHN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqchn0::W](W) writer structure"]
impl crate::Writable for SEQCHN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQCHN0 to value 0"]
impl crate::Resettable for SEQCHN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
