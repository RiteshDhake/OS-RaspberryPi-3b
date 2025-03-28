/* linker.ld */
ENTRY(_start)

SECTIONS
{
    /* Place the .init section (containing _start and vector table) at 0x80000 */
    .init 0x80000 : {
        KEEP(*(.init))
    }

    /* Code and read-only data */
    .text : {
        _stext = .;
        *(.text*)
        *(.rodata*)
        _etext = .;
    }

    /* Initialized data */
    .data : {
        _sdata = .;
        *(.data*)
        _edata = .;
    }

    /* Uninitialized data */
    .bss : {
        _sbss = .;
        *(.bss*)
        *(COMMON)
        _ebss = .;
    }

    /* Mark the end of the kernel image */
    _end = .;
}
