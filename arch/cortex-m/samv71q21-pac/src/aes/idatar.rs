#[doc = "Register `IDATAR[%s]` writer"]
pub type W = crate::W<IDATAR_SPEC>;
#[doc = "Field `IDATA` writer - Input Data Word"]
pub type IDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data Word"]
    #[inline(always)]
    #[must_use]
    pub fn idata(&mut self) -> IDATA_W<IDATAR_SPEC, 0> {
        IDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idatar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATAR_SPEC;
impl crate::RegisterSpec for IDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idatar::W`](W) writer structure"]
impl crate::Writable for IDATAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDATAR[%s]
to value 0"]
impl crate::Resettable for IDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
