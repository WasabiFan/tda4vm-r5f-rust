__DDR_START__ = 0xA6000000;
__DDR_DMA_RESERVED__ = 0x100000;
__RESOURCE_TABLE_LENGTH__ = 0x1000;
__DDR_LEN__ = 0x01000000;

MEMORY
{
    TCMA (rwx)         : ORIGIN = 0x0,                                                              LENGTH = 0x8000
    /* TODO: Make DRAM address more configurable (ideally, not dependent on a specific dts and core) */
    DDR_0 (rwx)        : ORIGIN = __DDR_START__ + __DDR_DMA_RESERVED__ + __RESOURCE_TABLE_LENGTH__, LENGTH = __DDR_LEN__ - __DDR_DMA_RESERVED__ - __RESOURCE_TABLE_LENGTH__
    RESOURCE_TABLE (r) : ORIGIN = __DDR_START__ + __DDR_DMA_RESERVED__,                             LENGTH = __RESOURCE_TABLE_LENGTH__
}

SECTIONS
{
    /* TODO: Combining resource tables from multiple sources wouldn't work like this */
    .resource_table :
    {
        *(.ddr.resource_table)
    } > DDR_0

    /* ISRs vector table must come first in TCMA */
    .isr_vector :
    {
        . = ALIGN(4);
        *(.tcm.isr_vector)
    } > TCMA

    .log_shared_mem :
    {
        . = ALIGN(4);
        *(.ddr.log_shared_mem)
    } > DDR_0

    .ddr.text :
    {
        . = ALIGN(4);
        *(.ddr.text)
    } > DDR_0

    .tcm.text :
    {
        . = ALIGN(4);
        *(.tcm.text)
    } > TCMA

    /* TODO: debugging sections -- .ARM.extab and .ARM.exidx */

    .ddr.data :
    {
        . = ALIGN(4);
        *(.ddr.data)
    } > DDR_0

    .tcm.data :
    {
        . = ALIGN(4);
        *(.tcm.data)
    } > TCMA

    .ddr.bss :
    {
        . = ALIGN(4);
        *(.ddr.bss)
    } > DDR_0

    .tcm.bss :
    {
        . = ALIGN(4);
        *(.tcm.bss)
    } > TCMA

    __StackTop = ORIGIN(TCMA) + LENGTH(TCMA);
}
