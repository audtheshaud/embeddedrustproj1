/* Linker script for the PICO-ICE */
MEMORY
{
  FLASH (rx) : ORIGIN = 0x10000000, LENGTH = 4M
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 256K
}

SECTIONS
{
  .text : { *(.text*) } > FLASH
  .rodata : { *(.rodata*) } > FLASH
  .bss : { *(.bss*) } > RAM
  .data : { *(.data*) } > RAM
}
