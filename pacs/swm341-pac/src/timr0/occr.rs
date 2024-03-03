#[doc = "Register `OCCR` reader"]
pub struct R(crate::R<OCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCCR` writer"]
pub struct W(crate::W<OCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCCR_SPEC>;
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
impl From<crate::W<OCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCELVL` reader - FORCELVL field"]
pub type FORCELVL_R = crate::BitReader<bool>;
#[doc = "Field `FORCELVL` writer - FORCELVL field"]
pub type FORCELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCCR_SPEC, bool, O>;
#[doc = "Field `INITLVL` reader - INITLVL field"]
pub type INITLVL_R = crate::BitReader<bool>;
#[doc = "Field `INITLVL` writer - INITLVL field"]
pub type INITLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCCR_SPEC, bool, O>;
#[doc = "Field `FORCEEN` reader - FORCEEN field"]
pub type FORCEEN_R = crate::BitReader<bool>;
#[doc = "Field `FORCEEN` writer - FORCEEN field"]
pub type FORCEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FORCELVL field"]
    #[inline(always)]
    pub fn forcelvl(&self) -> FORCELVL_R {
        FORCELVL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INITLVL field"]
    #[inline(always)]
    pub fn initlvl(&self) -> INITLVL_R {
        INITLVL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FORCEEN field"]
    #[inline(always)]
    pub fn forceen(&self) -> FORCEEN_R {
        FORCEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FORCELVL field"]
    #[inline(always)]
    pub fn forcelvl(&mut self) -> FORCELVL_W<0> {
        FORCELVL_W::new(self)
    }
    #[doc = "Bit 1 - INITLVL field"]
    #[inline(always)]
    pub fn initlvl(&mut self) -> INITLVL_W<1> {
        INITLVL_W::new(self)
    }
    #[doc = "Bit 2 - FORCEEN field"]
    #[inline(always)]
    pub fn forceen(&mut self) -> FORCEEN_W<2> {
        FORCEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [occr](index.html) module"]
pub struct OCCR_SPEC;
impl crate::RegisterSpec for OCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [occr::R](R) reader structure"]
impl crate::Readable for OCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [occr::W](W) writer structure"]
impl crate::Writable for OCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCCR to value 0"]
impl crate::Resettable for OCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
