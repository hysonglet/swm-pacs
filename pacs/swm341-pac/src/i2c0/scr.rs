#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR10` reader - ADDR10 field"]
pub type ADDR10_R = crate::BitReader<bool>;
#[doc = "Field `ADDR10` writer - ADDR10 field"]
pub type ADDR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `MCDE` reader - MCDE field"]
pub type MCDE_R = crate::BitReader<bool>;
#[doc = "Field `MCDE` writer - MCDE field"]
pub type MCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `STRE` reader - STRE field"]
pub type STRE_R = crate::BitReader<bool>;
#[doc = "Field `STRE` writer - STRE field"]
pub type STRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `ASDS` reader - ASDS field"]
pub type ASDS_R = crate::BitReader<bool>;
#[doc = "Field `ASDS` writer - ASDS field"]
pub type ASDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADDR10 field"]
    #[inline(always)]
    pub fn addr10(&self) -> ADDR10_R {
        ADDR10_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCDE field"]
    #[inline(always)]
    pub fn mcde(&self) -> MCDE_R {
        MCDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STRE field"]
    #[inline(always)]
    pub fn stre(&self) -> STRE_R {
        STRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ASDS field"]
    #[inline(always)]
    pub fn asds(&self) -> ASDS_R {
        ASDS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADDR10 field"]
    #[inline(always)]
    pub fn addr10(&mut self) -> ADDR10_W<0> {
        ADDR10_W::new(self)
    }
    #[doc = "Bit 1 - MCDE field"]
    #[inline(always)]
    pub fn mcde(&mut self) -> MCDE_W<1> {
        MCDE_W::new(self)
    }
    #[doc = "Bit 2 - STRE field"]
    #[inline(always)]
    pub fn stre(&mut self) -> STRE_W<2> {
        STRE_W::new(self)
    }
    #[doc = "Bit 3 - ASDS field"]
    #[inline(always)]
    pub fn asds(&mut self) -> ASDS_W<3> {
        ASDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
