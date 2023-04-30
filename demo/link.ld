__DDR_START__ = 0xA6000000;

MEMORY
{
    TCMA (rwx) : ORIGIN = 0x0, LENGTH = 0x8000
    /* TODO: Make DRAM address more configurable (ideally, not dependent on a specific dts and core) */
    DDR_0 (rwx) : ORIGIN = __DDR_START__ + 0x101000, LENGTH = 0xEFF000
    RESOURCE_TABLE (r) : ORIGIN = __DDR_START__ + 0x100000, LENGTH = 0x1000
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
        *(.tcm.isr_vector)
    } > TCMA

    .log_shared_mem :
    {
        *(.ddr.log_shared_mem)
    } > DDR_0

    .ddr.text :
    {
        *(.ddr.text)
    } > DDR_0

    .tcm.text :
    {
        *(.tcm.text)
    } > TCMA

    /* TODO: debugging sections -- .ARM.extab and .ARM.exidx */

    .ddr.data :
    {
        *(.ddr.data)
    } > DDR_0

    .tcm.data :
    {
        *(.tcm.data)
    } > TCMA

    .ddr.bss :
    {
        *(.ddr.bss)
    } > DDR_0

    .tcm.bss :
    {
        *(.tcm.bss)
    } > TCMA

    __StackTop = ORIGIN(TCMA) + LENGTH(TCMA);
}
