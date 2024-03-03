#[doc = "Register `TR` reader"]
pub struct R(crate::R<TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR` writer"]
pub struct W(crate::W<TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_SPEC>;
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
impl From<crate::W<TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXACK` reader - TXACK field"]
pub type TXACK_R = crate::BitReader<bool>;
#[doc = "Field `TXACK` writer - TXACK field"]
pub type TXACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `RXACK` reader - RXACK field"]
pub type RXACK_R = crate::BitReader<bool>;
#[doc = "Field `RXACK` writer - RXACK field"]
pub type RXACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `TXCLR` reader - TXCLR field"]
pub type TXCLR_R = crate::BitReader<bool>;
#[doc = "Field `TXCLR` writer - TXCLR field"]
pub type TXCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `SLVACT` reader - SLVACT field"]
pub type SLVACT_R = crate::BitReader<bool>;
#[doc = "Field `SLVACT` writer - SLVACT field"]
pub type SLVACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `SLVRD` reader - SLVRD field"]
pub type SLVRD_R = crate::BitReader<bool>;
#[doc = "Field `SLVRD` writer - SLVRD field"]
pub type SLVRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `SLVWR` reader - SLVWR field"]
pub type SLVWR_R = crate::BitReader<bool>;
#[doc = "Field `SLVWR` writer - SLVWR field"]
pub type SLVWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `SLVSTR` reader - SLVSTR field"]
pub type SLVSTR_R = crate::BitReader<bool>;
#[doc = "Field `SLVSTR` writer - SLVSTR field"]
pub type SLVSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `SLVRDS` reader - SLVRDS field"]
pub type SLVRDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLVRDS` writer - SLVRDS field"]
pub type SLVRDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - TXACK field"]
    #[inline(always)]
    pub fn txack(&self) -> TXACK_R {
        TXACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXACK field"]
    #[inline(always)]
    pub fn rxack(&self) -> RXACK_R {
        RXACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCLR field"]
    #[inline(always)]
    pub fn txclr(&self) -> TXCLR_R {
        TXCLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SLVACT field"]
    #[inline(always)]
    pub fn slvact(&self) -> SLVACT_R {
        SLVACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SLVRD field"]
    #[inline(always)]
    pub fn slvrd(&self) -> SLVRD_R {
        SLVRD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SLVWR field"]
    #[inline(always)]
    pub fn slvwr(&self) -> SLVWR_R {
        SLVWR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SLVSTR field"]
    #[inline(always)]
    pub fn slvstr(&self) -> SLVSTR_R {
        SLVSTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - SLVRDS field"]
    #[inline(always)]
    pub fn slvrds(&self) -> SLVRDS_R {
        SLVRDS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXACK field"]
    #[inline(always)]
    pub fn txack(&mut self) -> TXACK_W<0> {
        TXACK_W::new(self)
    }
    #[doc = "Bit 1 - RXACK field"]
    #[inline(always)]
    pub fn rxack(&mut self) -> RXACK_W<1> {
        RXACK_W::new(self)
    }
    #[doc = "Bit 2 - TXCLR field"]
    #[inline(always)]
    pub fn txclr(&mut self) -> TXCLR_W<2> {
        TXCLR_W::new(self)
    }
    #[doc = "Bit 8 - SLVACT field"]
    #[inline(always)]
    pub fn slvact(&mut self) -> SLVACT_W<8> {
        SLVACT_W::new(self)
    }
    #[doc = "Bit 9 - SLVRD field"]
    #[inline(always)]
    pub fn slvrd(&mut self) -> SLVRD_W<9> {
        SLVRD_W::new(self)
    }
    #[doc = "Bit 10 - SLVWR field"]
    #[inline(always)]
    pub fn slvwr(&mut self) -> SLVWR_W<10> {
        SLVWR_W::new(self)
    }
    #[doc = "Bit 11 - SLVSTR field"]
    #[inline(always)]
    pub fn slvstr(&mut self) -> SLVSTR_W<11> {
        SLVSTR_W::new(self)
    }
    #[doc = "Bits 12:13 - SLVRDS field"]
    #[inline(always)]
    pub fn slvrds(&mut self) -> SLVRDS_W<12> {
        SLVRDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](index.html) module"]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr::R](R) reader structure"]
impl crate::Readable for TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr::W](W) writer structure"]
impl crate::Writable for TR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
