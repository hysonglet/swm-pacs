#[doc = "Register `SADDR` reader"]
pub struct R(crate::R<SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDR` writer"]
pub struct W(crate::W<SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDR_SPEC>;
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
impl From<crate::W<SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR7` reader - ADDR7 field"]
pub type ADDR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR7` writer - ADDR7 field"]
pub type ADDR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `MASK7` reader - MASK7 field"]
pub type MASK7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK7` writer - MASK7 field"]
pub type MASK7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SADDR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 1:7 - ADDR7 field"]
    #[inline(always)]
    pub fn addr7(&self) -> ADDR7_R {
        ADDR7_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 17:23 - MASK7 field"]
    #[inline(always)]
    pub fn mask7(&self) -> MASK7_R {
        MASK7_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - ADDR7 field"]
    #[inline(always)]
    pub fn addr7(&mut self) -> ADDR7_W<1> {
        ADDR7_W::new(self)
    }
    #[doc = "Bits 17:23 - MASK7 field"]
    #[inline(always)]
    pub fn mask7(&mut self) -> MASK7_W<17> {
        MASK7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr](index.html) module"]
pub struct SADDR_SPEC;
impl crate::RegisterSpec for SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saddr::R](R) reader structure"]
impl crate::Readable for SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddr::W](W) writer structure"]
impl crate::Writable for SADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADDR to value 0"]
impl crate::Resettable for SADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
