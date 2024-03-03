#[doc = "Register `MONDAY` reader"]
pub struct R(crate::R<MONDAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONDAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONDAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONDAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONDAY` writer"]
pub struct W(crate::W<MONDAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONDAY_SPEC>;
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
impl From<crate::W<MONDAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONDAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAY` reader - DAY field"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAY` writer - DAY field"]
pub type DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MONDAY_SPEC, u8, u8, 3, O>;
#[doc = "Field `MON` reader - MON field"]
pub type MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON` writer - MON field"]
pub type MON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MONDAY_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - DAY field"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - MON field"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAY field"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W<0> {
        DAY_W::new(self)
    }
    #[doc = "Bits 3:6 - MON field"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W<3> {
        MON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MONDAY register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monday](index.html) module"]
pub struct MONDAY_SPEC;
impl crate::RegisterSpec for MONDAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monday::R](R) reader structure"]
impl crate::Readable for MONDAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monday::W](W) writer structure"]
impl crate::Writable for MONDAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MONDAY to value 0"]
impl crate::Resettable for MONDAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
