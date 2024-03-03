#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - RESET field"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET field"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
#[doc = "Field `START` reader - START field"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START field"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - BUSY field"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY field"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
#[doc = "Field `LOAD` reader - LOAD field"]
pub type LOAD_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` writer - LOAD field"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
#[doc = "Field `BYPASS` reader - BYPASS field"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - BYPASS field"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
#[doc = "Field `RESULT` reader - RESULT field"]
pub type RESULT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESULT` writer - RESULT field"]
pub type RESULT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALIB_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - RESET field"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - START field"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BUSY field"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LOAD field"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BYPASS field"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RESULT field"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RESET field"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Bit 1 - START field"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Bit 2 - BUSY field"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<2> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 3 - LOAD field"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<3> {
        LOAD_W::new(self)
    }
    #[doc = "Bit 4 - BYPASS field"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<4> {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 8:14 - RESULT field"]
    #[inline(always)]
    pub fn result(&mut self) -> RESULT_W<8> {
        RESULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CALIB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
