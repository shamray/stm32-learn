unsafe extern "C" {
    fn BusFault_Handler();
    fn MemManage_Handler();
    fn PendSV_Handler();
    fn SVCall_Handler();
    fn SysTick_Handler();
    fn UsageFault_Handler();
    fn ADC_Handler();
    fn DMA1_Stream0_Handler();
    fn DMA1_Stream1_Handler();
    fn DMA1_Stream2_Handler();
    fn DMA1_Stream3_Handler();
    fn DMA1_Stream4_Handler();
    fn DMA1_Stream5_Handler();
    fn DMA1_Stream6_Handler();
    fn DMA1_Stream7_Handler();
    fn DMA2_Stream0_Handler();
    fn DMA2_Stream1_Handler();
    fn DMA2_Stream2_Handler();
    fn DMA2_Stream3_Handler();
    fn DMA2_Stream4_Handler();
    fn DMA2_Stream5_Handler();
    fn DMA2_Stream6_Handler();
    fn DMA2_Stream7_Handler();
    fn EXTI0_Handler();
    fn EXTI15_10_Handler();
    fn EXTI16_PVD_Handler();
    fn EXTI17_RTC_Alarm_Handler();
    fn EXTI18_OTG_FS_WKUP_Handler();
    fn EXTI1_Handler();
    fn EXTI22_RTC_WKUP_Handler();
    fn EXTI2_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();
    fn EXTI9_5_Handler();
    fn FLASH_Handler();
    fn I2C1_ER_Handler();
    fn I2C1_EV_Handler();
    fn I2C2_ER_Handler();
    fn I2C2_EV_Handler();
    fn I2C3_ER_Handler();
    fn I2C3_EV_Handler();
    fn OTG_FS_Handler();
    fn RCC_Handler();
    fn SDIO_Handler();
    fn SPI1_Handler();
    fn SPI2_Handler();
    fn SPI3_Handler();
    fn SPI4_Handler();
    fn TAMP_STAMP_Handler();
    fn TIM1_BRK_TIM9_Handler();
    fn TIM1_CC_Handler();
    fn TIM1_TRG_COM_TIM11_Handler();
    fn TIM1_UP_TIM10_Handler();
    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();
    fn TIM5_Handler();
    fn USART1_Handler();
    fn USART2_Handler();
    fn USART6_Handler();
}

#[used]
#[unsafe(link_section = ".isr_vector")]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 100] = [
    Some(reset_handler),
    Some(nmi_handler),
    Some(hardfault_handler),
    Some(MemManage_Handler),
    Some(BusFault_Handler),
    Some(UsageFault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    None,
    Some(EXTI16_PVD_Handler),
    Some(TAMP_STAMP_Handler),
    Some(EXTI22_RTC_WKUP_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_Stream0_Handler),
    Some(DMA1_Stream1_Handler),
    Some(DMA1_Stream2_Handler),
    Some(DMA1_Stream3_Handler),
    Some(DMA1_Stream4_Handler),
    Some(DMA1_Stream5_Handler),
    Some(DMA1_Stream6_Handler),
    Some(ADC_Handler),
    None,
    None,
    None,
    None,
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM9_Handler),
    Some(TIM1_UP_TIM10_Handler),
    Some(TIM1_TRG_COM_TIM11_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_Handler),
    Some(USART2_Handler),
    None,
    Some(EXTI15_10_Handler),
    Some(EXTI17_RTC_Alarm_Handler),
    Some(EXTI18_OTG_FS_WKUP_Handler),
    None,
    None,
    None,
    None,
    Some(DMA1_Stream7_Handler),
    None,
    Some(SDIO_Handler),
    Some(TIM5_Handler),
    Some(SPI3_Handler),
    None,
    None,
    None,
    None,
    Some(DMA2_Stream0_Handler),
    Some(DMA2_Stream1_Handler),
    Some(DMA2_Stream2_Handler),
    Some(DMA2_Stream3_Handler),
    Some(DMA2_Stream4_Handler),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(OTG_FS_Handler),
    Some(DMA2_Stream5_Handler),
    Some(DMA2_Stream6_Handler),
    Some(DMA2_Stream7_Handler),
    Some(USART6_Handler),
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(SPI4_Handler),
];

#[unsafe(no_mangle)]
extern "C" fn nmi_handler() {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn hardfault_handler() {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn default_handler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn reset_handler() {
    crate::main();
}
