#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - LEN field"]
pub type LEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LEN` writer - LEN field"]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u32, u32, 20, O>;
#[doc = "Field `RXEN` reader - RXEN field"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - RXEN field"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - TXEN field"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - TXEN field"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `AUTORE` reader - AUTORE field"]
pub type AUTORE_R = crate::BitReader<bool>;
#[doc = "Field `AUTORE` writer - AUTORE field"]
pub type AUTORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STEPOP` reader - STEPOP field"]
pub type STEPOP_R = crate::BitReader<bool>;
#[doc = "Field `STEPOP` writer - STEPOP field"]
pub type STEPOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:19 - LEN field"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 24 - RXEN field"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TXEN field"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AUTORE field"]
    #[inline(always)]
    pub fn autore(&self) -> AUTORE_R {
        AUTORE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - STEPOP field"]
    #[inline(always)]
    pub fn stepop(&self) -> STEPOP_R {
        STEPOP_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - LEN field"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    #[doc = "Bit 24 - RXEN field"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W<24> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 25 - TXEN field"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W<25> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 26 - AUTORE field"]
    #[inline(always)]
    pub fn autore(&mut self) -> AUTORE_W<26> {
        AUTORE_W::new(self)
    }
    #[doc = "Bit 27 - STEPOP field"]
    #[inline(always)]
    pub fn stepop(&mut self) -> STEPOP_W<27> {
        STEPOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
