#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - BUSY field"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY field"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CUOVR` reader - CUOVR field"]
pub type CUOVR_R = crate::BitReader<bool>;
#[doc = "Field `CUOVR` writer - CUOVR field"]
pub type CUOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `REIMERR` reader - REIMERR field"]
pub type REIMERR_R = crate::BitReader<bool>;
#[doc = "Field `REIMERR` writer - REIMERR field"]
pub type REIMERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `BUFBUSY` reader - BUFBUSY field"]
pub type BUFBUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUFBUSY` writer - BUFBUSY field"]
pub type BUFBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `DMARDBUSY` reader - DMARDBUSY field"]
pub type DMARDBUSY_R = crate::BitReader<bool>;
#[doc = "Field `DMARDBUSY` writer - DMARDBUSY field"]
pub type DMARDBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `DMAWRBUSY` reader - DMAWRBUSY field"]
pub type DMAWRBUSY_R = crate::BitReader<bool>;
#[doc = "Field `DMAWRBUSY` writer - DMAWRBUSY field"]
pub type DMAWRBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BUSY field"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CUOVR field"]
    #[inline(always)]
    pub fn cuovr(&self) -> CUOVR_R {
        CUOVR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REIMERR field"]
    #[inline(always)]
    pub fn reimerr(&self) -> REIMERR_R {
        REIMERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - BUFBUSY field"]
    #[inline(always)]
    pub fn bufbusy(&self) -> BUFBUSY_R {
        BUFBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMARDBUSY field"]
    #[inline(always)]
    pub fn dmardbusy(&self) -> DMARDBUSY_R {
        DMARDBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAWRBUSY field"]
    #[inline(always)]
    pub fn dmawrbusy(&self) -> DMAWRBUSY_R {
        DMAWRBUSY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUSY field"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1 - CUOVR field"]
    #[inline(always)]
    pub fn cuovr(&mut self) -> CUOVR_W<1> {
        CUOVR_W::new(self)
    }
    #[doc = "Bit 2 - REIMERR field"]
    #[inline(always)]
    pub fn reimerr(&mut self) -> REIMERR_W<2> {
        REIMERR_W::new(self)
    }
    #[doc = "Bit 4 - BUFBUSY field"]
    #[inline(always)]
    pub fn bufbusy(&mut self) -> BUFBUSY_W<4> {
        BUFBUSY_W::new(self)
    }
    #[doc = "Bit 5 - DMARDBUSY field"]
    #[inline(always)]
    pub fn dmardbusy(&mut self) -> DMARDBUSY_W<5> {
        DMARDBUSY_W::new(self)
    }
    #[doc = "Bit 6 - DMAWRBUSY field"]
    #[inline(always)]
    pub fn dmawrbusy(&mut self) -> DMAWRBUSY_W<6> {
        DMAWRBUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
