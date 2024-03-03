#[doc = "Register `WHP` reader"]
pub struct R(crate::R<WHP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WHP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WHP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WHP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WHP` writer"]
pub struct W(crate::W<WHP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WHP_SPEC>;
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
impl From<crate::W<WHP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WHP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STA` reader - STA field"]
pub type STA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STA` writer - STA field"]
pub type STA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WHP_SPEC, u16, u16, 10, O>;
#[doc = "Field `STP` reader - STP field"]
pub type STP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STP` writer - STP field"]
pub type STP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WHP_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - STA field"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - STP field"]
    #[inline(always)]
    pub fn stp(&self) -> STP_R {
        STP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - STA field"]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W<0> {
        STA_W::new(self)
    }
    #[doc = "Bits 16:25 - STP field"]
    #[inline(always)]
    pub fn stp(&mut self) -> STP_W<16> {
        STP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WHP register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [whp](index.html) module"]
pub struct WHP_SPEC;
impl crate::RegisterSpec for WHP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [whp::R](R) reader structure"]
impl crate::Readable for WHP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [whp::W](W) writer structure"]
impl crate::Writable for WHP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WHP to value 0"]
impl crate::Resettable for WHP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
