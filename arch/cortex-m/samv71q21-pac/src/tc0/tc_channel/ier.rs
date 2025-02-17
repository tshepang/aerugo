#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `COVFS` writer - Counter Overflow"]
pub type COVFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOVRS` writer - Load Overrun"]
pub type LOVRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPAS` writer - RA Compare"]
pub type CPAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPBS` writer - RB Compare"]
pub type CPBS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPCS` writer - RC Compare"]
pub type CPCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LDRAS` writer - RA Loading"]
pub type LDRAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LDRBS` writer - RB Loading"]
pub type LDRBS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETRGS` writer - External Trigger"]
pub type ETRGS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn covfs(&mut self) -> COVFS_W<IER_SPEC, 0> {
        COVFS_W::new(self)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn lovrs(&mut self) -> LOVRS_W<IER_SPEC, 1> {
        LOVRS_W::new(self)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpas(&mut self) -> CPAS_W<IER_SPEC, 2> {
        CPAS_W::new(self)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpbs(&mut self) -> CPBS_W<IER_SPEC, 3> {
        CPBS_W::new(self)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpcs(&mut self) -> CPCS_W<IER_SPEC, 4> {
        CPCS_W::new(self)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldras(&mut self) -> LDRAS_W<IER_SPEC, 5> {
        LDRAS_W::new(self)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldrbs(&mut self) -> LDRBS_W<IER_SPEC, 6> {
        LDRBS_W::new(self)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn etrgs(&mut self) -> ETRGS_W<IER_SPEC, 7> {
        ETRGS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register (channel = 0)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
