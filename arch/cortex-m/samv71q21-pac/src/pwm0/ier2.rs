#[doc = "Register `IER2` writer"]
pub type W = crate::W<IER2_SPEC>;
#[doc = "Field `WRDY` writer - Write Ready for Synchronous Channels Update Interrupt Enable"]
pub type WRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNRE` writer - Synchronous Channels Update Underrun Error Interrupt Enable"]
pub type UNRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPM0` writer - Comparison 0 Match Interrupt Enable"]
pub type CMPM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPM1` writer - Comparison 1 Match Interrupt Enable"]
pub type CMPM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPM2` writer - Comparison 2 Match Interrupt Enable"]
pub type CMPM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPM3` writer - Comparison 3 Match Interrupt Enable"]
pub type CMPM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPM4` writer - Comparison 4 Match Interrupt Enable"]
pub type CMPM4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPM5` writer - Comparison 5 Match Interrupt Enable"]
pub type CMPM5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPM6` writer - Comparison 6 Match Interrupt Enable"]
pub type CMPM6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPM7` writer - Comparison 7 Match Interrupt Enable"]
pub type CMPM7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPU0` writer - Comparison 0 Update Interrupt Enable"]
pub type CMPU0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPU1` writer - Comparison 1 Update Interrupt Enable"]
pub type CMPU1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPU2` writer - Comparison 2 Update Interrupt Enable"]
pub type CMPU2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPU3` writer - Comparison 3 Update Interrupt Enable"]
pub type CMPU3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPU4` writer - Comparison 4 Update Interrupt Enable"]
pub type CMPU4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPU5` writer - Comparison 5 Update Interrupt Enable"]
pub type CMPU5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPU6` writer - Comparison 6 Update Interrupt Enable"]
pub type CMPU6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPU7` writer - Comparison 7 Update Interrupt Enable"]
pub type CMPU7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrdy(&mut self) -> WRDY_W<IER2_SPEC, 0> {
        WRDY_W::new(self)
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<IER2_SPEC, 3> {
        UNRE_W::new(self)
    }
    #[doc = "Bit 8 - Comparison 0 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm0(&mut self) -> CMPM0_W<IER2_SPEC, 8> {
        CMPM0_W::new(self)
    }
    #[doc = "Bit 9 - Comparison 1 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm1(&mut self) -> CMPM1_W<IER2_SPEC, 9> {
        CMPM1_W::new(self)
    }
    #[doc = "Bit 10 - Comparison 2 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm2(&mut self) -> CMPM2_W<IER2_SPEC, 10> {
        CMPM2_W::new(self)
    }
    #[doc = "Bit 11 - Comparison 3 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm3(&mut self) -> CMPM3_W<IER2_SPEC, 11> {
        CMPM3_W::new(self)
    }
    #[doc = "Bit 12 - Comparison 4 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm4(&mut self) -> CMPM4_W<IER2_SPEC, 12> {
        CMPM4_W::new(self)
    }
    #[doc = "Bit 13 - Comparison 5 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm5(&mut self) -> CMPM5_W<IER2_SPEC, 13> {
        CMPM5_W::new(self)
    }
    #[doc = "Bit 14 - Comparison 6 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm6(&mut self) -> CMPM6_W<IER2_SPEC, 14> {
        CMPM6_W::new(self)
    }
    #[doc = "Bit 15 - Comparison 7 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm7(&mut self) -> CMPM7_W<IER2_SPEC, 15> {
        CMPM7_W::new(self)
    }
    #[doc = "Bit 16 - Comparison 0 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu0(&mut self) -> CMPU0_W<IER2_SPEC, 16> {
        CMPU0_W::new(self)
    }
    #[doc = "Bit 17 - Comparison 1 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu1(&mut self) -> CMPU1_W<IER2_SPEC, 17> {
        CMPU1_W::new(self)
    }
    #[doc = "Bit 18 - Comparison 2 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu2(&mut self) -> CMPU2_W<IER2_SPEC, 18> {
        CMPU2_W::new(self)
    }
    #[doc = "Bit 19 - Comparison 3 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu3(&mut self) -> CMPU3_W<IER2_SPEC, 19> {
        CMPU3_W::new(self)
    }
    #[doc = "Bit 20 - Comparison 4 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu4(&mut self) -> CMPU4_W<IER2_SPEC, 20> {
        CMPU4_W::new(self)
    }
    #[doc = "Bit 21 - Comparison 5 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu5(&mut self) -> CMPU5_W<IER2_SPEC, 21> {
        CMPU5_W::new(self)
    }
    #[doc = "Bit 22 - Comparison 6 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu6(&mut self) -> CMPU6_W<IER2_SPEC, 22> {
        CMPU6_W::new(self)
    }
    #[doc = "Bit 23 - Comparison 7 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu7(&mut self) -> CMPU7_W<IER2_SPEC, 23> {
        CMPU7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Interrupt Enable Register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER2_SPEC;
impl crate::RegisterSpec for IER2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier2::W`](W) writer structure"]
impl crate::Writable for IER2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER2 to value 0"]
impl crate::Resettable for IER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
