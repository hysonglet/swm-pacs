#[doc = "Register `CTSCR` reader"]
pub struct R(crate::R<CTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSCR` writer"]
pub struct W(crate::W<CTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSCR_SPEC>;
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
impl From<crate::W<CTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN field"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN field"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSCR_SPEC, bool, O>;
#[doc = "Field `POL` reader - POL field"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - POL field"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSCR_SPEC, bool, O>;
#[doc = "Field `STAT` reader - STAT field"]
pub type STAT_R = crate::BitReader<bool>;
#[doc = "Field `STAT` writer - STAT field"]
pub type STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - POL field"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - STAT field"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN field"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 2 - POL field"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<2> {
        POL_W::new(self)
    }
    #[doc = "Bit 7 - STAT field"]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W<7> {
        STAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctscr](index.html) module"]
pub struct CTSCR_SPEC;
impl crate::RegisterSpec for CTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctscr::R](R) reader structure"]
impl crate::Readable for CTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctscr::W](W) writer structure"]
impl crate::Writable for CTSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTSCR to value 0"]
impl crate::Resettable for CTSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
