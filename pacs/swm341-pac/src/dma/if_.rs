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
#[doc = "Field `CH0` reader - CH0 field"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - CH0 field"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CH1` reader - CH1 field"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - CH1 field"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CH2` reader - CH2 field"]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH2` writer - CH2 field"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `CH3` reader - CH3 field"]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH3` writer - CH3 field"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0 field"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 field"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 field"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 field"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 field"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - CH1 field"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - CH2 field"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - CH3 field"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
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
