#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - DMAEN field"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMAEN field"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `BLKCNTEN` reader - BLKCNTEN field"]
pub type BLKCNTEN_R = crate::BitReader<bool>;
#[doc = "Field `BLKCNTEN` writer - BLKCNTEN field"]
pub type BLKCNTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `AUTOCMD12` reader - AUTOCMD12 field"]
pub type AUTOCMD12_R = crate::BitReader<bool>;
#[doc = "Field `AUTOCMD12` writer - AUTOCMD12 field"]
pub type AUTOCMD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `DIRREAD` reader - DIRREAD field"]
pub type DIRREAD_R = crate::BitReader<bool>;
#[doc = "Field `DIRREAD` writer - DIRREAD field"]
pub type DIRREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `MULTBLK` reader - MULTBLK field"]
pub type MULTBLK_R = crate::BitReader<bool>;
#[doc = "Field `MULTBLK` writer - MULTBLK field"]
pub type MULTBLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RESPTYPE` reader - RESPTYPE field"]
pub type RESPTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESPTYPE` writer - RESPTYPE field"]
pub type RESPTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `CRCCHECK` reader - CRCCHECK field"]
pub type CRCCHECK_R = crate::BitReader<bool>;
#[doc = "Field `CRCCHECK` writer - CRCCHECK field"]
pub type CRCCHECK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `IDXCHECK` reader - IDXCHECK field"]
pub type IDXCHECK_R = crate::BitReader<bool>;
#[doc = "Field `IDXCHECK` writer - IDXCHECK field"]
pub type IDXCHECK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `HAVEDATA` reader - HAVEDATA field"]
pub type HAVEDATA_R = crate::BitReader<bool>;
#[doc = "Field `HAVEDATA` writer - HAVEDATA field"]
pub type HAVEDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CMDTYPE` reader - CMDTYPE field"]
pub type CMDTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDTYPE` writer - CMDTYPE field"]
pub type CMDTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `CMDINDX` reader - CMDINDX field"]
pub type CMDINDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINDX` writer - CMDINDX field"]
pub type CMDINDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - DMAEN field"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BLKCNTEN field"]
    #[inline(always)]
    pub fn blkcnten(&self) -> BLKCNTEN_R {
        BLKCNTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AUTOCMD12 field"]
    #[inline(always)]
    pub fn autocmd12(&self) -> AUTOCMD12_R {
        AUTOCMD12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DIRREAD field"]
    #[inline(always)]
    pub fn dirread(&self) -> DIRREAD_R {
        DIRREAD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MULTBLK field"]
    #[inline(always)]
    pub fn multblk(&self) -> MULTBLK_R {
        MULTBLK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RESPTYPE field"]
    #[inline(always)]
    pub fn resptype(&self) -> RESPTYPE_R {
        RESPTYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - CRCCHECK field"]
    #[inline(always)]
    pub fn crccheck(&self) -> CRCCHECK_R {
        CRCCHECK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IDXCHECK field"]
    #[inline(always)]
    pub fn idxcheck(&self) -> IDXCHECK_R {
        IDXCHECK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HAVEDATA field"]
    #[inline(always)]
    pub fn havedata(&self) -> HAVEDATA_R {
        HAVEDATA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - CMDTYPE field"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CMDTYPE_R {
        CMDTYPE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - CMDINDX field"]
    #[inline(always)]
    pub fn cmdindx(&self) -> CMDINDX_R {
        CMDINDX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN field"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - BLKCNTEN field"]
    #[inline(always)]
    pub fn blkcnten(&mut self) -> BLKCNTEN_W<1> {
        BLKCNTEN_W::new(self)
    }
    #[doc = "Bit 2 - AUTOCMD12 field"]
    #[inline(always)]
    pub fn autocmd12(&mut self) -> AUTOCMD12_W<2> {
        AUTOCMD12_W::new(self)
    }
    #[doc = "Bit 4 - DIRREAD field"]
    #[inline(always)]
    pub fn dirread(&mut self) -> DIRREAD_W<4> {
        DIRREAD_W::new(self)
    }
    #[doc = "Bit 5 - MULTBLK field"]
    #[inline(always)]
    pub fn multblk(&mut self) -> MULTBLK_W<5> {
        MULTBLK_W::new(self)
    }
    #[doc = "Bits 16:17 - RESPTYPE field"]
    #[inline(always)]
    pub fn resptype(&mut self) -> RESPTYPE_W<16> {
        RESPTYPE_W::new(self)
    }
    #[doc = "Bit 19 - CRCCHECK field"]
    #[inline(always)]
    pub fn crccheck(&mut self) -> CRCCHECK_W<19> {
        CRCCHECK_W::new(self)
    }
    #[doc = "Bit 20 - IDXCHECK field"]
    #[inline(always)]
    pub fn idxcheck(&mut self) -> IDXCHECK_W<20> {
        IDXCHECK_W::new(self)
    }
    #[doc = "Bit 21 - HAVEDATA field"]
    #[inline(always)]
    pub fn havedata(&mut self) -> HAVEDATA_W<21> {
        HAVEDATA_W::new(self)
    }
    #[doc = "Bits 22:23 - CMDTYPE field"]
    #[inline(always)]
    pub fn cmdtype(&mut self) -> CMDTYPE_W<22> {
        CMDTYPE_W::new(self)
    }
    #[doc = "Bits 24:29 - CMDINDX field"]
    #[inline(always)]
    pub fn cmdindx(&mut self) -> CMDINDX_W<24> {
        CMDINDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMD register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
