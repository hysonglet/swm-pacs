#[doc = "Register `SEQTRG` reader"]
pub struct R(crate::R<SEQTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQTRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQTRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQTRG` writer"]
pub struct W(crate::W<SEQTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQTRG_SPEC>;
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
impl From<crate::W<SEQTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ0` reader - SEQ0 field"]
pub type SEQ0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ0` writer - SEQ0 field"]
pub type SEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQTRG_SPEC, u8, u8, 5, O>;
#[doc = "Field `SEQ1` reader - SEQ1 field"]
pub type SEQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ1` writer - SEQ1 field"]
pub type SEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQTRG_SPEC, u8, u8, 5, O>;
#[doc = "Field `SEQ2` reader - SEQ2 field"]
pub type SEQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ2` writer - SEQ2 field"]
pub type SEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQTRG_SPEC, u8, u8, 5, O>;
#[doc = "Field `SEQ3` reader - SEQ3 field"]
pub type SEQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ3` writer - SEQ3 field"]
pub type SEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQTRG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - SEQ0 field"]
    #[inline(always)]
    pub fn seq0(&self) -> SEQ0_R {
        SEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - SEQ1 field"]
    #[inline(always)]
    pub fn seq1(&self) -> SEQ1_R {
        SEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - SEQ2 field"]
    #[inline(always)]
    pub fn seq2(&self) -> SEQ2_R {
        SEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - SEQ3 field"]
    #[inline(always)]
    pub fn seq3(&self) -> SEQ3_R {
        SEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SEQ0 field"]
    #[inline(always)]
    pub fn seq0(&mut self) -> SEQ0_W<0> {
        SEQ0_W::new(self)
    }
    #[doc = "Bits 8:12 - SEQ1 field"]
    #[inline(always)]
    pub fn seq1(&mut self) -> SEQ1_W<8> {
        SEQ1_W::new(self)
    }
    #[doc = "Bits 16:20 - SEQ2 field"]
    #[inline(always)]
    pub fn seq2(&mut self) -> SEQ2_W<16> {
        SEQ2_W::new(self)
    }
    #[doc = "Bits 24:28 - SEQ3 field"]
    #[inline(always)]
    pub fn seq3(&mut self) -> SEQ3_W<24> {
        SEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SEQTRG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqtrg](index.html) module"]
pub struct SEQTRG_SPEC;
impl crate::RegisterSpec for SEQTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqtrg::R](R) reader structure"]
impl crate::Readable for SEQTRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqtrg::W](W) writer structure"]
impl crate::Writable for SEQTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQTRG to value 0"]
impl crate::Resettable for SEQTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
