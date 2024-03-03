#[doc = "Register `TXCR` reader"]
pub struct R(crate::R<TXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCR` writer"]
pub struct W(crate::W<TXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCR_SPEC>;
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
impl From<crate::W<TXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNDSTALL` reader - SNDSTALL field"]
pub type SNDSTALL_R = crate::BitReader<bool>;
#[doc = "Field `SNDSTALL` writer - SNDSTALL field"]
pub type SNDSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCR_SPEC, bool, O>;
#[doc = "Field `FLUSHFF` reader - FLUSHFF field"]
pub type FLUSHFF_R = crate::BitReader<bool>;
#[doc = "Field `FLUSHFF` writer - FLUSHFF field"]
pub type FLUSHFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCR_SPEC, bool, O>;
#[doc = "Field `FFRDY` reader - FFRDY field"]
pub type FFRDY_R = crate::BitReader<bool>;
#[doc = "Field `FFRDY` writer - FFRDY field"]
pub type FFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SNDSTALL field"]
    #[inline(always)]
    pub fn sndstall(&self) -> SNDSTALL_R {
        SNDSTALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - FLUSHFF field"]
    #[inline(always)]
    pub fn flushff(&self) -> FLUSHFF_R {
        FLUSHFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FFRDY field"]
    #[inline(always)]
    pub fn ffrdy(&self) -> FFRDY_R {
        FFRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SNDSTALL field"]
    #[inline(always)]
    pub fn sndstall(&mut self) -> SNDSTALL_W<0> {
        SNDSTALL_W::new(self)
    }
    #[doc = "Bit 2 - FLUSHFF field"]
    #[inline(always)]
    pub fn flushff(&mut self) -> FLUSHFF_W<2> {
        FLUSHFF_W::new(self)
    }
    #[doc = "Bit 3 - FFRDY field"]
    #[inline(always)]
    pub fn ffrdy(&mut self) -> FFRDY_W<3> {
        FFRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcr](index.html) module"]
pub struct TXCR_SPEC;
impl crate::RegisterSpec for TXCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txcr::R](R) reader structure"]
impl crate::Readable for TXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcr::W](W) writer structure"]
impl crate::Writable for TXCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCR to value 0"]
impl crate::Resettable for TXCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
