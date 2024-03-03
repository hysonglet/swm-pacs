#[doc = "Register `MPUCR` reader"]
pub struct R(crate::R<MPUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPUCR` writer"]
pub struct W(crate::W<MPUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPUCR_SPEC>;
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
impl From<crate::W<MPUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCS1_0` reader - RCS1_0 field"]
pub type RCS1_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCS1_0` writer - RCS1_0 field"]
pub type RCS1_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPUCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `RDHOLD` reader - RDHOLD field"]
pub type RDHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDHOLD` writer - RDHOLD field"]
pub type RDHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPUCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `WCS1_0` reader - WCS1_0 field"]
pub type WCS1_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WCS1_0` writer - WCS1_0 field"]
pub type WCS1_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPUCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `WR1CS1` reader - WR1CS1 field"]
pub type WR1CS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR1CS1` writer - WR1CS1 field"]
pub type WR1CS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPUCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WRHOLD` reader - WRHOLD field"]
pub type WRHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRHOLD` writer - WRHOLD field"]
pub type WRHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPUCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CS0WR0` reader - CS0WR0 field"]
pub type CS0WR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS0WR0` writer - CS0WR0 field"]
pub type CS0WR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPUCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4 - RCS1_0 field"]
    #[inline(always)]
    pub fn rcs1_0(&self) -> RCS1_0_R {
        RCS1_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - RDHOLD field"]
    #[inline(always)]
    pub fn rdhold(&self) -> RDHOLD_R {
        RDHOLD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:13 - WCS1_0 field"]
    #[inline(always)]
    pub fn wcs1_0(&self) -> WCS1_0_R {
        WCS1_0_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - WR1CS1 field"]
    #[inline(always)]
    pub fn wr1cs1(&self) -> WR1CS1_R {
        WR1CS1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - WRHOLD field"]
    #[inline(always)]
    pub fn wrhold(&self) -> WRHOLD_R {
        WRHOLD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - CS0WR0 field"]
    #[inline(always)]
    pub fn cs0wr0(&self) -> CS0WR0_R {
        CS0WR0_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - RCS1_0 field"]
    #[inline(always)]
    pub fn rcs1_0(&mut self) -> RCS1_0_W<0> {
        RCS1_0_W::new(self)
    }
    #[doc = "Bits 5:9 - RDHOLD field"]
    #[inline(always)]
    pub fn rdhold(&mut self) -> RDHOLD_W<5> {
        RDHOLD_W::new(self)
    }
    #[doc = "Bits 10:13 - WCS1_0 field"]
    #[inline(always)]
    pub fn wcs1_0(&mut self) -> WCS1_0_W<10> {
        WCS1_0_W::new(self)
    }
    #[doc = "Bits 14:15 - WR1CS1 field"]
    #[inline(always)]
    pub fn wr1cs1(&mut self) -> WR1CS1_W<14> {
        WR1CS1_W::new(self)
    }
    #[doc = "Bits 16:19 - WRHOLD field"]
    #[inline(always)]
    pub fn wrhold(&mut self) -> WRHOLD_W<16> {
        WRHOLD_W::new(self)
    }
    #[doc = "Bits 20:21 - CS0WR0 field"]
    #[inline(always)]
    pub fn cs0wr0(&mut self) -> CS0WR0_W<20> {
        CS0WR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPUCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpucr](index.html) module"]
pub struct MPUCR_SPEC;
impl crate::RegisterSpec for MPUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpucr::R](R) reader structure"]
impl crate::Readable for MPUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpucr::W](W) writer structure"]
impl crate::Writable for MPUCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPUCR to value 0"]
impl crate::Resettable for MPUCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
