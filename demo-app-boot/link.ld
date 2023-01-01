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
    /* system-level metadata ===================== */
    .resource_table :
    {
        KEEP(*(.resource_table));
        *(.resource_table)
    } > RESOURCE_TABLE

    .isr_vector :
    {
        . = 0x0;
        KEEP(*(.isr_vector))
    } > TCMA

    .log_shared_mem :
    {
        /* TODO: needed? */
        _log_shared_mem_addr = .;
        *(.log_shared_mem*)
    } > DDR_0

    /* DDR RAM: main app ========================= */
    .ddr.text :
    {
        /* explicit placement */
        *(.ddr.text*)

        /* dupes of unqualified .text rules, for app lib */
        */libdemo_app.a:*(.text*)

        KEEP(*/libdemo_app.a:*(.init))
        KEEP(*/libdemo_app.a:*(.fini))

        /* .ctors */
        */libdemo_app.a:*crtbegin.o(.ctors)
        */libdemo_app.a:*crtbegin?.o(.ctors)
        */libdemo_app.a:*(EXCLUDE_FILE(*crtend?.o *crtend.o) .ctors)
        */libdemo_app.a:*(SORT(.ctors.*))
        */libdemo_app.a:*(.ctors)

        /* .dtors */
        */libdemo_app.a:*crtbegin.o(.dtors)
        */libdemo_app.a:*crtbegin?.o(.dtors)
        */libdemo_app.a:*(EXCLUDE_FILE(*crtend?.o *crtend.o) .dtors)
        */libdemo_app.a:*(SORT(.dtors.*))
        */libdemo_app.a:*(.dtors)

        */libdemo_app.a:*(.rodata*)

        KEEP(*/libdemo_app.a:*(.eh_frame*))
    } > DDR_0

    .data :
    {
        . = ALIGN (4);
        */libdemo_app.a:*(vtable)
        */libdemo_app.a:*(.data*)

        . = ALIGN(4);
        /* preinit data */
        KEEP(*/libdemo_app.a:*(.preinit_array))

        . = ALIGN(4);
        /* init data */
        KEEP(*/libdemo_app.a:*(SORT(.init_array.*)))
        KEEP(*/libdemo_app.a:*(.init_array))

        . = ALIGN(4);
        /* finit data */
        KEEP(*/libdemo_app.a:*(SORT(.fini_array.*)))
        KEEP(*/libdemo_app.a:*(.fini_array))

        KEEP(*/libdemo_app.a:*(.jcr*))
    } > DDR_0

    .bss :
    {
        . = ALIGN(4);
        */libdemo_app.a:*(.bss*)
        */libdemo_app.a:*(COMMON)
    } > DDR_0

    /* TCMA: boot app ============================ */
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


    .data :
    {
        . = ALIGN (4);
        *(vtable)
        *(.data*)

        . = ALIGN(4);
        /* preinit data */
        KEEP(*(.preinit_array))

        . = ALIGN(4);
        /* init data */
        KEEP(*(SORT(.init_array.*)))
        KEEP(*(.init_array))

        . = ALIGN(4);
        /* finit data */
        KEEP(*(SORT(.fini_array.*)))
        KEEP(*(.fini_array))

        KEEP(*(.jcr*))
    } > TCMA

    .bss :
    {
        . = ALIGN(4);
        *(.bss*)
        *(COMMON)
    } > TCMA

    .heap (COPY):
    {
        . = ALIGN(4);
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
