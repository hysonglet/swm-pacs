#[doc = "Register `HALLIE` reader"]
pub struct R(crate::R<HALLIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALLIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HALLIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HALLIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HALLIE` writer"]
pub struct W(crate::W<HALLIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HALLIE_SPEC>;
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
impl From<crate::W<HALLIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HALLIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALL0` reader - HALL0 field"]
pub type HALL0_R = crate::BitReader<bool>;
#[doc = "Field `HALL0` writer - HALL0 field"]
pub type HALL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HALLIE_SPEC, bool, O>;
#[doc = "Field `HALL3` reader - HALL3 field"]
pub type HALL3_R = crate::BitReader<bool>;
#[doc = "Field `HALL3` writer - HALL3 field"]
pub type HALL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HALLIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HALL0 field"]
    #[inline(always)]
    pub fn hall0(&self) -> HALL0_R {
        HALL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HALL3 field"]
    #[inline(always)]
    pub fn hall3(&self) -> HALL3_R {
        HALL3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HALL0 field"]
    #[inline(always)]
    pub fn hall0(&mut self) -> HALL0_W<0> {
        HALL0_W::new(self)
    }
    #[doc = "Bit 1 - HALL3 field"]
    #[inline(always)]
    pub fn hall3(&mut self) -> HALL3_W<1> {
        HALL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HALLIE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hallie](index.html) module"]
pub struct HALLIE_SPEC;
impl crate::RegisterSpec for HALLIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hallie::R](R) reader structure"]
impl crate::Readable for HALLIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hallie::W](W) writer structure"]
impl crate::Writable for HALLIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HALLIE to value 0"]
impl crate::Resettable for HALLIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
