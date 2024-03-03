#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXSTAT` reader - RXSTAT field"]
pub type RXSTAT_R = crate::BitReader<bool>;
#[doc = "Field `RXSTAT` writer - RXSTAT field"]
pub type RXSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `ABVTHR` reader - ABVTHR field"]
pub type ABVTHR_R = crate::BitReader<bool>;
#[doc = "Field `ABVTHR` writer - ABVTHR field"]
pub type ABVTHR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `BLWTHR` reader - BLWTHR field"]
pub type BLWTHR_R = crate::BitReader<bool>;
#[doc = "Field `BLWTHR` writer - BLWTHR field"]
pub type BLWTHR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `SOF` reader - SOF field"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - SOF field"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `PORT` reader - PORT field"]
pub type PORT_R = crate::BitReader<bool>;
#[doc = "Field `PORT` writer - PORT field"]
pub type PORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `OTG` reader - OTG field"]
pub type OTG_R = crate::BitReader<bool>;
#[doc = "Field `OTG` writer - OTG field"]
pub type OTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXSTAT field"]
    #[inline(always)]
    pub fn rxstat(&self) -> RXSTAT_R {
        RXSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ABVTHR field"]
    #[inline(always)]
    pub fn abvthr(&self) -> ABVTHR_R {
        ABVTHR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BLWTHR field"]
    #[inline(always)]
    pub fn blwthr(&self) -> BLWTHR_R {
        BLWTHR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF field"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - PORT field"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OTG field"]
    #[inline(always)]
    pub fn otg(&self) -> OTG_R {
        OTG_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXSTAT field"]
    #[inline(always)]
    pub fn rxstat(&mut self) -> RXSTAT_W<0> {
        RXSTAT_W::new(self)
    }
    #[doc = "Bit 1 - ABVTHR field"]
    #[inline(always)]
    pub fn abvthr(&mut self) -> ABVTHR_W<1> {
        ABVTHR_W::new(self)
    }
    #[doc = "Bit 2 - BLWTHR field"]
    #[inline(always)]
    pub fn blwthr(&mut self) -> BLWTHR_W<2> {
        BLWTHR_W::new(self)
    }
    #[doc = "Bit 3 - SOF field"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<3> {
        SOF_W::new(self)
    }
    #[doc = "Bit 8 - PORT field"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W<8> {
        PORT_W::new(self)
    }
    #[doc = "Bit 9 - OTG field"]
    #[inline(always)]
    pub fn otg(&mut self) -> OTG_W<9> {
        OTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
