#[doc = "Register `OTGCSR` reader"]
pub struct R(crate::R<OTGCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGCSR` writer"]
pub struct W(crate::W<OTGCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGCSR_SPEC>;
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
impl From<crate::W<OTGCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRSUCC` reader - SRSUCC field"]
pub type SRSUCC_R = crate::BitReader<bool>;
#[doc = "Field `SRSUCC` writer - SRSUCC field"]
pub type SRSUCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `SRSCHG` reader - SRSCHG field"]
pub type SRSCHG_R = crate::BitReader<bool>;
#[doc = "Field `SRSCHG` writer - SRSCHG field"]
pub type SRSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `HNSUCC` reader - HNSUCC field"]
pub type HNSUCC_R = crate::BitReader<bool>;
#[doc = "Field `HNSUCC` writer - HNSUCC field"]
pub type HNSUCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `HNSCHG` reader - HNSCHG field"]
pub type HNSCHG_R = crate::BitReader<bool>;
#[doc = "Field `HNSCHG` writer - HNSCHG field"]
pub type HNSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `SRDET` reader - SRDET field"]
pub type SRDET_R = crate::BitReader<bool>;
#[doc = "Field `SRDET` writer - SRDET field"]
pub type SRDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `SRDCHG` reader - SRDCHG field"]
pub type SRDCHG_R = crate::BitReader<bool>;
#[doc = "Field `SRDCHG` writer - SRDCHG field"]
pub type SRDCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `HNDET` reader - HNDET field"]
pub type HNDET_R = crate::BitReader<bool>;
#[doc = "Field `HNDET` writer - HNDET field"]
pub type HNDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `HNDCHG` reader - HNDCHG field"]
pub type HNDCHG_R = crate::BitReader<bool>;
#[doc = "Field `HNDCHG` writer - HNDCHG field"]
pub type HNDCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `ID` reader - ID field"]
pub type ID_R = crate::BitReader<bool>;
#[doc = "Field `ID` writer - ID field"]
pub type ID_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `IDCHG` reader - IDCHG field"]
pub type IDCHG_R = crate::BitReader<bool>;
#[doc = "Field `IDCHG` writer - IDCHG field"]
pub type IDCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - MODE field"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - MODE field"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `SRPREQ` reader - SRPREQ field"]
pub type SRPREQ_R = crate::BitReader<bool>;
#[doc = "Field `SRPREQ` writer - SRPREQ field"]
pub type SRPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `HNPREQ` reader - HNPREQ field"]
pub type HNPREQ_R = crate::BitReader<bool>;
#[doc = "Field `HNPREQ` writer - HNPREQ field"]
pub type HNPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `HHNPENA` reader - HHNPENA field"]
pub type HHNPENA_R = crate::BitReader<bool>;
#[doc = "Field `HHNPENA` writer - HHNPENA field"]
pub type HHNPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `HNPENA` reader - HNPENA field"]
pub type HNPENA_R = crate::BitReader<bool>;
#[doc = "Field `HNPENA` writer - HNPENA field"]
pub type HNPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `SRPCAP` reader - SRPCAP field"]
pub type SRPCAP_R = crate::BitReader<bool>;
#[doc = "Field `SRPCAP` writer - SRPCAP field"]
pub type SRPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
#[doc = "Field `HNPCAP` reader - HNPCAP field"]
pub type HNPCAP_R = crate::BitReader<bool>;
#[doc = "Field `HNPCAP` writer - HNPCAP field"]
pub type HNPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SRSUCC field"]
    #[inline(always)]
    pub fn srsucc(&self) -> SRSUCC_R {
        SRSUCC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRSCHG field"]
    #[inline(always)]
    pub fn srschg(&self) -> SRSCHG_R {
        SRSCHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HNSUCC field"]
    #[inline(always)]
    pub fn hnsucc(&self) -> HNSUCC_R {
        HNSUCC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HNSCHG field"]
    #[inline(always)]
    pub fn hnschg(&self) -> HNSCHG_R {
        HNSCHG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRDET field"]
    #[inline(always)]
    pub fn srdet(&self) -> SRDET_R {
        SRDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRDCHG field"]
    #[inline(always)]
    pub fn srdchg(&self) -> SRDCHG_R {
        SRDCHG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HNDET field"]
    #[inline(always)]
    pub fn hndet(&self) -> HNDET_R {
        HNDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HNDCHG field"]
    #[inline(always)]
    pub fn hndchg(&self) -> HNDCHG_R {
        HNDCHG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ID field"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDCHG field"]
    #[inline(always)]
    pub fn idchg(&self) -> IDCHG_R {
        IDCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MODE field"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - SRPREQ field"]
    #[inline(always)]
    pub fn srpreq(&self) -> SRPREQ_R {
        SRPREQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HNPREQ field"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HHNPENA field"]
    #[inline(always)]
    pub fn hhnpena(&self) -> HHNPENA_R {
        HHNPENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HNPENA field"]
    #[inline(always)]
    pub fn hnpena(&self) -> HNPENA_R {
        HNPENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SRPCAP field"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HNPCAP field"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRSUCC field"]
    #[inline(always)]
    pub fn srsucc(&mut self) -> SRSUCC_W<0> {
        SRSUCC_W::new(self)
    }
    #[doc = "Bit 1 - SRSCHG field"]
    #[inline(always)]
    pub fn srschg(&mut self) -> SRSCHG_W<1> {
        SRSCHG_W::new(self)
    }
    #[doc = "Bit 2 - HNSUCC field"]
    #[inline(always)]
    pub fn hnsucc(&mut self) -> HNSUCC_W<2> {
        HNSUCC_W::new(self)
    }
    #[doc = "Bit 3 - HNSCHG field"]
    #[inline(always)]
    pub fn hnschg(&mut self) -> HNSCHG_W<3> {
        HNSCHG_W::new(self)
    }
    #[doc = "Bit 4 - SRDET field"]
    #[inline(always)]
    pub fn srdet(&mut self) -> SRDET_W<4> {
        SRDET_W::new(self)
    }
    #[doc = "Bit 5 - SRDCHG field"]
    #[inline(always)]
    pub fn srdchg(&mut self) -> SRDCHG_W<5> {
        SRDCHG_W::new(self)
    }
    #[doc = "Bit 6 - HNDET field"]
    #[inline(always)]
    pub fn hndet(&mut self) -> HNDET_W<6> {
        HNDET_W::new(self)
    }
    #[doc = "Bit 7 - HNDCHG field"]
    #[inline(always)]
    pub fn hndchg(&mut self) -> HNDCHG_W<7> {
        HNDCHG_W::new(self)
    }
    #[doc = "Bit 8 - ID field"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W<8> {
        ID_W::new(self)
    }
    #[doc = "Bit 9 - IDCHG field"]
    #[inline(always)]
    pub fn idchg(&mut self) -> IDCHG_W<9> {
        IDCHG_W::new(self)
    }
    #[doc = "Bit 10 - MODE field"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<10> {
        MODE_W::new(self)
    }
    #[doc = "Bit 16 - SRPREQ field"]
    #[inline(always)]
    pub fn srpreq(&mut self) -> SRPREQ_W<16> {
        SRPREQ_W::new(self)
    }
    #[doc = "Bit 17 - HNPREQ field"]
    #[inline(always)]
    pub fn hnpreq(&mut self) -> HNPREQ_W<17> {
        HNPREQ_W::new(self)
    }
    #[doc = "Bit 18 - HHNPENA field"]
    #[inline(always)]
    pub fn hhnpena(&mut self) -> HHNPENA_W<18> {
        HHNPENA_W::new(self)
    }
    #[doc = "Bit 19 - HNPENA field"]
    #[inline(always)]
    pub fn hnpena(&mut self) -> HNPENA_W<19> {
        HNPENA_W::new(self)
    }
    #[doc = "Bit 20 - SRPCAP field"]
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W<20> {
        SRPCAP_W::new(self)
    }
    #[doc = "Bit 21 - HNPCAP field"]
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W<21> {
        HNPCAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTGCSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgcsr](index.html) module"]
pub struct OTGCSR_SPEC;
impl crate::RegisterSpec for OTGCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otgcsr::R](R) reader structure"]
impl crate::Readable for OTGCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgcsr::W](W) writer structure"]
impl crate::Writable for OTGCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTGCSR to value 0"]
impl crate::Resettable for OTGCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
