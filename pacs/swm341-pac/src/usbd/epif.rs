#[doc = "Register `EPIF` reader"]
pub struct R(crate::R<EPIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPIF` writer"]
pub struct W(crate::W<EPIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPIF_SPEC>;
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
impl From<crate::W<EPIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEP0` reader - INEP0 field"]
pub type INEP0_R = crate::BitReader<bool>;
#[doc = "Field `INEP0` writer - INEP0 field"]
pub type INEP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `INEP1` reader - INEP1 field"]
pub type INEP1_R = crate::BitReader<bool>;
#[doc = "Field `INEP1` writer - INEP1 field"]
pub type INEP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `INEP2` reader - INEP2 field"]
pub type INEP2_R = crate::BitReader<bool>;
#[doc = "Field `INEP2` writer - INEP2 field"]
pub type INEP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `INEP3` reader - INEP3 field"]
pub type INEP3_R = crate::BitReader<bool>;
#[doc = "Field `INEP3` writer - INEP3 field"]
pub type INEP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `INEP4` reader - INEP4 field"]
pub type INEP4_R = crate::BitReader<bool>;
#[doc = "Field `INEP4` writer - INEP4 field"]
pub type INEP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `INEP5` reader - INEP5 field"]
pub type INEP5_R = crate::BitReader<bool>;
#[doc = "Field `INEP5` writer - INEP5 field"]
pub type INEP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `INEP6` reader - INEP6 field"]
pub type INEP6_R = crate::BitReader<bool>;
#[doc = "Field `INEP6` writer - INEP6 field"]
pub type INEP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `INEP7` reader - INEP7 field"]
pub type INEP7_R = crate::BitReader<bool>;
#[doc = "Field `INEP7` writer - INEP7 field"]
pub type INEP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `OUTEP0` reader - OUTEP0 field"]
pub type OUTEP0_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP0` writer - OUTEP0 field"]
pub type OUTEP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `OUTEP1` reader - OUTEP1 field"]
pub type OUTEP1_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP1` writer - OUTEP1 field"]
pub type OUTEP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `OUTEP2` reader - OUTEP2 field"]
pub type OUTEP2_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP2` writer - OUTEP2 field"]
pub type OUTEP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `OUTEP3` reader - OUTEP3 field"]
pub type OUTEP3_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP3` writer - OUTEP3 field"]
pub type OUTEP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `OUTEP4` reader - OUTEP4 field"]
pub type OUTEP4_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP4` writer - OUTEP4 field"]
pub type OUTEP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `OUTEP5` reader - OUTEP5 field"]
pub type OUTEP5_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP5` writer - OUTEP5 field"]
pub type OUTEP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `OUTEP6` reader - OUTEP6 field"]
pub type OUTEP6_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP6` writer - OUTEP6 field"]
pub type OUTEP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
#[doc = "Field `OUTEP7` reader - OUTEP7 field"]
pub type OUTEP7_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP7` writer - OUTEP7 field"]
pub type OUTEP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPIF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - INEP0 field"]
    #[inline(always)]
    pub fn inep0(&self) -> INEP0_R {
        INEP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INEP1 field"]
    #[inline(always)]
    pub fn inep1(&self) -> INEP1_R {
        INEP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INEP2 field"]
    #[inline(always)]
    pub fn inep2(&self) -> INEP2_R {
        INEP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INEP3 field"]
    #[inline(always)]
    pub fn inep3(&self) -> INEP3_R {
        INEP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INEP4 field"]
    #[inline(always)]
    pub fn inep4(&self) -> INEP4_R {
        INEP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INEP5 field"]
    #[inline(always)]
    pub fn inep5(&self) -> INEP5_R {
        INEP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INEP6 field"]
    #[inline(always)]
    pub fn inep6(&self) -> INEP6_R {
        INEP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INEP7 field"]
    #[inline(always)]
    pub fn inep7(&self) -> INEP7_R {
        INEP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - OUTEP0 field"]
    #[inline(always)]
    pub fn outep0(&self) -> OUTEP0_R {
        OUTEP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OUTEP1 field"]
    #[inline(always)]
    pub fn outep1(&self) -> OUTEP1_R {
        OUTEP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OUTEP2 field"]
    #[inline(always)]
    pub fn outep2(&self) -> OUTEP2_R {
        OUTEP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUTEP3 field"]
    #[inline(always)]
    pub fn outep3(&self) -> OUTEP3_R {
        OUTEP3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OUTEP4 field"]
    #[inline(always)]
    pub fn outep4(&self) -> OUTEP4_R {
        OUTEP4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OUTEP5 field"]
    #[inline(always)]
    pub fn outep5(&self) -> OUTEP5_R {
        OUTEP5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OUTEP6 field"]
    #[inline(always)]
    pub fn outep6(&self) -> OUTEP6_R {
        OUTEP6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OUTEP7 field"]
    #[inline(always)]
    pub fn outep7(&self) -> OUTEP7_R {
        OUTEP7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INEP0 field"]
    #[inline(always)]
    pub fn inep0(&mut self) -> INEP0_W<0> {
        INEP0_W::new(self)
    }
    #[doc = "Bit 1 - INEP1 field"]
    #[inline(always)]
    pub fn inep1(&mut self) -> INEP1_W<1> {
        INEP1_W::new(self)
    }
    #[doc = "Bit 2 - INEP2 field"]
    #[inline(always)]
    pub fn inep2(&mut self) -> INEP2_W<2> {
        INEP2_W::new(self)
    }
    #[doc = "Bit 3 - INEP3 field"]
    #[inline(always)]
    pub fn inep3(&mut self) -> INEP3_W<3> {
        INEP3_W::new(self)
    }
    #[doc = "Bit 4 - INEP4 field"]
    #[inline(always)]
    pub fn inep4(&mut self) -> INEP4_W<4> {
        INEP4_W::new(self)
    }
    #[doc = "Bit 5 - INEP5 field"]
    #[inline(always)]
    pub fn inep5(&mut self) -> INEP5_W<5> {
        INEP5_W::new(self)
    }
    #[doc = "Bit 6 - INEP6 field"]
    #[inline(always)]
    pub fn inep6(&mut self) -> INEP6_W<6> {
        INEP6_W::new(self)
    }
    #[doc = "Bit 7 - INEP7 field"]
    #[inline(always)]
    pub fn inep7(&mut self) -> INEP7_W<7> {
        INEP7_W::new(self)
    }
    #[doc = "Bit 16 - OUTEP0 field"]
    #[inline(always)]
    pub fn outep0(&mut self) -> OUTEP0_W<16> {
        OUTEP0_W::new(self)
    }
    #[doc = "Bit 17 - OUTEP1 field"]
    #[inline(always)]
    pub fn outep1(&mut self) -> OUTEP1_W<17> {
        OUTEP1_W::new(self)
    }
    #[doc = "Bit 18 - OUTEP2 field"]
    #[inline(always)]
    pub fn outep2(&mut self) -> OUTEP2_W<18> {
        OUTEP2_W::new(self)
    }
    #[doc = "Bit 19 - OUTEP3 field"]
    #[inline(always)]
    pub fn outep3(&mut self) -> OUTEP3_W<19> {
        OUTEP3_W::new(self)
    }
    #[doc = "Bit 20 - OUTEP4 field"]
    #[inline(always)]
    pub fn outep4(&mut self) -> OUTEP4_W<20> {
        OUTEP4_W::new(self)
    }
    #[doc = "Bit 21 - OUTEP5 field"]
    #[inline(always)]
    pub fn outep5(&mut self) -> OUTEP5_W<21> {
        OUTEP5_W::new(self)
    }
    #[doc = "Bit 22 - OUTEP6 field"]
    #[inline(always)]
    pub fn outep6(&mut self) -> OUTEP6_W<22> {
        OUTEP6_W::new(self)
    }
    #[doc = "Bit 23 - OUTEP7 field"]
    #[inline(always)]
    pub fn outep7(&mut self) -> OUTEP7_W<23> {
        OUTEP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPIF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epif](index.html) module"]
pub struct EPIF_SPEC;
impl crate::RegisterSpec for EPIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epif::R](R) reader structure"]
impl crate::Readable for EPIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epif::W](W) writer structure"]
impl crate::Writable for EPIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPIF to value 0"]
impl crate::Resettable for EPIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
