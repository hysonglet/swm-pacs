#[doc = "Register `TIM` reader"]
pub struct R(crate::R<TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM` writer"]
pub struct W(crate::W<TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM_SPEC>;
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
impl From<crate::W<TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRCD` reader - TRCD field"]
pub type TRCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRCD` writer - TRCD field"]
pub type TRCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRFC` reader - TRFC field"]
pub type TRFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRFC` writer - TRFC field"]
pub type TRFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRP` reader - TRP field"]
pub type TRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRP` writer - TRP field"]
pub type TRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `T100US` reader - T100US field"]
pub type T100US_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T100US` writer - T100US field"]
pub type T100US_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:1 - TRCD field"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - TRFC field"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - TRP field"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:22 - T100US field"]
    #[inline(always)]
    pub fn t100us(&self) -> T100US_R {
        T100US_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - TRCD field"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W<0> {
        TRCD_W::new(self)
    }
    #[doc = "Bits 2:5 - TRFC field"]
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W<2> {
        TRFC_W::new(self)
    }
    #[doc = "Bits 6:7 - TRP field"]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W<6> {
        TRP_W::new(self)
    }
    #[doc = "Bits 8:22 - T100US field"]
    #[inline(always)]
    pub fn t100us(&mut self) -> T100US_W<8> {
        T100US_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim](index.html) module"]
pub struct TIM_SPEC;
impl crate::RegisterSpec for TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim::R](R) reader structure"]
impl crate::Readable for TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim::W](W) writer structure"]
impl crate::Writable for TIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
