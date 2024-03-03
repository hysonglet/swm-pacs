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
#[doc = "Field `SEC` reader - SEC field"]
pub type SEC_R = crate::BitReader<bool>;
#[doc = "Field `SEC` writer - SEC field"]
pub type SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MIN` reader - MIN field"]
pub type MIN_R = crate::BitReader<bool>;
#[doc = "Field `MIN` writer - MIN field"]
pub type MIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `HOUR` reader - HOUR field"]
pub type HOUR_R = crate::BitReader<bool>;
#[doc = "Field `HOUR` writer - HOUR field"]
pub type HOUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `DATE` reader - DATE field"]
pub type DATE_R = crate::BitReader<bool>;
#[doc = "Field `DATE` writer - DATE field"]
pub type DATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ALARM` reader - ALARM field"]
pub type ALARM_R = crate::BitReader<bool>;
#[doc = "Field `ALARM` writer - ALARM field"]
pub type ALARM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TRIM` reader - TRIM field"]
pub type TRIM_R = crate::BitReader<bool>;
#[doc = "Field `TRIM` writer - TRIM field"]
pub type TRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `HSEC` reader - HSEC field"]
pub type HSEC_R = crate::BitReader<bool>;
#[doc = "Field `HSEC` writer - HSEC field"]
pub type HSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `QSEC` reader - QSEC field"]
pub type QSEC_R = crate::BitReader<bool>;
#[doc = "Field `QSEC` writer - QSEC field"]
pub type QSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SEC field"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MIN field"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HOUR field"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DATE field"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ALARM field"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRIM field"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSEC field"]
    #[inline(always)]
    pub fn hsec(&self) -> HSEC_R {
        HSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - QSEC field"]
    #[inline(always)]
    pub fn qsec(&self) -> QSEC_R {
        QSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SEC field"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
    }
    #[doc = "Bit 1 - MIN field"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W<1> {
        MIN_W::new(self)
    }
    #[doc = "Bit 2 - HOUR field"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W<2> {
        HOUR_W::new(self)
    }
    #[doc = "Bit 3 - DATE field"]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W<3> {
        DATE_W::new(self)
    }
    #[doc = "Bit 4 - ALARM field"]
    #[inline(always)]
    pub fn alarm(&mut self) -> ALARM_W<4> {
        ALARM_W::new(self)
    }
    #[doc = "Bit 5 - TRIM field"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<5> {
        TRIM_W::new(self)
    }
    #[doc = "Bit 6 - HSEC field"]
    #[inline(always)]
    pub fn hsec(&mut self) -> HSEC_W<6> {
        HSEC_W::new(self)
    }
    #[doc = "Bit 7 - QSEC field"]
    #[inline(always)]
    pub fn qsec(&mut self) -> QSEC_W<7> {
        QSEC_W::new(self)
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
