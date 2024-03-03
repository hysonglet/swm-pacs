#[doc = "Register `DATHUR` reader"]
pub struct R(crate::R<DATHUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATHUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATHUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATHUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATHUR` writer"]
pub struct W(crate::W<DATHUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATHUR_SPEC>;
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
impl From<crate::W<DATHUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATHUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOUR` reader - HOUR field"]
pub type HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOUR` writer - HOUR field"]
pub type HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATHUR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATE` reader - DATE field"]
pub type DATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATE` writer - DATE field"]
pub type DATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATHUR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - HOUR field"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - DATE field"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - HOUR field"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W<0> {
        HOUR_W::new(self)
    }
    #[doc = "Bits 5:9 - DATE field"]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W<5> {
        DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DATHUR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dathur](index.html) module"]
pub struct DATHUR_SPEC;
impl crate::RegisterSpec for DATHUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dathur::R](R) reader structure"]
impl crate::Readable for DATHUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dathur::W](W) writer structure"]
impl crate::Writable for DATHUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATHUR to value 0"]
impl crate::Resettable for DATHUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
