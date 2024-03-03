#[doc = "Register `SETUPSR` reader"]
pub struct R(crate::R<SETUPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUPSR` writer"]
pub struct W(crate::W<SETUPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUPSR_SPEC>;
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
impl From<crate::W<SETUPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUCC` reader - SUCC field"]
pub type SUCC_R = crate::BitReader<bool>;
#[doc = "Field `SUCC` writer - SUCC field"]
pub type SUCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETUPSR_SPEC, bool, O>;
#[doc = "Field `DONE` reader - DONE field"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - DONE field"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETUPSR_SPEC, bool, O>;
#[doc = "Field `EPNR` reader - EPNR field"]
pub type EPNR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNR` writer - EPNR field"]
pub type EPNR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUPSR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - SUCC field"]
    #[inline(always)]
    pub fn succ(&self) -> SUCC_R {
        SUCC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DONE field"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - EPNR field"]
    #[inline(always)]
    pub fn epnr(&self) -> EPNR_R {
        EPNR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SUCC field"]
    #[inline(always)]
    pub fn succ(&mut self) -> SUCC_W<0> {
        SUCC_W::new(self)
    }
    #[doc = "Bit 1 - DONE field"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<1> {
        DONE_W::new(self)
    }
    #[doc = "Bits 16:19 - EPNR field"]
    #[inline(always)]
    pub fn epnr(&mut self) -> EPNR_W<16> {
        EPNR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SETUPSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setupsr](index.html) module"]
pub struct SETUPSR_SPEC;
impl crate::RegisterSpec for SETUPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setupsr::R](R) reader structure"]
impl crate::Readable for SETUPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setupsr::W](W) writer structure"]
impl crate::Writable for SETUPSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETUPSR to value 0"]
impl crate::Resettable for SETUPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
