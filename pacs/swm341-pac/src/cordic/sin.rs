#[doc = "Register `SIN` reader"]
pub struct R(crate::R<SIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIN` writer"]
pub struct W(crate::W<SIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIN_SPEC>;
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
impl From<crate::W<SIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F` reader - F field"]
pub type F_R = crate::FieldReader<u16, u16>;
#[doc = "Field `F` writer - F field"]
pub type F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIN_SPEC, u16, u16, 14, O>;
#[doc = "Field `I` reader - I field"]
pub type I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I` writer - I field"]
pub type I_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIN_SPEC, u8, u8, 2, O>;
#[doc = "Field `DONE` reader - DONE field"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - DONE field"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - F field"]
    #[inline(always)]
    pub fn f(&self) -> F_R {
        F_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - I field"]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - DONE field"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - F field"]
    #[inline(always)]
    pub fn f(&mut self) -> F_W<0> {
        F_W::new(self)
    }
    #[doc = "Bits 14:15 - I field"]
    #[inline(always)]
    pub fn i(&mut self) -> I_W<14> {
        I_W::new(self)
    }
    #[doc = "Bit 16 - DONE field"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<16> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SIN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sin](index.html) module"]
pub struct SIN_SPEC;
impl crate::RegisterSpec for SIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sin::R](R) reader structure"]
impl crate::Readable for SIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sin::W](W) writer structure"]
impl crate::Writable for SIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIN to value 0"]
impl crate::Resettable for SIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
