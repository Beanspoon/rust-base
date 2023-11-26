MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 0x80000
    RAM : ORIGIN = 0x20000000, LENGTH = 0x10000
}

ENTRY(Reset);

EXTERN(RESET_VECTOR);

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        // Stack pointer
        LONG(ORIGIN(RAM) + LENGTH(RAM));

        // Reset vector
        KEEP(*(.vector_table.reset_vector))
    } > FLASH

    .text :
    {
        *(.text .text.*)
    } > FLASH

    .rodata :
    {
        *(.rodata .rodata.*);
    } > FLASH

    .bss :
    {
        *(.bss .bss.*);
    } > RAM

    .data :
    {
        *(.data .data.*);
    } > RAM

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*)
    }
}