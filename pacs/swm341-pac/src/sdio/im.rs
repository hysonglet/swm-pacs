#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDDONE` reader - CMDDONE field"]
pub type CMDDONE_R = crate::BitReader<bool>;
#[doc = "Field `CMDDONE` writer - CMDDONE field"]
pub type CMDDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `TRXDONE` reader - TRXDONE field"]
pub type TRXDONE_R = crate::BitReader<bool>;
#[doc = "Field `TRXDONE` writer - TRXDONE field"]
pub type TRXDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `BLKGAP` reader - BLKGAP field"]
pub type BLKGAP_R = crate::BitReader<bool>;
#[doc = "Field `BLKGAP` writer - BLKGAP field"]
pub type BLKGAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `DMADONE` reader - DMADONE field"]
pub type DMADONE_R = crate::BitReader<bool>;
#[doc = "Field `DMADONE` writer - DMADONE field"]
pub type DMADONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `BUFWRRDY` reader - BUFWRRDY field"]
pub type BUFWRRDY_R = crate::BitReader<bool>;
#[doc = "Field `BUFWRRDY` writer - BUFWRRDY field"]
pub type BUFWRRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `BUFRDRDY` reader - BUFRDRDY field"]
pub type BUFRDRDY_R = crate::BitReader<bool>;
#[doc = "Field `BUFRDRDY` writer - BUFRDRDY field"]
pub type BUFRDRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CARDINSR` reader - CARDINSR field"]
pub type CARDINSR_R = crate::BitReader<bool>;
#[doc = "Field `CARDINSR` writer - CARDINSR field"]
pub type CARDINSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CARDRMOV` reader - CARDRMOV field"]
pub type CARDRMOV_R = crate::BitReader<bool>;
#[doc = "Field `CARDRMOV` writer - CARDRMOV field"]
pub type CARDRMOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CARD` reader - CARD field"]
pub type CARD_R = crate::BitReader<bool>;
#[doc = "Field `CARD` writer - CARD field"]
pub type CARD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CMDTIMEOUT` reader - CMDTIMEOUT field"]
pub type CMDTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `CMDTIMEOUT` writer - CMDTIMEOUT field"]
pub type CMDTIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CMDCRCERR` reader - CMDCRCERR field"]
pub type CMDCRCERR_R = crate::BitReader<bool>;
#[doc = "Field `CMDCRCERR` writer - CMDCRCERR field"]
pub type CMDCRCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CMDENDERR` reader - CMDENDERR field"]
pub type CMDENDERR_R = crate::BitReader<bool>;
#[doc = "Field `CMDENDERR` writer - CMDENDERR field"]
pub type CMDENDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CMDIDXERR` reader - CMDIDXERR field"]
pub type CMDIDXERR_R = crate::BitReader<bool>;
#[doc = "Field `CMDIDXERR` writer - CMDIDXERR field"]
pub type CMDIDXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `DATTIMEOUT` reader - DATTIMEOUT field"]
pub type DATTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `DATTIMEOUT` writer - DATTIMEOUT field"]
pub type DATTIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `DATCRCERR` reader - DATCRCERR field"]
pub type DATCRCERR_R = crate::BitReader<bool>;
#[doc = "Field `DATCRCERR` writer - DATCRCERR field"]
pub type DATCRCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `DATENDERR` reader - DATENDERR field"]
pub type DATENDERR_R = crate::BitReader<bool>;
#[doc = "Field `DATENDERR` writer - DATENDERR field"]
pub type DATENDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CURLIMERR` reader - CURLIMERR field"]
pub type CURLIMERR_R = crate::BitReader<bool>;
#[doc = "Field `CURLIMERR` writer - CURLIMERR field"]
pub type CURLIMERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `CMD12ERR` reader - CMD12ERR field"]
pub type CMD12ERR_R = crate::BitReader<bool>;
#[doc = "Field `CMD12ERR` writer - CMD12ERR field"]
pub type CMD12ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `DMAERR` reader - DMAERR field"]
pub type DMAERR_R = crate::BitReader<bool>;
#[doc = "Field `DMAERR` writer - DMAERR field"]
pub type DMAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `RESPERR` reader - RESPERR field"]
pub type RESPERR_R = crate::BitReader<bool>;
#[doc = "Field `RESPERR` writer - RESPERR field"]
pub type RESPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CMDDONE field"]
    #[inline(always)]
    pub fn cmddone(&self) -> CMDDONE_R {
        CMDDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TRXDONE field"]
    #[inline(always)]
    pub fn trxdone(&self) -> TRXDONE_R {
        TRXDONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BLKGAP field"]
    #[inline(always)]
    pub fn blkgap(&self) -> BLKGAP_R {
        BLKGAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMADONE field"]
    #[inline(always)]
    pub fn dmadone(&self) -> DMADONE_R {
        DMADONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BUFWRRDY field"]
    #[inline(always)]
    pub fn bufwrrdy(&self) -> BUFWRRDY_R {
        BUFWRRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BUFRDRDY field"]
    #[inline(always)]
    pub fn bufrdrdy(&self) -> BUFRDRDY_R {
        BUFRDRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CARDINSR field"]
    #[inline(always)]
    pub fn cardinsr(&self) -> CARDINSR_R {
        CARDINSR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CARDRMOV field"]
    #[inline(always)]
    pub fn cardrmov(&self) -> CARDRMOV_R {
        CARDRMOV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CARD field"]
    #[inline(always)]
    pub fn card(&self) -> CARD_R {
        CARD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - CMDTIMEOUT field"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CMDTIMEOUT_R {
        CMDTIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CMDCRCERR field"]
    #[inline(always)]
    pub fn cmdcrcerr(&self) -> CMDCRCERR_R {
        CMDCRCERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CMDENDERR field"]
    #[inline(always)]
    pub fn cmdenderr(&self) -> CMDENDERR_R {
        CMDENDERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CMDIDXERR field"]
    #[inline(always)]
    pub fn cmdidxerr(&self) -> CMDIDXERR_R {
        CMDIDXERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DATTIMEOUT field"]
    #[inline(always)]
    pub fn dattimeout(&self) -> DATTIMEOUT_R {
        DATTIMEOUT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DATCRCERR field"]
    #[inline(always)]
    pub fn datcrcerr(&self) -> DATCRCERR_R {
        DATCRCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DATENDERR field"]
    #[inline(always)]
    pub fn datenderr(&self) -> DATENDERR_R {
        DATENDERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CURLIMERR field"]
    #[inline(always)]
    pub fn curlimerr(&self) -> CURLIMERR_R {
        CURLIMERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CMD12ERR field"]
    #[inline(always)]
    pub fn cmd12err(&self) -> CMD12ERR_R {
        CMD12ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMAERR field"]
    #[inline(always)]
    pub fn dmaerr(&self) -> DMAERR_R {
        DMAERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - RESPERR field"]
    #[inline(always)]
    pub fn resperr(&self) -> RESPERR_R {
        RESPERR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMDDONE field"]
    #[inline(always)]
    pub fn cmddone(&mut self) -> CMDDONE_W<0> {
        CMDDONE_W::new(self)
    }
    #[doc = "Bit 1 - TRXDONE field"]
    #[inline(always)]
    pub fn trxdone(&mut self) -> TRXDONE_W<1> {
        TRXDONE_W::new(self)
    }
    #[doc = "Bit 2 - BLKGAP field"]
    #[inline(always)]
    pub fn blkgap(&mut self) -> BLKGAP_W<2> {
        BLKGAP_W::new(self)
    }
    #[doc = "Bit 3 - DMADONE field"]
    #[inline(always)]
    pub fn dmadone(&mut self) -> DMADONE_W<3> {
        DMADONE_W::new(self)
    }
    #[doc = "Bit 4 - BUFWRRDY field"]
    #[inline(always)]
    pub fn bufwrrdy(&mut self) -> BUFWRRDY_W<4> {
        BUFWRRDY_W::new(self)
    }
    #[doc = "Bit 5 - BUFRDRDY field"]
    #[inline(always)]
    pub fn bufrdrdy(&mut self) -> BUFRDRDY_W<5> {
        BUFRDRDY_W::new(self)
    }
    #[doc = "Bit 6 - CARDINSR field"]
    #[inline(always)]
    pub fn cardinsr(&mut self) -> CARDINSR_W<6> {
        CARDINSR_W::new(self)
    }
    #[doc = "Bit 7 - CARDRMOV field"]
    #[inline(always)]
    pub fn cardrmov(&mut self) -> CARDRMOV_W<7> {
        CARDRMOV_W::new(self)
    }
    #[doc = "Bit 8 - CARD field"]
    #[inline(always)]
    pub fn card(&mut self) -> CARD_W<8> {
        CARD_W::new(self)
    }
    #[doc = "Bit 16 - CMDTIMEOUT field"]
    #[inline(always)]
    pub fn cmdtimeout(&mut self) -> CMDTIMEOUT_W<16> {
        CMDTIMEOUT_W::new(self)
    }
    #[doc = "Bit 17 - CMDCRCERR field"]
    #[inline(always)]
    pub fn cmdcrcerr(&mut self) -> CMDCRCERR_W<17> {
        CMDCRCERR_W::new(self)
    }
    #[doc = "Bit 18 - CMDENDERR field"]
    #[inline(always)]
    pub fn cmdenderr(&mut self) -> CMDENDERR_W<18> {
        CMDENDERR_W::new(self)
    }
    #[doc = "Bit 19 - CMDIDXERR field"]
    #[inline(always)]
    pub fn cmdidxerr(&mut self) -> CMDIDXERR_W<19> {
        CMDIDXERR_W::new(self)
    }
    #[doc = "Bit 20 - DATTIMEOUT field"]
    #[inline(always)]
    pub fn dattimeout(&mut self) -> DATTIMEOUT_W<20> {
        DATTIMEOUT_W::new(self)
    }
    #[doc = "Bit 21 - DATCRCERR field"]
    #[inline(always)]
    pub fn datcrcerr(&mut self) -> DATCRCERR_W<21> {
        DATCRCERR_W::new(self)
    }
    #[doc = "Bit 22 - DATENDERR field"]
    #[inline(always)]
    pub fn datenderr(&mut self) -> DATENDERR_W<22> {
        DATENDERR_W::new(self)
    }
    #[doc = "Bit 23 - CURLIMERR field"]
    #[inline(always)]
    pub fn curlimerr(&mut self) -> CURLIMERR_W<23> {
        CURLIMERR_W::new(self)
    }
    #[doc = "Bit 24 - CMD12ERR field"]
    #[inline(always)]
    pub fn cmd12err(&mut self) -> CMD12ERR_W<24> {
        CMD12ERR_W::new(self)
    }
    #[doc = "Bit 25 - DMAERR field"]
    #[inline(always)]
    pub fn dmaerr(&mut self) -> DMAERR_W<25> {
        DMAERR_W::new(self)
    }
    #[doc = "Bit 28 - RESPERR field"]
    #[inline(always)]
    pub fn resperr(&mut self) -> RESPERR_W<28> {
        RESPERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
