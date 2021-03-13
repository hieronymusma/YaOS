ENTRY(start)

KERNEL_OFFSET = 0xC0000000;
KERNEL_START = 0x10000;

SECTIONS {
  . = KERNEL_START + KERNEL_OFFSET;

  .rodata ALIGN(4K) : AT(ADDR(.rodata)-KERNEL_OFFSET)
  {
    /* ensure that the multiboot header is at the beginning */
    KEEP(*(.multiboot_header))
    *(.rodata .rodata.*)
  }

  .text ALIGN(4K) : AT(ADDR(.text)-KERNEL_OFFSET)
  {
    *(.text .text.*)
  }

  .data ALIGN(4K) : AT(ADDR(.data)-KERNEL_OFFSET)
  {
    *(.data .data.*)
  }

  .bss ALIGN(4K) : AT(ADDR(.bss)-KERNEL_OFFSET)
  {
    *(.bss .bss.*)
  }
  
  .data.rel.ro ALIGN(4K) : AT(ADDR(.data.rel.ro)-KERNEL_OFFSET)
  {
    *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
  }
}