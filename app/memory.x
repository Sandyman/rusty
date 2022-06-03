# This file must be named memory.x because link.x relies on it
MEMORY
{
  /* Flash memory */
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  /* SRAM */
  RAM : ORIGIN = 0x20000000, LENGTH = 12K
  /* CCM RAM (we'll probably need our own link.x to use this */
  RAM2 : ORIGIN = 0x10000000, LENGTH = 4K
}
