ENTRY(reset_handler)

MEMORY
{
    FLASH(rx)   : ORIGIN = 0x8000000, LENGTH = 256K
    RAM (rwx)   : ORIGIN = 0x2000000, LENGTH = 64K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;
_min_heap_size = 0x400;

SECTIONS
{
    .text :
    {
        . = ORIGIN(FLASH);
        LONG(_start_of_stack)
        KEEP(*(.isr_vector));
        *(.text)
        *(.text.*)
        . = ALIGN(4);
    } > FLASH

    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata.*)
        . = ALIGN(4);
    } > FLASH

    .data : 
    {
        _sidata = LOADADDR(.data);
        . = ALIGN(4);
        _sdata = .;
        *(.data)
        *(.data.*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT > FLASH

    .bss : 
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss)
        *(.bss.*)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .ram_usage_check : 
    {
        . = ALIGN(8);
        . = . + _min_stack_size;
        . = . + _min_heap_size;
        . = ALIGN(8);
    } > RAM
}

PROVIDE(MemManage_Handler = default_handler);
PROVIDE(BusFault_Handler = default_handler);
PROVIDE(UsageFault_Handler = default_handler);
PROVIDE(SVCall_Handler = default_handler);
PROVIDE(DebugMon_Handler = default_handler);
PROVIDE(PendSV_Handler = default_handler);
PROVIDE(SysTick_Handler = default_handler);
PROVIDE(ADC_Handler = default_handler);
PROVIDE(DMA1_Stream0_Handler = default_handler);
PROVIDE(DMA1_Stream1_Handler = default_handler);
PROVIDE(DMA1_Stream2_Handler = default_handler);
PROVIDE(DMA1_Stream3_Handler = default_handler);
PROVIDE(DMA1_Stream4_Handler = default_handler);
PROVIDE(DMA1_Stream5_Handler = default_handler);
PROVIDE(DMA1_Stream6_Handler = default_handler);
PROVIDE(DMA1_Stream7_Handler = default_handler);
PROVIDE(DMA2_Stream0_Handler = default_handler);
PROVIDE(DMA2_Stream1_Handler = default_handler);
PROVIDE(DMA2_Stream2_Handler = default_handler);
PROVIDE(DMA2_Stream3_Handler = default_handler);
PROVIDE(DMA2_Stream4_Handler = default_handler);
PROVIDE(DMA2_Stream5_Handler = default_handler);
PROVIDE(DMA2_Stream6_Handler = default_handler);
PROVIDE(DMA2_Stream7_Handler = default_handler);
PROVIDE(EXTI0_Handler = default_handler);
PROVIDE(EXTI15_10_Handler = default_handler);
PROVIDE(EXTI16_PVD_Handler = default_handler);
PROVIDE(EXTI17_RTC_Alarm_Handler = default_handler);
PROVIDE(EXTI18_OTG_FS_WKUP_Handler = default_handler);
PROVIDE(EXTI1_Handler = default_handler);
PROVIDE(EXTI22_RTC_WKUP_Handler = default_handler);
PROVIDE(EXTI2_Handler = default_handler);
PROVIDE(EXTI3_Handler = default_handler);
PROVIDE(EXTI4_Handler = default_handler);
PROVIDE(EXTI9_5_Handler = default_handler);
PROVIDE(FLASH_Handler = default_handler);
PROVIDE(I2C1_ER_Handler = default_handler);
PROVIDE(I2C1_EV_Handler = default_handler);
PROVIDE(I2C2_ER_Handler = default_handler);
PROVIDE(I2C2_EV_Handler = default_handler);
PROVIDE(I2C3_ER_Handler = default_handler);
PROVIDE(I2C3_EV_Handler = default_handler);
PROVIDE(OTG_FS_Handler = default_handler);
PROVIDE(RCC_Handler = default_handler);
PROVIDE(SDIO_Handler = default_handler);
PROVIDE(SPI1_Handler = default_handler);
PROVIDE(SPI2_Handler = default_handler);
PROVIDE(SPI3_Handler = default_handler);
PROVIDE(SPI4_Handler = default_handler);
PROVIDE(TAMP_STAMP_Handler = default_handler);
PROVIDE(TIM1_BRK_TIM9_Handler = default_handler);
PROVIDE(TIM1_CC_Handler = default_handler);
PROVIDE(TIM1_TRG_COM_TIM11_Handler = default_handler);
PROVIDE(TIM1_UP_TIM10_Handler = default_handler);
PROVIDE(TIM2_Handler = default_handler);
PROVIDE(TIM3_Handler = default_handler);
PROVIDE(TIM4_Handler = default_handler);
PROVIDE(TIM5_Handler = default_handler);
PROVIDE(USART1_Handler = default_handler);
PROVIDE(USART2_Handler = default_handler);
PROVIDE(USART6_Handler = default_handler);
