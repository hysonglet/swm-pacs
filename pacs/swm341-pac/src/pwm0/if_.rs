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
#[doc = "Field `UPOVF` reader - UPOVF field"]
pub type UPOVF_R = crate::BitReader<bool>;
#[doc = "Field `UPOVF` writer - UPOVF field"]
pub type UPOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `DNOVF` reader - DNOVF field"]
pub type DNOVF_R = crate::BitReader<bool>;
#[doc = "Field `DNOVF` writer - DNOVF field"]
pub type DNOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `UPCMPA` reader - UPCMPA field"]
pub type UPCMPA_R = crate::BitReader<bool>;
#[doc = "Field `UPCMPA` writer - UPCMPA field"]
pub type UPCMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `UPCMPB` reader - UPCMPB field"]
pub type UPCMPB_R = crate::BitReader<bool>;
#[doc = "Field `UPCMPB` writer - UPCMPB field"]
pub type UPCMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `DNCMPA` reader - DNCMPA field"]
pub type DNCMPA_R = crate::BitReader<bool>;
#[doc = "Field `DNCMPA` writer - DNCMPA field"]
pub type DNCMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `DNCMPB` reader - DNCMPB field"]
pub type DNCMPB_R = crate::BitReader<bool>;
#[doc = "Field `DNCMPB` writer - DNCMPB field"]
pub type DNCMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UPOVF field"]
    #[inline(always)]
    pub fn upovf(&self) -> UPOVF_R {
        UPOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DNOVF field"]
    #[inline(always)]
    pub fn dnovf(&self) -> DNOVF_R {
        DNOVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UPCMPA field"]
    #[inline(always)]
    pub fn upcmpa(&self) -> UPCMPA_R {
        UPCMPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UPCMPB field"]
    #[inline(always)]
    pub fn upcmpb(&self) -> UPCMPB_R {
        UPCMPB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DNCMPA field"]
    #[inline(always)]
    pub fn dncmpa(&self) -> DNCMPA_R {
        DNCMPA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DNCMPB field"]
    #[inline(always)]
    pub fn dncmpb(&self) -> DNCMPB_R {
        DNCMPB_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UPOVF field"]
    #[inline(always)]
    pub fn upovf(&mut self) -> UPOVF_W<0> {
        UPOVF_W::new(self)
    }
    #[doc = "Bit 1 - DNOVF field"]
    #[inline(always)]
    pub fn dnovf(&mut self) -> DNOVF_W<1> {
        DNOVF_W::new(self)
    }
    #[doc = "Bit 2 - UPCMPA field"]
    #[inline(always)]
    pub fn upcmpa(&mut self) -> UPCMPA_W<2> {
        UPCMPA_W::new(self)
    }
    #[doc = "Bit 3 - UPCMPB field"]
    #[inline(always)]
    pub fn upcmpb(&mut self) -> UPCMPB_W<3> {
        UPCMPB_W::new(self)
    }
    #[doc = "Bit 4 - DNCMPA field"]
    #[inline(always)]
    pub fn dncmpa(&mut self) -> DNCMPA_W<4> {
        DNCMPA_W::new(self)
    }
    #[doc = "Bit 5 - DNCMPB field"]
    #[inline(always)]
    pub fn dncmpb(&mut self) -> DNCMPB_W<5> {
        DNCMPB_W::new(self)
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
