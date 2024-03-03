#[doc = "Register `TOKEN` reader"]
pub struct R(crate::R<TOKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOKEN` writer"]
pub struct W(crate::W<TOKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOKEN_SPEC>;
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
impl From<crate::W<TOKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - ADDR field"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - ADDR field"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOKEN_SPEC, u8, u8, 7, O>;
#[doc = "Field `EPNR` reader - EPNR field"]
pub type EPNR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNR` writer - EPNR field"]
pub type EPNR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOKEN_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATAX` reader - DATAX field"]
pub type DATAX_R = crate::BitReader<bool>;
#[doc = "Field `DATAX` writer - DATAX field"]
pub type DATAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOKEN_SPEC, bool, O>;
#[doc = "Field `TYPE` reader - TYPE field"]
pub type TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TYPE` writer - TYPE field"]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOKEN_SPEC, u8, u8, 4, O>;
#[doc = "Field `ISO` reader - ISO field"]
pub type ISO_R = crate::BitReader<bool>;
#[doc = "Field `ISO` writer - ISO field"]
pub type ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOKEN_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - SPEED field"]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPEED` writer - SPEED field"]
pub type SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOKEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRSZ` reader - TRSZ field"]
pub type TRSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRSZ` writer - TRSZ field"]
pub type TRSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOKEN_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:6 - ADDR field"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - EPNR field"]
    #[inline(always)]
    pub fn epnr(&self) -> EPNR_R {
        EPNR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - DATAX field"]
    #[inline(always)]
    pub fn datax(&self) -> DATAX_R {
        DATAX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 13:16 - TYPE field"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - ISO field"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - SPEED field"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:29 - TRSZ field"]
    #[inline(always)]
    pub fn trsz(&self) -> TRSZ_R {
        TRSZ_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADDR field"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 7:10 - EPNR field"]
    #[inline(always)]
    pub fn epnr(&mut self) -> EPNR_W<7> {
        EPNR_W::new(self)
    }
    #[doc = "Bit 11 - DATAX field"]
    #[inline(always)]
    pub fn datax(&mut self) -> DATAX_W<11> {
        DATAX_W::new(self)
    }
    #[doc = "Bits 13:16 - TYPE field"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<13> {
        TYPE_W::new(self)
    }
    #[doc = "Bit 17 - ISO field"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W<17> {
        ISO_W::new(self)
    }
    #[doc = "Bits 18:19 - SPEED field"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W<18> {
        SPEED_W::new(self)
    }
    #[doc = "Bits 20:29 - TRSZ field"]
    #[inline(always)]
    pub fn trsz(&mut self) -> TRSZ_W<20> {
        TRSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TOKEN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [token](index.html) module"]
pub struct TOKEN_SPEC;
impl crate::RegisterSpec for TOKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [token::R](R) reader structure"]
impl crate::Readable for TOKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [token::W](W) writer structure"]
impl crate::Writable for TOKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOKEN to value 0"]
impl crate::Resettable for TOKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
