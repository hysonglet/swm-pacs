#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEBUSY` reader - ERASEBUSY field"]
pub type ERASEBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ERASEBUSY` writer - ERASEBUSY field"]
pub type ERASEBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `PROGBUSY` reader - PROGBUSY field"]
pub type PROGBUSY_R = crate::BitReader<bool>;
#[doc = "Field `PROGBUSY` writer - PROGBUSY field"]
pub type PROGBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `READBUSY` reader - READBUSY field"]
pub type READBUSY_R = crate::BitReader<bool>;
#[doc = "Field `READBUSY` writer - READBUSY field"]
pub type READBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FIFOEMPTY` reader - FIFOEMPTY field"]
pub type FIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOEMPTY` writer - FIFOEMPTY field"]
pub type FIFOEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FIFOFULL` reader - FIFOFULL field"]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFULL` writer - FIFOFULL field"]
pub type FIFOFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `IDLE` reader - IDLE field"]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `IDLE` writer - IDLE field"]
pub type IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ERASEBUSY field"]
    #[inline(always)]
    pub fn erasebusy(&self) -> ERASEBUSY_R {
        ERASEBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PROGBUSY field"]
    #[inline(always)]
    pub fn progbusy(&self) -> PROGBUSY_R {
        PROGBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - READBUSY field"]
    #[inline(always)]
    pub fn readbusy(&self) -> READBUSY_R {
        READBUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFOEMPTY field"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFOFULL field"]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - IDLE field"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERASEBUSY field"]
    #[inline(always)]
    pub fn erasebusy(&mut self) -> ERASEBUSY_W<0> {
        ERASEBUSY_W::new(self)
    }
    #[doc = "Bit 1 - PROGBUSY field"]
    #[inline(always)]
    pub fn progbusy(&mut self) -> PROGBUSY_W<1> {
        PROGBUSY_W::new(self)
    }
    #[doc = "Bit 2 - READBUSY field"]
    #[inline(always)]
    pub fn readbusy(&mut self) -> READBUSY_W<2> {
        READBUSY_W::new(self)
    }
    #[doc = "Bit 3 - FIFOEMPTY field"]
    #[inline(always)]
    pub fn fifoempty(&mut self) -> FIFOEMPTY_W<3> {
        FIFOEMPTY_W::new(self)
    }
    #[doc = "Bit 4 - FIFOFULL field"]
    #[inline(always)]
    pub fn fifofull(&mut self) -> FIFOFULL_W<4> {
        FIFOFULL_W::new(self)
    }
    #[doc = "Bit 31 - IDLE field"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W<31> {
        IDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
