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
#[doc = "Field `RXDA` reader - RXDA field"]
pub type RXDA_R = crate::BitReader<bool>;
#[doc = "Field `RXOV` reader - RXOV field"]
pub type RXOV_R = crate::BitReader<bool>;
#[doc = "Field `TXBR` reader - TXBR field"]
pub type TXBR_R = crate::BitReader<bool>;
#[doc = "Field `TXOK` reader - TXOK field"]
pub type TXOK_R = crate::BitReader<bool>;
#[doc = "Field `RXBUSY` reader - RXBUSY field"]
pub type RXBUSY_R = crate::BitReader<bool>;
#[doc = "Field `TXBUSY` reader - TXBUSY field"]
pub type TXBUSY_R = crate::BitReader<bool>;
#[doc = "Field `ERRWARN` reader - ERRWARN field"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `BUSOFF` reader - BUSOFF field"]
pub type BUSOFF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RXDA field"]
    #[inline(always)]
    pub fn rxda(&self) -> RXDA_R {
        RXDA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXOV field"]
    #[inline(always)]
    pub fn rxov(&self) -> RXOV_R {
        RXOV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXBR field"]
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXOK field"]
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXBUSY field"]
    #[inline(always)]
    pub fn rxbusy(&self) -> RXBUSY_R {
        RXBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXBUSY field"]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ERRWARN field"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BUSOFF field"]
    #[inline(always)]
    pub fn busoff(&self) -> BUSOFF_R {
        BUSOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "SR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
