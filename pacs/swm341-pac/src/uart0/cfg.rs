#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSBF` reader - MSBF field"]
pub type MSBF_R = crate::BitReader<bool>;
#[doc = "Field `MSBF` writer - MSBF field"]
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `BRKTXLEN` reader - BRKTXLEN field"]
pub type BRKTXLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRKTXLEN` writer - BRKTXLEN field"]
pub type BRKTXLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `BRKRXLEN` reader - BRKRXLEN field"]
pub type BRKRXLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRKRXLEN` writer - BRKRXLEN field"]
pub type BRKRXLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RXINV` reader - RXINV field"]
pub type RXINV_R = crate::BitReader<bool>;
#[doc = "Field `RXINV` writer - RXINV field"]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `TXINV` reader - TXINV field"]
pub type TXINV_R = crate::BitReader<bool>;
#[doc = "Field `TXINV` writer - TXINV field"]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - MSBF field"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - BRKTXLEN field"]
    #[inline(always)]
    pub fn brktxlen(&self) -> BRKTXLEN_R {
        BRKTXLEN_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - BRKRXLEN field"]
    #[inline(always)]
    pub fn brkrxlen(&self) -> BRKRXLEN_R {
        BRKRXLEN_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - RXINV field"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXINV field"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MSBF field"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W<1> {
        MSBF_W::new(self)
    }
    #[doc = "Bits 2:5 - BRKTXLEN field"]
    #[inline(always)]
    pub fn brktxlen(&mut self) -> BRKTXLEN_W<2> {
        BRKTXLEN_W::new(self)
    }
    #[doc = "Bits 6:9 - BRKRXLEN field"]
    #[inline(always)]
    pub fn brkrxlen(&mut self) -> BRKRXLEN_W<6> {
        BRKRXLEN_W::new(self)
    }
    #[doc = "Bit 10 - RXINV field"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W<10> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 11 - TXINV field"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W<11> {
        TXINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
