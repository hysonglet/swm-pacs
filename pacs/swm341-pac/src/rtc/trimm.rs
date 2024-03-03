#[doc = "Register `TRIMM` reader"]
pub struct R(crate::R<TRIMM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIMM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIMM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIMM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIMM` writer"]
pub struct W(crate::W<TRIMM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIMM_SPEC>;
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
impl From<crate::W<TRIMM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIMM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CYCLE` reader - CYCLE field"]
pub type CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CYCLE` writer - CYCLE field"]
pub type CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIMM_SPEC, u8, u8, 3, O>;
#[doc = "Field `INC` reader - INC field"]
pub type INC_R = crate::BitReader<bool>;
#[doc = "Field `INC` writer - INC field"]
pub type INC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIMM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - CYCLE field"]
    #[inline(always)]
    pub fn cycle(&self) -> CYCLE_R {
        CYCLE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - INC field"]
    #[inline(always)]
    pub fn inc(&self) -> INC_R {
        INC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CYCLE field"]
    #[inline(always)]
    pub fn cycle(&mut self) -> CYCLE_W<0> {
        CYCLE_W::new(self)
    }
    #[doc = "Bit 3 - INC field"]
    #[inline(always)]
    pub fn inc(&mut self) -> INC_W<3> {
        INC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRIMM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trimm](index.html) module"]
pub struct TRIMM_SPEC;
impl crate::RegisterSpec for TRIMM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trimm::R](R) reader structure"]
impl crate::Readable for TRIMM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trimm::W](W) writer structure"]
impl crate::Writable for TRIMM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIMM to value 0"]
impl crate::Resettable for TRIMM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
