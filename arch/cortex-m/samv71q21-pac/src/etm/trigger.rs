#[doc = "Register `TRIGGER` reader"]
pub type R = crate::R<TRIGGER_SPEC>;
#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TRIGGER_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TRIGGER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ETM Trigger Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigger::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
