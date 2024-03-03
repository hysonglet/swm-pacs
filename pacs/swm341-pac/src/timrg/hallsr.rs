#[doc = "Register `HALLSR` reader"]
pub struct R(crate::R<HALLSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALLSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HALLSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HALLSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HALLSR` writer"]
pub struct W(crate::W<HALLSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HALLSR_SPEC>;
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
impl From<crate::W<HALLSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HALLSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H0IN0` reader - H0IN0 field"]
pub type H0IN0_R = crate::BitReader<bool>;
#[doc = "Field `H0IN0` writer - H0IN0 field"]
pub type H0IN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HALLSR_SPEC, bool, O>;
#[doc = "Field `H0IN1` reader - H0IN1 field"]
pub type H0IN1_R = crate::BitReader<bool>;
#[doc = "Field `H0IN1` writer - H0IN1 field"]
pub type H0IN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HALLSR_SPEC, bool, O>;
#[doc = "Field `H0IN2` reader - H0IN2 field"]
pub type H0IN2_R = crate::BitReader<bool>;
#[doc = "Field `H0IN2` writer - H0IN2 field"]
pub type H0IN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HALLSR_SPEC, bool, O>;
#[doc = "Field `H3IN0` reader - H3IN0 field"]
pub type H3IN0_R = crate::BitReader<bool>;
#[doc = "Field `H3IN0` writer - H3IN0 field"]
pub type H3IN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HALLSR_SPEC, bool, O>;
#[doc = "Field `H3IN1` reader - H3IN1 field"]
pub type H3IN1_R = crate::BitReader<bool>;
#[doc = "Field `H3IN1` writer - H3IN1 field"]
pub type H3IN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HALLSR_SPEC, bool, O>;
#[doc = "Field `H3IN2` reader - H3IN2 field"]
pub type H3IN2_R = crate::BitReader<bool>;
#[doc = "Field `H3IN2` writer - H3IN2 field"]
pub type H3IN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HALLSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - H0IN0 field"]
    #[inline(always)]
    pub fn h0in0(&self) -> H0IN0_R {
        H0IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - H0IN1 field"]
    #[inline(always)]
    pub fn h0in1(&self) -> H0IN1_R {
        H0IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - H0IN2 field"]
    #[inline(always)]
    pub fn h0in2(&self) -> H0IN2_R {
        H0IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - H3IN0 field"]
    #[inline(always)]
    pub fn h3in0(&self) -> H3IN0_R {
        H3IN0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - H3IN1 field"]
    #[inline(always)]
    pub fn h3in1(&self) -> H3IN1_R {
        H3IN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - H3IN2 field"]
    #[inline(always)]
    pub fn h3in2(&self) -> H3IN2_R {
        H3IN2_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - H0IN0 field"]
    #[inline(always)]
    pub fn h0in0(&mut self) -> H0IN0_W<0> {
        H0IN0_W::new(self)
    }
    #[doc = "Bit 1 - H0IN1 field"]
    #[inline(always)]
    pub fn h0in1(&mut self) -> H0IN1_W<1> {
        H0IN1_W::new(self)
    }
    #[doc = "Bit 2 - H0IN2 field"]
    #[inline(always)]
    pub fn h0in2(&mut self) -> H0IN2_W<2> {
        H0IN2_W::new(self)
    }
    #[doc = "Bit 3 - H3IN0 field"]
    #[inline(always)]
    pub fn h3in0(&mut self) -> H3IN0_W<3> {
        H3IN0_W::new(self)
    }
    #[doc = "Bit 4 - H3IN1 field"]
    #[inline(always)]
    pub fn h3in1(&mut self) -> H3IN1_W<4> {
        H3IN1_W::new(self)
    }
    #[doc = "Bit 5 - H3IN2 field"]
    #[inline(always)]
    pub fn h3in2(&mut self) -> H3IN2_W<5> {
        H3IN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HALLSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hallsr](index.html) module"]
pub struct HALLSR_SPEC;
impl crate::RegisterSpec for HALLSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hallsr::R](R) reader structure"]
impl crate::Readable for HALLSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hallsr::W](W) writer structure"]
impl crate::Writable for HALLSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HALLSR to value 0"]
impl crate::Resettable for HALLSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
