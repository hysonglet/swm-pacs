#[doc = "Register `TOCR` reader"]
pub struct R(crate::R<TOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCR` writer"]
pub struct W(crate::W<TOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCR_SPEC>;
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
impl From<crate::W<TOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - TIME field"]
pub type TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIME` writer - TIME field"]
pub type TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `MODE` reader - MODE field"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - MODE field"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOCR_SPEC, bool, O>;
#[doc = "Field `IFCLR` reader - IFCLR field"]
pub type IFCLR_R = crate::BitReader<bool>;
#[doc = "Field `IFCLR` writer - IFCLR field"]
pub type IFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - TIME field"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - MODE field"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IFCLR field"]
    #[inline(always)]
    pub fn ifclr(&self) -> IFCLR_R {
        IFCLR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIME field"]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W<0> {
        TIME_W::new(self)
    }
    #[doc = "Bit 12 - MODE field"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<12> {
        MODE_W::new(self)
    }
    #[doc = "Bit 13 - IFCLR field"]
    #[inline(always)]
    pub fn ifclr(&mut self) -> IFCLR_W<13> {
        IFCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TOCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocr](index.html) module"]
pub struct TOCR_SPEC;
impl crate::RegisterSpec for TOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tocr::R](R) reader structure"]
impl crate::Readable for TOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocr::W](W) writer structure"]
impl crate::Writable for TOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOCR to value 0"]
impl crate::Resettable for TOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
