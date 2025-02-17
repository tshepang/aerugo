#[doc = "Register `IAR` reader"]
pub type R = crate::R<IAR_SPEC>;
#[doc = "Register `IAR` writer"]
pub type W = crate::W<IAR_SPEC>;
#[doc = "Field `ADDR` reader - Address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<IAR_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Instruction Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IAR_SPEC;
impl crate::RegisterSpec for IAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iar::R`](R) reader structure"]
impl crate::Readable for IAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iar::W`](W) writer structure"]
impl crate::Writable for IAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IAR to value 0"]
impl crate::Resettable for IAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
