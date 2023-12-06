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
        KEEP(*(.vector_table.reset_vector));

        // Exceptions
        KEEP(*(.vector_table.exceptions));
    } > FLASH

    .text :
    {
        *(.text .text.*)
    } > FLASH

    .rodata :
    {
        *(.rodata .rodata.*);
        . = ALIGN(4);
        _erodata = .;
    } > FLASH

    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss .bss.*);
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .data : AT(_erodata)
    {
        . = ALIGN(4);
        _sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
        _edata = .;
    } > RAM

    _sidata = LOADADDR(.data);

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*)
    }
}

PROVIDE(nmi_handler = default_exception_handler);
PROVIDE(hard_fault_handler = default_exception_handler);
PROVIDE(memory_management_fault_handler = default_exception_handler);
PROVIDE(bus_fault_handler = default_exception_handler);
PROVIDE(usage_fault_handler = default_exception_handler);
PROVIDE(sv_call_handler = default_exception_handler);
PROVIDE(pending_sv_handler = default_exception_handler);
PROVIDE(systick_handler = default_exception_handler);