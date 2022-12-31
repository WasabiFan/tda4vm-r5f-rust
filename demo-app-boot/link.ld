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
    .resource_table :
    {
        KEEP(*(.resource_table));
        *(.resource_table)
    } > RESOURCE_TABLE

    .log_shared_mem :
    {
        /* TODO: needed? */
        _log_shared_mem_addr = .;
        *(.log_shared_mem*)
    } > DDR_0

    .isr_vector :
    {
        . = 0x0;
        KEEP(*(.isr_vector))
    } > TCMA

    .textddr :
    {
        *(.textddr*)
    } > DDR_0

    .text :
    {
        *(.text*)

        KEEP(*(.init))
        KEEP(*(.fini))

        /* .ctors */
        *crtbegin.o(.ctors)
        *crtbegin?.o(.ctors)
        *(EXCLUDE_FILE(*crtend?.o *crtend.o) .ctors)
        *(SORT(.ctors.*))
        *(.ctors)

        /* .dtors */
        *crtbegin.o(.dtors)
        *crtbegin?.o(.dtors)
        *(EXCLUDE_FILE(*crtend?.o *crtend.o) .dtors)
        *(SORT(.dtors.*))
        *(.dtors)

        *(.rodata*)

        KEEP(*(.eh_frame*))
    } > TCMA

    .ARM.extab :
    {
        *(.ARM.extab* .gnu.linkonce.armextab.*)
    } > TCMA

    __exidx_start = .;
    /* TODO: exidx metadata doesn't seem to support far code relocations */
    /DISCARD/ :
    {
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
    } > TCMA
    __exidx_end = .;


    /* Location counter can end up 2byte aligned with narrow Thumb code but
        __etext is assumed by startup code to be the LMA of a section in RAM
        which must be 4byte aligned */
    __etext = ALIGN (4);

    .data : AT (__etext)
    {
        __data_start__ = .;
        *(vtable)
        *(.data*)

        . = ALIGN(4);
        /* preinit data */
        PROVIDE_HIDDEN (__preinit_array_start = .);
        KEEP(*(.preinit_array))
        PROVIDE_HIDDEN (__preinit_array_end = .);

        . = ALIGN(4);
        /* init data */
        PROVIDE_HIDDEN (__init_array_start = .);
        KEEP(*(SORT(.init_array.*)))
        KEEP(*(.init_array))
        PROVIDE_HIDDEN (__init_array_end = .);


        . = ALIGN(4);
        /* finit data */
        PROVIDE_HIDDEN (__fini_array_start = .);
        KEEP(*(SORT(.fini_array.*)))
        KEEP(*(.fini_array))
        PROVIDE_HIDDEN (__fini_array_end = .);

        KEEP(*(.jcr*))
        . = ALIGN(4);
        /* All data end */
        __data_end__ = .;

    } > TCMA

    .bss :
    {
        . = ALIGN(4);
        __bss_start__ = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        __bss_end__ = .;
    } > TCMA

    .heap (COPY):
    {
        __end__ = .;
        PROVIDE(end = .);
        *(.heap*)
        __HeapLimit = .;
    } > TCMA

    /* .stack_dummy section doesn't contains any symbols. It is only
        * used for linker to calculate size of stack sections, and assign
        * values to stack symbols later */
    .stack_dummy (COPY):
    {
        *(.stack*)
    } > TCMA

    /* Set stack top to end of RAM, and stack limit move down by
        * size of stack_dummy section */
    __StackTop = ORIGIN(TCMA) + LENGTH(TCMA);
    __StackLimit = __StackTop - SIZEOF(.stack_dummy);
    PROVIDE(__stack = __StackTop);

    /* Check if data + heap + stack exceeds RAM limit */
    ASSERT(__StackLimit >= __HeapLimit, "region TCMA overflowed with stack")
}
