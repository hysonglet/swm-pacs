#[doc = "Register `EPCFG[%s]` reader"]
pub struct R(crate::R<EPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPCFG[%s]` writer"]
pub struct W(crate::W<EPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPCFG_SPEC>;
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
impl From<crate::W<EPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPNR` reader - EPNR field"]
pub type EPNR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNR` writer - EPNR field"]
pub type EPNR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIR` reader - DIR field"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - DIR field"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPCFG_SPEC, bool, O>;
#[doc = "Field `TYPE` reader - TYPE field"]
pub type TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TYPE` writer - TYPE field"]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CFG` reader - CFG field"]
pub type CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG` writer - CFG field"]
pub type CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `INTF` reader - INTF field"]
pub type INTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTF` writer - INTF field"]
pub type INTF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALT` reader - ALT field"]
pub type ALT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALT` writer - ALT field"]
pub type ALT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `PKSZ` reader - PKSZ field"]
pub type PKSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKSZ` writer - PKSZ field"]
pub type PKSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPCFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - EPNR field"]
    #[inline(always)]
    pub fn epnr(&self) -> EPNR_R {
        EPNR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - DIR field"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - TYPE field"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:10 - CFG field"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - INTF field"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - ALT field"]
    #[inline(always)]
    pub fn alt(&self) -> ALT_R {
        ALT_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:26 - PKSZ field"]
    #[inline(always)]
    pub fn pksz(&self) -> PKSZ_R {
        PKSZ_R::new(((self.bits >> 19) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EPNR field"]
    #[inline(always)]
    pub fn epnr(&mut self) -> EPNR_W<0> {
        EPNR_W::new(self)
    }
    #[doc = "Bit 4 - DIR field"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bits 5:6 - TYPE field"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<5> {
        TYPE_W::new(self)
    }
    #[doc = "Bits 7:10 - CFG field"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W<7> {
        CFG_W::new(self)
    }
    #[doc = "Bits 11:14 - INTF field"]
    #[inline(always)]
    pub fn intf(&mut self) -> INTF_W<11> {
        INTF_W::new(self)
    }
    #[doc = "Bits 15:18 - ALT field"]
    #[inline(always)]
    pub fn alt(&mut self) -> ALT_W<15> {
        ALT_W::new(self)
    }
    #[doc = "Bits 19:26 - PKSZ field"]
    #[inline(always)]
    pub fn pksz(&mut self) -> PKSZ_W<19> {
        PKSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPCFG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcfg](index.html) module"]
pub struct EPCFG_SPEC;
impl crate::RegisterSpec for EPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epcfg::R](R) reader structure"]
impl crate::Readable for EPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epcfg::W](W) writer structure"]
impl crate::Writable for EPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPCFG[%s]
to value 0"]
impl crate::Resettable for EPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
