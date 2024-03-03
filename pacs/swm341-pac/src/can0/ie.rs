#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDA` reader - RXDA field"]
pub type RXDA_R = crate::BitReader<bool>;
#[doc = "Field `RXDA` writer - RXDA field"]
pub type RXDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TXBR` reader - TXBR field"]
pub type TXBR_R = crate::BitReader<bool>;
#[doc = "Field `TXBR` writer - TXBR field"]
pub type TXBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ERRWARN` reader - ERRWARN field"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `ERRWARN` writer - ERRWARN field"]
pub type ERRWARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `RXOV` reader - RXOV field"]
pub type RXOV_R = crate::BitReader<bool>;
#[doc = "Field `RXOV` writer - RXOV field"]
pub type RXOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `WKUP` reader - WKUP field"]
pub type WKUP_R = crate::BitReader<bool>;
#[doc = "Field `WKUP` writer - WKUP field"]
pub type WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ERRPASS` reader - ERRPASS field"]
pub type ERRPASS_R = crate::BitReader<bool>;
#[doc = "Field `ERRPASS` writer - ERRPASS field"]
pub type ERRPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ARBLOST` reader - ARBLOST field"]
pub type ARBLOST_R = crate::BitReader<bool>;
#[doc = "Field `ARBLOST` writer - ARBLOST field"]
pub type ARBLOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `BUSERR` reader - BUSERR field"]
pub type BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSERR` writer - BUSERR field"]
pub type BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXDA field"]
    #[inline(always)]
    pub fn rxda(&self) -> RXDA_R {
        RXDA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXBR field"]
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRWARN field"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXOV field"]
    #[inline(always)]
    pub fn rxov(&self) -> RXOV_R {
        RXOV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WKUP field"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ERRPASS field"]
    #[inline(always)]
    pub fn errpass(&self) -> ERRPASS_R {
        ERRPASS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ARBLOST field"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BUSERR field"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDA field"]
    #[inline(always)]
    pub fn rxda(&mut self) -> RXDA_W<0> {
        RXDA_W::new(self)
    }
    #[doc = "Bit 1 - TXBR field"]
    #[inline(always)]
    pub fn txbr(&mut self) -> TXBR_W<1> {
        TXBR_W::new(self)
    }
    #[doc = "Bit 2 - ERRWARN field"]
    #[inline(always)]
    pub fn errwarn(&mut self) -> ERRWARN_W<2> {
        ERRWARN_W::new(self)
    }
    #[doc = "Bit 3 - RXOV field"]
    #[inline(always)]
    pub fn rxov(&mut self) -> RXOV_W<3> {
        RXOV_W::new(self)
    }
    #[doc = "Bit 4 - WKUP field"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W<4> {
        WKUP_W::new(self)
    }
    #[doc = "Bit 5 - ERRPASS field"]
    #[inline(always)]
    pub fn errpass(&mut self) -> ERRPASS_W<5> {
        ERRPASS_W::new(self)
    }
    #[doc = "Bit 6 - ARBLOST field"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ARBLOST_W<6> {
        ARBLOST_W::new(self)
    }
    #[doc = "Bit 7 - BUSERR field"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W<7> {
        BUSERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
