#[doc = "Register `DEVIF` reader"]
pub struct R(crate::R<DEVIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVIF` writer"]
pub struct W(crate::W<DEVIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVIF_SPEC>;
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
impl From<crate::W<DEVIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETCFG` reader - SETCFG field"]
pub type SETCFG_R = crate::BitReader<bool>;
#[doc = "Field `SETCFG` writer - SETCFG field"]
pub type SETCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVIF_SPEC, bool, O>;
#[doc = "Field `SETINTF` reader - SETINTF field"]
pub type SETINTF_R = crate::BitReader<bool>;
#[doc = "Field `SETINTF` writer - SETINTF field"]
pub type SETINTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVIF_SPEC, bool, O>;
#[doc = "Field `RST` reader - RST field"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - RST field"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVIF_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - SUSP field"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - SUSP field"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVIF_SPEC, bool, O>;
#[doc = "Field `SOF` reader - SOF field"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - SOF field"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVIF_SPEC, bool, O>;
#[doc = "Field `SETUP` reader - SETUP field"]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - SETUP field"]
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVIF_SPEC, bool, O>;
#[doc = "Field `OUT` reader - OUT field"]
pub type OUT_R = crate::BitReader<bool>;
#[doc = "Field `OUT` writer - OUT field"]
pub type OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVIF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SETCFG field"]
    #[inline(always)]
    pub fn setcfg(&self) -> SETCFG_R {
        SETCFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SETINTF field"]
    #[inline(always)]
    pub fn setintf(&self) -> SETINTF_R {
        SETINTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RST field"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUSP field"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SOF field"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SETUP field"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OUT field"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SETCFG field"]
    #[inline(always)]
    pub fn setcfg(&mut self) -> SETCFG_W<0> {
        SETCFG_W::new(self)
    }
    #[doc = "Bit 1 - SETINTF field"]
    #[inline(always)]
    pub fn setintf(&mut self) -> SETINTF_W<1> {
        SETINTF_W::new(self)
    }
    #[doc = "Bit 3 - RST field"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<3> {
        RST_W::new(self)
    }
    #[doc = "Bit 4 - SUSP field"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<4> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 5 - SOF field"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<5> {
        SOF_W::new(self)
    }
    #[doc = "Bit 6 - SETUP field"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W<6> {
        SETUP_W::new(self)
    }
    #[doc = "Bit 7 - OUT field"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W<7> {
        OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVIF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devif](index.html) module"]
pub struct DEVIF_SPEC;
impl crate::RegisterSpec for DEVIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devif::R](R) reader structure"]
impl crate::Readable for DEVIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devif::W](W) writer structure"]
impl crate::Writable for DEVIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVIF to value 0"]
impl crate::Resettable for DEVIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
