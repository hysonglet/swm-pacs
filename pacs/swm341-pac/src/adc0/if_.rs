#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ0EOC` reader - SEQ0EOC field"]
pub type SEQ0EOC_R = crate::BitReader<bool>;
#[doc = "Field `SEQ0EOC` writer - SEQ0EOC field"]
pub type SEQ0EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ0OVF` reader - SEQ0OVF field"]
pub type SEQ0OVF_R = crate::BitReader<bool>;
#[doc = "Field `SEQ0OVF` writer - SEQ0OVF field"]
pub type SEQ0OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ0HALF` reader - SEQ0HALF field"]
pub type SEQ0HALF_R = crate::BitReader<bool>;
#[doc = "Field `SEQ0HALF` writer - SEQ0HALF field"]
pub type SEQ0HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ0FULL` reader - SEQ0FULL field"]
pub type SEQ0FULL_R = crate::BitReader<bool>;
#[doc = "Field `SEQ0FULL` writer - SEQ0FULL field"]
pub type SEQ0FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ0CMPMAX` reader - SEQ0CMPMAX field"]
pub type SEQ0CMPMAX_R = crate::BitReader<bool>;
#[doc = "Field `SEQ0CMPMAX` writer - SEQ0CMPMAX field"]
pub type SEQ0CMPMAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ0CMPMIN` reader - SEQ0CMPMIN field"]
pub type SEQ0CMPMIN_R = crate::BitReader<bool>;
#[doc = "Field `SEQ0CMPMIN` writer - SEQ0CMPMIN field"]
pub type SEQ0CMPMIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ1EOC` reader - SEQ1EOC field"]
pub type SEQ1EOC_R = crate::BitReader<bool>;
#[doc = "Field `SEQ1EOC` writer - SEQ1EOC field"]
pub type SEQ1EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ1OVF` reader - SEQ1OVF field"]
pub type SEQ1OVF_R = crate::BitReader<bool>;
#[doc = "Field `SEQ1OVF` writer - SEQ1OVF field"]
pub type SEQ1OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ1HALF` reader - SEQ1HALF field"]
pub type SEQ1HALF_R = crate::BitReader<bool>;
#[doc = "Field `SEQ1HALF` writer - SEQ1HALF field"]
pub type SEQ1HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ1FULL` reader - SEQ1FULL field"]
pub type SEQ1FULL_R = crate::BitReader<bool>;
#[doc = "Field `SEQ1FULL` writer - SEQ1FULL field"]
pub type SEQ1FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ1CMPMAX` reader - SEQ1CMPMAX field"]
pub type SEQ1CMPMAX_R = crate::BitReader<bool>;
#[doc = "Field `SEQ1CMPMAX` writer - SEQ1CMPMAX field"]
pub type SEQ1CMPMAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ1CMPMIN` reader - SEQ1CMPMIN field"]
pub type SEQ1CMPMIN_R = crate::BitReader<bool>;
#[doc = "Field `SEQ1CMPMIN` writer - SEQ1CMPMIN field"]
pub type SEQ1CMPMIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ2EOC` reader - SEQ2EOC field"]
pub type SEQ2EOC_R = crate::BitReader<bool>;
#[doc = "Field `SEQ2EOC` writer - SEQ2EOC field"]
pub type SEQ2EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ2OVF` reader - SEQ2OVF field"]
pub type SEQ2OVF_R = crate::BitReader<bool>;
#[doc = "Field `SEQ2OVF` writer - SEQ2OVF field"]
pub type SEQ2OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ2HALF` reader - SEQ2HALF field"]
pub type SEQ2HALF_R = crate::BitReader<bool>;
#[doc = "Field `SEQ2HALF` writer - SEQ2HALF field"]
pub type SEQ2HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ2FULL` reader - SEQ2FULL field"]
pub type SEQ2FULL_R = crate::BitReader<bool>;
#[doc = "Field `SEQ2FULL` writer - SEQ2FULL field"]
pub type SEQ2FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ2CMPMAX` reader - SEQ2CMPMAX field"]
pub type SEQ2CMPMAX_R = crate::BitReader<bool>;
#[doc = "Field `SEQ2CMPMAX` writer - SEQ2CMPMAX field"]
pub type SEQ2CMPMAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ2CMPMIN` reader - SEQ2CMPMIN field"]
pub type SEQ2CMPMIN_R = crate::BitReader<bool>;
#[doc = "Field `SEQ2CMPMIN` writer - SEQ2CMPMIN field"]
pub type SEQ2CMPMIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ3EOC` reader - SEQ3EOC field"]
pub type SEQ3EOC_R = crate::BitReader<bool>;
#[doc = "Field `SEQ3EOC` writer - SEQ3EOC field"]
pub type SEQ3EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ3OVF` reader - SEQ3OVF field"]
pub type SEQ3OVF_R = crate::BitReader<bool>;
#[doc = "Field `SEQ3OVF` writer - SEQ3OVF field"]
pub type SEQ3OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ3HALF` reader - SEQ3HALF field"]
pub type SEQ3HALF_R = crate::BitReader<bool>;
#[doc = "Field `SEQ3HALF` writer - SEQ3HALF field"]
pub type SEQ3HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ3FULL` reader - SEQ3FULL field"]
pub type SEQ3FULL_R = crate::BitReader<bool>;
#[doc = "Field `SEQ3FULL` writer - SEQ3FULL field"]
pub type SEQ3FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ3CMPMAX` reader - SEQ3CMPMAX field"]
pub type SEQ3CMPMAX_R = crate::BitReader<bool>;
#[doc = "Field `SEQ3CMPMAX` writer - SEQ3CMPMAX field"]
pub type SEQ3CMPMAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SEQ3CMPMIN` reader - SEQ3CMPMIN field"]
pub type SEQ3CMPMIN_R = crate::BitReader<bool>;
#[doc = "Field `SEQ3CMPMIN` writer - SEQ3CMPMIN field"]
pub type SEQ3CMPMIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SEQ0EOC field"]
    #[inline(always)]
    pub fn seq0eoc(&self) -> SEQ0EOC_R {
        SEQ0EOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SEQ0OVF field"]
    #[inline(always)]
    pub fn seq0ovf(&self) -> SEQ0OVF_R {
        SEQ0OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEQ0HALF field"]
    #[inline(always)]
    pub fn seq0half(&self) -> SEQ0HALF_R {
        SEQ0HALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SEQ0FULL field"]
    #[inline(always)]
    pub fn seq0full(&self) -> SEQ0FULL_R {
        SEQ0FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEQ0CMPMAX field"]
    #[inline(always)]
    pub fn seq0cmpmax(&self) -> SEQ0CMPMAX_R {
        SEQ0CMPMAX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SEQ0CMPMIN field"]
    #[inline(always)]
    pub fn seq0cmpmin(&self) -> SEQ0CMPMIN_R {
        SEQ0CMPMIN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SEQ1EOC field"]
    #[inline(always)]
    pub fn seq1eoc(&self) -> SEQ1EOC_R {
        SEQ1EOC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SEQ1OVF field"]
    #[inline(always)]
    pub fn seq1ovf(&self) -> SEQ1OVF_R {
        SEQ1OVF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SEQ1HALF field"]
    #[inline(always)]
    pub fn seq1half(&self) -> SEQ1HALF_R {
        SEQ1HALF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SEQ1FULL field"]
    #[inline(always)]
    pub fn seq1full(&self) -> SEQ1FULL_R {
        SEQ1FULL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SEQ1CMPMAX field"]
    #[inline(always)]
    pub fn seq1cmpmax(&self) -> SEQ1CMPMAX_R {
        SEQ1CMPMAX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SEQ1CMPMIN field"]
    #[inline(always)]
    pub fn seq1cmpmin(&self) -> SEQ1CMPMIN_R {
        SEQ1CMPMIN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - SEQ2EOC field"]
    #[inline(always)]
    pub fn seq2eoc(&self) -> SEQ2EOC_R {
        SEQ2EOC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SEQ2OVF field"]
    #[inline(always)]
    pub fn seq2ovf(&self) -> SEQ2OVF_R {
        SEQ2OVF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SEQ2HALF field"]
    #[inline(always)]
    pub fn seq2half(&self) -> SEQ2HALF_R {
        SEQ2HALF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SEQ2FULL field"]
    #[inline(always)]
    pub fn seq2full(&self) -> SEQ2FULL_R {
        SEQ2FULL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SEQ2CMPMAX field"]
    #[inline(always)]
    pub fn seq2cmpmax(&self) -> SEQ2CMPMAX_R {
        SEQ2CMPMAX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SEQ2CMPMIN field"]
    #[inline(always)]
    pub fn seq2cmpmin(&self) -> SEQ2CMPMIN_R {
        SEQ2CMPMIN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - SEQ3EOC field"]
    #[inline(always)]
    pub fn seq3eoc(&self) -> SEQ3EOC_R {
        SEQ3EOC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SEQ3OVF field"]
    #[inline(always)]
    pub fn seq3ovf(&self) -> SEQ3OVF_R {
        SEQ3OVF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SEQ3HALF field"]
    #[inline(always)]
    pub fn seq3half(&self) -> SEQ3HALF_R {
        SEQ3HALF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SEQ3FULL field"]
    #[inline(always)]
    pub fn seq3full(&self) -> SEQ3FULL_R {
        SEQ3FULL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SEQ3CMPMAX field"]
    #[inline(always)]
    pub fn seq3cmpmax(&self) -> SEQ3CMPMAX_R {
        SEQ3CMPMAX_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SEQ3CMPMIN field"]
    #[inline(always)]
    pub fn seq3cmpmin(&self) -> SEQ3CMPMIN_R {
        SEQ3CMPMIN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SEQ0EOC field"]
    #[inline(always)]
    pub fn seq0eoc(&mut self) -> SEQ0EOC_W<0> {
        SEQ0EOC_W::new(self)
    }
    #[doc = "Bit 1 - SEQ0OVF field"]
    #[inline(always)]
    pub fn seq0ovf(&mut self) -> SEQ0OVF_W<1> {
        SEQ0OVF_W::new(self)
    }
    #[doc = "Bit 2 - SEQ0HALF field"]
    #[inline(always)]
    pub fn seq0half(&mut self) -> SEQ0HALF_W<2> {
        SEQ0HALF_W::new(self)
    }
    #[doc = "Bit 3 - SEQ0FULL field"]
    #[inline(always)]
    pub fn seq0full(&mut self) -> SEQ0FULL_W<3> {
        SEQ0FULL_W::new(self)
    }
    #[doc = "Bit 4 - SEQ0CMPMAX field"]
    #[inline(always)]
    pub fn seq0cmpmax(&mut self) -> SEQ0CMPMAX_W<4> {
        SEQ0CMPMAX_W::new(self)
    }
    #[doc = "Bit 5 - SEQ0CMPMIN field"]
    #[inline(always)]
    pub fn seq0cmpmin(&mut self) -> SEQ0CMPMIN_W<5> {
        SEQ0CMPMIN_W::new(self)
    }
    #[doc = "Bit 8 - SEQ1EOC field"]
    #[inline(always)]
    pub fn seq1eoc(&mut self) -> SEQ1EOC_W<8> {
        SEQ1EOC_W::new(self)
    }
    #[doc = "Bit 9 - SEQ1OVF field"]
    #[inline(always)]
    pub fn seq1ovf(&mut self) -> SEQ1OVF_W<9> {
        SEQ1OVF_W::new(self)
    }
    #[doc = "Bit 10 - SEQ1HALF field"]
    #[inline(always)]
    pub fn seq1half(&mut self) -> SEQ1HALF_W<10> {
        SEQ1HALF_W::new(self)
    }
    #[doc = "Bit 11 - SEQ1FULL field"]
    #[inline(always)]
    pub fn seq1full(&mut self) -> SEQ1FULL_W<11> {
        SEQ1FULL_W::new(self)
    }
    #[doc = "Bit 12 - SEQ1CMPMAX field"]
    #[inline(always)]
    pub fn seq1cmpmax(&mut self) -> SEQ1CMPMAX_W<12> {
        SEQ1CMPMAX_W::new(self)
    }
    #[doc = "Bit 13 - SEQ1CMPMIN field"]
    #[inline(always)]
    pub fn seq1cmpmin(&mut self) -> SEQ1CMPMIN_W<13> {
        SEQ1CMPMIN_W::new(self)
    }
    #[doc = "Bit 16 - SEQ2EOC field"]
    #[inline(always)]
    pub fn seq2eoc(&mut self) -> SEQ2EOC_W<16> {
        SEQ2EOC_W::new(self)
    }
    #[doc = "Bit 17 - SEQ2OVF field"]
    #[inline(always)]
    pub fn seq2ovf(&mut self) -> SEQ2OVF_W<17> {
        SEQ2OVF_W::new(self)
    }
    #[doc = "Bit 18 - SEQ2HALF field"]
    #[inline(always)]
    pub fn seq2half(&mut self) -> SEQ2HALF_W<18> {
        SEQ2HALF_W::new(self)
    }
    #[doc = "Bit 19 - SEQ2FULL field"]
    #[inline(always)]
    pub fn seq2full(&mut self) -> SEQ2FULL_W<19> {
        SEQ2FULL_W::new(self)
    }
    #[doc = "Bit 20 - SEQ2CMPMAX field"]
    #[inline(always)]
    pub fn seq2cmpmax(&mut self) -> SEQ2CMPMAX_W<20> {
        SEQ2CMPMAX_W::new(self)
    }
    #[doc = "Bit 21 - SEQ2CMPMIN field"]
    #[inline(always)]
    pub fn seq2cmpmin(&mut self) -> SEQ2CMPMIN_W<21> {
        SEQ2CMPMIN_W::new(self)
    }
    #[doc = "Bit 24 - SEQ3EOC field"]
    #[inline(always)]
    pub fn seq3eoc(&mut self) -> SEQ3EOC_W<24> {
        SEQ3EOC_W::new(self)
    }
    #[doc = "Bit 25 - SEQ3OVF field"]
    #[inline(always)]
    pub fn seq3ovf(&mut self) -> SEQ3OVF_W<25> {
        SEQ3OVF_W::new(self)
    }
    #[doc = "Bit 26 - SEQ3HALF field"]
    #[inline(always)]
    pub fn seq3half(&mut self) -> SEQ3HALF_W<26> {
        SEQ3HALF_W::new(self)
    }
    #[doc = "Bit 27 - SEQ3FULL field"]
    #[inline(always)]
    pub fn seq3full(&mut self) -> SEQ3FULL_W<27> {
        SEQ3FULL_W::new(self)
    }
    #[doc = "Bit 28 - SEQ3CMPMAX field"]
    #[inline(always)]
    pub fn seq3cmpmax(&mut self) -> SEQ3CMPMAX_W<28> {
        SEQ3CMPMAX_W::new(self)
    }
    #[doc = "Bit 29 - SEQ3CMPMIN field"]
    #[inline(always)]
    pub fn seq3cmpmin(&mut self) -> SEQ3CMPMIN_W<29> {
        SEQ3CMPMIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
