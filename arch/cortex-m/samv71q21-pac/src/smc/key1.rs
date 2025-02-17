#[doc = "Register `KEY1` writer"]
pub type W = crate::W<KEY1_SPEC>;
#[doc = "Field `KEY1` writer - Off-Chip Memory Scrambling (OCMS) Key Part 1"]
pub type KEY1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Off-Chip Memory Scrambling (OCMS) Key Part 1"]
    #[inline(always)]
    #[must_use]
    pub fn key1(&mut self) -> KEY1_W<KEY1_SPEC, 0> {
        KEY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY1_SPEC;
impl crate::RegisterSpec for KEY1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key1::W`](W) writer structure"]
impl crate::Writable for KEY1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for KEY1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
