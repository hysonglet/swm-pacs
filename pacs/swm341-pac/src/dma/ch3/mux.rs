#[doc = "Register `MUX` reader"]
pub struct R(crate::R<MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX` writer"]
pub struct W(crate::W<MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX_SPEC>;
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
impl From<crate::W<MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTHSSIG` reader - DSTHSSIG field"]
pub type DSTHSSIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSTHSSIG` writer - DSTHSSIG field"]
pub type DSTHSSIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SPEC, u8, u8, 2, O>;
#[doc = "Field `DSTHSEN` reader - DSTHSEN field"]
pub type DSTHSEN_R = crate::BitReader<bool>;
#[doc = "Field `DSTHSEN` writer - DSTHSEN field"]
pub type DSTHSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SPEC, bool, O>;
#[doc = "Field `SRCHSSIG` reader - SRCHSSIG field"]
pub type SRCHSSIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCHSSIG` writer - SRCHSSIG field"]
pub type SRCHSSIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SPEC, u8, u8, 2, O>;
#[doc = "Field `SRCHSEN` reader - SRCHSEN field"]
pub type SRCHSEN_R = crate::BitReader<bool>;
#[doc = "Field `SRCHSEN` writer - SRCHSEN field"]
pub type SRCHSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SPEC, bool, O>;
#[doc = "Field `EXTHSSIG` reader - EXTHSSIG field"]
pub type EXTHSSIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTHSSIG` writer - EXTHSSIG field"]
pub type EXTHSSIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `EXTHSEN` reader - EXTHSEN field"]
pub type EXTHSEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTHSEN` writer - EXTHSEN field"]
pub type EXTHSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - DSTHSSIG field"]
    #[inline(always)]
    pub fn dsthssig(&self) -> DSTHSSIG_R {
        DSTHSSIG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DSTHSEN field"]
    #[inline(always)]
    pub fn dsthsen(&self) -> DSTHSEN_R {
        DSTHSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SRCHSSIG field"]
    #[inline(always)]
    pub fn srchssig(&self) -> SRCHSSIG_R {
        SRCHSSIG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SRCHSEN field"]
    #[inline(always)]
    pub fn srchsen(&self) -> SRCHSEN_R {
        SRCHSEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:18 - EXTHSSIG field"]
    #[inline(always)]
    pub fn exthssig(&self) -> EXTHSSIG_R {
        EXTHSSIG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - EXTHSEN field"]
    #[inline(always)]
    pub fn exthsen(&self) -> EXTHSEN_R {
        EXTHSEN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DSTHSSIG field"]
    #[inline(always)]
    pub fn dsthssig(&mut self) -> DSTHSSIG_W<0> {
        DSTHSSIG_W::new(self)
    }
    #[doc = "Bit 2 - DSTHSEN field"]
    #[inline(always)]
    pub fn dsthsen(&mut self) -> DSTHSEN_W<2> {
        DSTHSEN_W::new(self)
    }
    #[doc = "Bits 8:9 - SRCHSSIG field"]
    #[inline(always)]
    pub fn srchssig(&mut self) -> SRCHSSIG_W<8> {
        SRCHSSIG_W::new(self)
    }
    #[doc = "Bit 10 - SRCHSEN field"]
    #[inline(always)]
    pub fn srchsen(&mut self) -> SRCHSEN_W<10> {
        SRCHSEN_W::new(self)
    }
    #[doc = "Bits 16:18 - EXTHSSIG field"]
    #[inline(always)]
    pub fn exthssig(&mut self) -> EXTHSSIG_W<16> {
        EXTHSSIG_W::new(self)
    }
    #[doc = "Bit 19 - EXTHSEN field"]
    #[inline(always)]
    pub fn exthsen(&mut self) -> EXTHSEN_W<19> {
        EXTHSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MUX register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux](index.html) module"]
pub struct MUX_SPEC;
impl crate::RegisterSpec for MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux::R](R) reader structure"]
impl crate::Readable for MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux::W](W) writer structure"]
impl crate::Writable for MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX to value 0"]
impl crate::Resettable for MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
