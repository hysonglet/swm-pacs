#[doc = "Register `GO` reader"]
pub struct R(crate::R<GO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GO` writer"]
pub struct W(crate::W<GO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GO_SPEC>;
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
impl From<crate::W<GO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ0` reader - SEQ0 field"]
pub type SEQ0_R = crate::BitReader<bool>;
#[doc = "Field `SEQ0` writer - SEQ0 field"]
pub type SEQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GO_SPEC, bool, O>;
#[doc = "Field `SEQ1` reader - SEQ1 field"]
pub type SEQ1_R = crate::BitReader<bool>;
#[doc = "Field `SEQ1` writer - SEQ1 field"]
pub type SEQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GO_SPEC, bool, O>;
#[doc = "Field `SEQ2` reader - SEQ2 field"]
pub type SEQ2_R = crate::BitReader<bool>;
#[doc = "Field `SEQ2` writer - SEQ2 field"]
pub type SEQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GO_SPEC, bool, O>;
#[doc = "Field `SEQ3` reader - SEQ3 field"]
pub type SEQ3_R = crate::BitReader<bool>;
#[doc = "Field `SEQ3` writer - SEQ3 field"]
pub type SEQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GO_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - BUSY field"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY field"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SEQ0 field"]
    #[inline(always)]
    pub fn seq0(&self) -> SEQ0_R {
        SEQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SEQ1 field"]
    #[inline(always)]
    pub fn seq1(&self) -> SEQ1_R {
        SEQ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEQ2 field"]
    #[inline(always)]
    pub fn seq2(&self) -> SEQ2_R {
        SEQ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SEQ3 field"]
    #[inline(always)]
    pub fn seq3(&self) -> SEQ3_R {
        SEQ3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BUSY field"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SEQ0 field"]
    #[inline(always)]
    pub fn seq0(&mut self) -> SEQ0_W<0> {
        SEQ0_W::new(self)
    }
    #[doc = "Bit 1 - SEQ1 field"]
    #[inline(always)]
    pub fn seq1(&mut self) -> SEQ1_W<1> {
        SEQ1_W::new(self)
    }
    #[doc = "Bit 2 - SEQ2 field"]
    #[inline(always)]
    pub fn seq2(&mut self) -> SEQ2_W<2> {
        SEQ2_W::new(self)
    }
    #[doc = "Bit 3 - SEQ3 field"]
    #[inline(always)]
    pub fn seq3(&mut self) -> SEQ3_W<3> {
        SEQ3_W::new(self)
    }
    #[doc = "Bit 4 - BUSY field"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<4> {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GO register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [go](index.html) module"]
pub struct GO_SPEC;
impl crate::RegisterSpec for GO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [go::R](R) reader structure"]
impl crate::Readable for GO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [go::W](W) writer structure"]
impl crate::Writable for GO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GO to value 0"]
impl crate::Resettable for GO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
