MEMORY
{
  /* 
  Different settings are required, depending on if you are
  using SWD or UF2 bootloader
  */
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 1024K /* SWD */
  /* FLASH (rx) : ORIGIN = 0x00001000, LENGTH = 1020K /* UF2 */
  /* FLASH (rx)     : ORIGIN = 0x27000, LENGTH = 0xED000 - 0x27000 /* UF2 + softdevice */
  RAM (rwx)   : ORIGIN = 0x20000000, LENGTH = 256K /* w/o softdevice */
  /* RAM (rwx) :  ORIGIN = 0x20006000, LENGTH = 0x20040000 - 0x20006000 /* w/ softdevice */
}
