SECTIONS
{
    /* system-level metadata ===================== */

    .ddr.log_shared_mem :
    {
        /* TODO: needed? */
        /* _log_shared_mem_addr = .; */
        *(.log_shared_mem*)
    }

    /* DDR RAM =================================== */
    .ddr.text :
    {
        *(.ddr.text*)
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
    }

    /* TODO: exidx and co. need to be combined */
    /* TODO: Need to place metadata close to corresponding sections so relocations fit */
    .ARM.extab :
    {
        *(.ARM.extab* .gnu.linkonce.armextab.*)
    }
    /* __exidx_start = .; */
    /DISCARD/ :
     /* .ARM.exidx : */
    {
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
    }
    /* __exidx_end = .; */

    .ddr.data :
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
    }

    .ddr.bss :
    {
        . = ALIGN(4);
        *(.bss*)
        *(COMMON)
    }
}
