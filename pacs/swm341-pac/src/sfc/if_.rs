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
#[doc = "Field `OVER` reader - OVER field"]
pub type OVER_R = crate::BitReader<bool>;
#[doc = "Field `OVER` writer - OVER field"]
pub type OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ERR` reader - ERR field"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - ERR field"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `TO` reader - TO field"]
pub type TO_R = crate::BitReader<bool>;
#[doc = "Field `TO` writer - TO field"]
pub type TO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `FIFOEMPTY` reader - FIFOEMPTY field"]
pub type FIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOEMPTY` writer - FIFOEMPTY field"]
pub type FIFOEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `FIFOHFULL` reader - FIFOHFULL field"]
pub type FIFOHFULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFOHFULL` writer - FIFOHFULL field"]
pub type FIFOHFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `FIFOFULL` reader - FIFOFULL field"]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFULL` writer - FIFOFULL field"]
pub type FIFOFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `FIFOOVF` reader - FIFOOVF field"]
pub type FIFOOVF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOOVF` writer - FIFOOVF field"]
pub type FIFOOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `FIFOUVF` reader - FIFOUVF field"]
pub type FIFOUVF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOUVF` writer - FIFOUVF field"]
pub type FIFOUVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OVER field"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERR field"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TO field"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 27 - FIFOEMPTY field"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FIFOHFULL field"]
    #[inline(always)]
    pub fn fifohfull(&self) -> FIFOHFULL_R {
        FIFOHFULL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFOFULL field"]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - FIFOOVF field"]
    #[inline(always)]
    pub fn fifoovf(&self) -> FIFOOVF_R {
        FIFOOVF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FIFOUVF field"]
    #[inline(always)]
    pub fn fifouvf(&self) -> FIFOUVF_R {
        FIFOUVF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVER field"]
    #[inline(always)]
    pub fn over(&mut self) -> OVER_W<0> {
        OVER_W::new(self)
    }
    #[doc = "Bit 1 - ERR field"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W<1> {
        ERR_W::new(self)
    }
    #[doc = "Bit 2 - TO field"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W<2> {
        TO_W::new(self)
    }
    #[doc = "Bit 27 - FIFOEMPTY field"]
    #[inline(always)]
    pub fn fifoempty(&mut self) -> FIFOEMPTY_W<27> {
        FIFOEMPTY_W::new(self)
    }
    #[doc = "Bit 28 - FIFOHFULL field"]
    #[inline(always)]
    pub fn fifohfull(&mut self) -> FIFOHFULL_W<28> {
        FIFOHFULL_W::new(self)
    }
    #[doc = "Bit 29 - FIFOFULL field"]
    #[inline(always)]
    pub fn fifofull(&mut self) -> FIFOFULL_W<29> {
        FIFOFULL_W::new(self)
    }
    #[doc = "Bit 30 - FIFOOVF field"]
    #[inline(always)]
    pub fn fifoovf(&mut self) -> FIFOOVF_W<30> {
        FIFOOVF_W::new(self)
    }
    #[doc = "Bit 31 - FIFOUVF field"]
    #[inline(always)]
    pub fn fifouvf(&mut self) -> FIFOUVF_W<31> {
        FIFOUVF_W::new(self)
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
