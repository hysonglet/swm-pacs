#[doc = "Register `CMD12ERR` reader"]
pub struct R(crate::R<CMD12ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD12ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD12ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD12ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD12ERR` writer"]
pub struct W(crate::W<CMD12ERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD12ERR_SPEC>;
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
impl From<crate::W<CMD12ERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD12ERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NE` reader - NE field"]
pub type NE_R = crate::BitReader<bool>;
#[doc = "Field `NE` writer - NE field"]
pub type NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD12ERR_SPEC, bool, O>;
#[doc = "Field `TO` reader - TO field"]
pub type TO_R = crate::BitReader<bool>;
#[doc = "Field `TO` writer - TO field"]
pub type TO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD12ERR_SPEC, bool, O>;
#[doc = "Field `CRC` reader - CRC field"]
pub type CRC_R = crate::BitReader<bool>;
#[doc = "Field `CRC` writer - CRC field"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD12ERR_SPEC, bool, O>;
#[doc = "Field `END` reader - END field"]
pub type END_R = crate::BitReader<bool>;
#[doc = "Field `END` writer - END field"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD12ERR_SPEC, bool, O>;
#[doc = "Field `INDEX` reader - INDEX field"]
pub type INDEX_R = crate::BitReader<bool>;
#[doc = "Field `INDEX` writer - INDEX field"]
pub type INDEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD12ERR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NE field"]
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TO field"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC field"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - END field"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INDEX field"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NE field"]
    #[inline(always)]
    pub fn ne(&mut self) -> NE_W<0> {
        NE_W::new(self)
    }
    #[doc = "Bit 1 - TO field"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W<1> {
        TO_W::new(self)
    }
    #[doc = "Bit 2 - CRC field"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<2> {
        CRC_W::new(self)
    }
    #[doc = "Bit 3 - END field"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W<3> {
        END_W::new(self)
    }
    #[doc = "Bit 4 - INDEX field"]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W<4> {
        INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMD12ERR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd12err](index.html) module"]
pub struct CMD12ERR_SPEC;
impl crate::RegisterSpec for CMD12ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd12err::R](R) reader structure"]
impl crate::Readable for CMD12ERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd12err::W](W) writer structure"]
impl crate::Writable for CMD12ERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD12ERR to value 0"]
impl crate::Resettable for CMD12ERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
