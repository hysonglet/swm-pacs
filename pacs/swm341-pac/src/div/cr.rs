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
#[doc = "Field `DIVGO` reader - DIVGO field"]
pub type DIVGO_R = crate::BitReader<bool>;
#[doc = "Field `DIVGO` writer - DIVGO field"]
pub type DIVGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DIVSIGN` reader - DIVSIGN field"]
pub type DIVSIGN_R = crate::BitReader<bool>;
#[doc = "Field `DIVSIGN` writer - DIVSIGN field"]
pub type DIVSIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ROOTGO` reader - ROOTGO field"]
pub type ROOTGO_R = crate::BitReader<bool>;
#[doc = "Field `ROOTGO` writer - ROOTGO field"]
pub type ROOTGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ROOTMOD` reader - ROOTMOD field"]
pub type ROOTMOD_R = crate::BitReader<bool>;
#[doc = "Field `ROOTMOD` writer - ROOTMOD field"]
pub type ROOTMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DIVGO field"]
    #[inline(always)]
    pub fn divgo(&self) -> DIVGO_R {
        DIVGO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIVSIGN field"]
    #[inline(always)]
    pub fn divsign(&self) -> DIVSIGN_R {
        DIVSIGN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - ROOTGO field"]
    #[inline(always)]
    pub fn rootgo(&self) -> ROOTGO_R {
        ROOTGO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ROOTMOD field"]
    #[inline(always)]
    pub fn rootmod(&self) -> ROOTMOD_R {
        ROOTMOD_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIVGO field"]
    #[inline(always)]
    pub fn divgo(&mut self) -> DIVGO_W<0> {
        DIVGO_W::new(self)
    }
    #[doc = "Bit 1 - DIVSIGN field"]
    #[inline(always)]
    pub fn divsign(&mut self) -> DIVSIGN_W<1> {
        DIVSIGN_W::new(self)
    }
    #[doc = "Bit 8 - ROOTGO field"]
    #[inline(always)]
    pub fn rootgo(&mut self) -> ROOTGO_W<8> {
        ROOTGO_W::new(self)
    }
    #[doc = "Bit 9 - ROOTMOD field"]
    #[inline(always)]
    pub fn rootmod(&mut self) -> ROOTMOD_W<9> {
        ROOTMOD_W::new(self)
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
