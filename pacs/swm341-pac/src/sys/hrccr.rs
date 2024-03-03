#[doc = "Register `HRCCR` reader"]
pub struct R(crate::R<HRCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRCCR` writer"]
pub struct W(crate::W<HRCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRCCR_SPEC>;
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
impl From<crate::W<HRCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ON` reader - ON field"]
pub type ON_R = crate::BitReader<bool>;
#[doc = "Field `ON` writer - ON field"]
pub type ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCR_SPEC, bool, O>;
#[doc = "Field `DBL` reader - DBL field"]
pub type DBL_R = crate::BitReader<bool>;
#[doc = "Field `DBL` writer - DBL field"]
pub type DBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ON field"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBL field"]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ON field"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W<0> {
        ON_W::new(self)
    }
    #[doc = "Bit 1 - DBL field"]
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W<1> {
        DBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRCCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrccr](index.html) module"]
pub struct HRCCR_SPEC;
impl crate::RegisterSpec for HRCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrccr::R](R) reader structure"]
impl crate::Readable for HRCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrccr::W](W) writer structure"]
impl crate::Writable for HRCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HRCCR to value 0"]
impl crate::Resettable for HRCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
