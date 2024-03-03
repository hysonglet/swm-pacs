#[doc = "Register `ERASE` reader"]
pub struct R(crate::R<ERASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASE` writer"]
pub struct W(crate::W<ERASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASE_SPEC>;
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
impl From<crate::W<ERASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - ADDR field"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - ADDR field"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ERASE_SPEC, u32, u32, 19, O>;
#[doc = "Field `REQ` reader - REQ field"]
pub type REQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REQ` writer - REQ field"]
pub type REQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ERASE_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:18 - ADDR field"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 27:31 - REQ field"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - ADDR field"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 27:31 - REQ field"]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W<27> {
        REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ERASE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erase](index.html) module"]
pub struct ERASE_SPEC;
impl crate::RegisterSpec for ERASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erase::R](R) reader structure"]
impl crate::Readable for ERASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erase::W](W) writer structure"]
impl crate::Writable for ERASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASE to value 0"]
impl crate::Resettable for ERASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
