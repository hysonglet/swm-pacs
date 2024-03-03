#[doc = "Register `FFTHR` reader"]
pub struct R(crate::R<FFTHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFTHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFTHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFTHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFTHR` writer"]
pub struct W(crate::W<FFTHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFTHR_SPEC>;
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
impl From<crate::W<FFTHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFTHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX` reader - TX field"]
pub type TX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX` writer - TX field"]
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFTHR_SPEC, u16, u16, 10, O>;
#[doc = "Field `RX` reader - RX field"]
pub type RX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX` writer - RX field"]
pub type RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFTHR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - TX field"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - RX field"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TX field"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W<0> {
        TX_W::new(self)
    }
    #[doc = "Bits 16:25 - RX field"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W<16> {
        RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FFTHR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffthr](index.html) module"]
pub struct FFTHR_SPEC;
impl crate::RegisterSpec for FFTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffthr::R](R) reader structure"]
impl crate::Readable for FFTHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffthr::W](W) writer structure"]
impl crate::Writable for FFTHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFTHR to value 0"]
impl crate::Resettable for FFTHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
