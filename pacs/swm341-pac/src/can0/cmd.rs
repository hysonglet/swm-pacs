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
#[doc = "Field `TXREQ` writer - TXREQ field"]
pub type TXREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `ABTTX` writer - ABTTX field"]
pub type ABTTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RRB` writer - RRB field"]
pub type RRB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CLROV` writer - CLROV field"]
pub type CLROV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `SRR` writer - SRR field"]
pub type SRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - TXREQ field"]
    #[inline(always)]
    pub fn txreq(&mut self) -> TXREQ_W<0> {
        TXREQ_W::new(self)
    }
    #[doc = "Bit 1 - ABTTX field"]
    #[inline(always)]
    pub fn abttx(&mut self) -> ABTTX_W<1> {
        ABTTX_W::new(self)
    }
    #[doc = "Bit 2 - RRB field"]
    #[inline(always)]
    pub fn rrb(&mut self) -> RRB_W<2> {
        RRB_W::new(self)
    }
    #[doc = "Bit 3 - CLROV field"]
    #[inline(always)]
    pub fn clrov(&mut self) -> CLROV_W<3> {
        CLROV_W::new(self)
    }
    #[doc = "Bit 4 - SRR field"]
    #[inline(always)]
    pub fn srr(&mut self) -> SRR_W<4> {
        SRR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMD register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
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
