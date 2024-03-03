#[doc = "Register `XTALSR` reader"]
pub struct R(crate::R<XTALSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALSR` writer"]
pub struct W(crate::W<XTALSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALSR_SPEC>;
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
impl From<crate::W<XTALSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `_32KSTOP` reader - _32KSTOP field"]
pub type _32KSTOP_R = crate::BitReader<bool>;
#[doc = "Field `_32KSTOP` writer - _32KSTOP field"]
pub type _32KSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTALSR_SPEC, bool, O>;
#[doc = "Field `STOP` reader - STOP field"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - STOP field"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTALSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - _32KSTOP field"]
    #[inline(always)]
    pub fn _32kstop(&self) -> _32KSTOP_R {
        _32KSTOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP field"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - _32KSTOP field"]
    #[inline(always)]
    pub fn _32kstop(&mut self) -> _32KSTOP_W<0> {
        _32KSTOP_W::new(self)
    }
    #[doc = "Bit 1 - STOP field"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<1> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTALSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalsr](index.html) module"]
pub struct XTALSR_SPEC;
impl crate::RegisterSpec for XTALSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtalsr::R](R) reader structure"]
impl crate::Readable for XTALSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalsr::W](W) writer structure"]
impl crate::Writable for XTALSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTALSR to value 0"]
impl crate::Resettable for XTALSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
