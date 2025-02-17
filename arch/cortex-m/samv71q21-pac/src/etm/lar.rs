#[doc = "Register `LAR` writer"]
pub type W = crate::W<LAR_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<LAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
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
#[doc = "ETM Lock Access Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LAR_SPEC;
impl crate::RegisterSpec for LAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lar::W`](W) writer structure"]
impl crate::Writable for LAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
