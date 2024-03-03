#[doc = "Register `PULSE` reader"]
pub struct R(crate::R<PULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULSE` writer"]
pub struct W(crate::W<PULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSE_SPEC>;
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
impl From<crate::W<PULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGE0` reader - EDGE0 field"]
pub type EDGE0_R = crate::BitReader<bool>;
#[doc = "Field `EDGE0` writer - EDGE0 field"]
pub type EDGE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULSE_SPEC, bool, O>;
#[doc = "Field `EDGE1` reader - EDGE1 field"]
pub type EDGE1_R = crate::BitReader<bool>;
#[doc = "Field `EDGE1` writer - EDGE1 field"]
pub type EDGE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULSE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EDGE0 field"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EDGE1 field"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EDGE0 field"]
    #[inline(always)]
    pub fn edge0(&mut self) -> EDGE0_W<0> {
        EDGE0_W::new(self)
    }
    #[doc = "Bit 1 - EDGE1 field"]
    #[inline(always)]
    pub fn edge1(&mut self) -> EDGE1_W<1> {
        EDGE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PULSE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse](index.html) module"]
pub struct PULSE_SPEC;
impl crate::RegisterSpec for PULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulse::R](R) reader structure"]
impl crate::Readable for PULSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulse::W](W) writer structure"]
impl crate::Writable for PULSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PULSE to value 0"]
impl crate::Resettable for PULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
