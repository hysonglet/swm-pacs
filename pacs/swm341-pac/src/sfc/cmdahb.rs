#[doc = "Register `CMDAHB` reader"]
pub struct R(crate::R<CMDAHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDAHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDAHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDAHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDAHB` writer"]
pub struct W(crate::W<CMDAHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDAHB_SPEC>;
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
impl From<crate::W<CMDAHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDAHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WREN` reader - WREN field"]
pub type WREN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WREN` writer - WREN field"]
pub type WREN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDAHB_SPEC, u8, u8, 8, O>;
#[doc = "Field `PP` reader - PP field"]
pub type PP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PP` writer - PP field"]
pub type PP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDAHB_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDSRL` reader - RDSRL field"]
pub type RDSRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDSRL` writer - RDSRL field"]
pub type RDSRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDAHB_SPEC, u8, u8, 8, O>;
#[doc = "Field `READ` reader - READ field"]
pub type READ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `READ` writer - READ field"]
pub type READ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDAHB_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - WREN field"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PP field"]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RDSRL field"]
    #[inline(always)]
    pub fn rdsrl(&self) -> RDSRL_R {
        RDSRL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - READ field"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WREN field"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<0> {
        WREN_W::new(self)
    }
    #[doc = "Bits 8:15 - PP field"]
    #[inline(always)]
    pub fn pp(&mut self) -> PP_W<8> {
        PP_W::new(self)
    }
    #[doc = "Bits 16:23 - RDSRL field"]
    #[inline(always)]
    pub fn rdsrl(&mut self) -> RDSRL_W<16> {
        RDSRL_W::new(self)
    }
    #[doc = "Bits 24:31 - READ field"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W<24> {
        READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMDAHB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdahb](index.html) module"]
pub struct CMDAHB_SPEC;
impl crate::RegisterSpec for CMDAHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdahb::R](R) reader structure"]
impl crate::Readable for CMDAHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdahb::W](W) writer structure"]
impl crate::Writable for CMDAHB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDAHB to value 0"]
impl crate::Resettable for CMDAHB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
