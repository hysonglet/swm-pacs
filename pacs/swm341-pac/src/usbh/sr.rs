#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESP` reader - RESP field"]
pub type RESP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESP` writer - RESP field"]
pub type RESP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRSZ` reader - TRSZ field"]
pub type TRSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRSZ` writer - TRSZ field"]
pub type TRSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:3 - RESP field"]
    #[inline(always)]
    pub fn resp(&self) -> RESP_R {
        RESP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:13 - TRSZ field"]
    #[inline(always)]
    pub fn trsz(&self) -> TRSZ_R {
        TRSZ_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - RESP field"]
    #[inline(always)]
    pub fn resp(&mut self) -> RESP_W<0> {
        RESP_W::new(self)
    }
    #[doc = "Bits 4:13 - TRSZ field"]
    #[inline(always)]
    pub fn trsz(&mut self) -> TRSZ_W<4> {
        TRSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
