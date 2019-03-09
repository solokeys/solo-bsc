MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* 14K offset for bootloader, 38K at end for data, 8 option bytes */
  FLASH (rx) : ORIGIN = 0x08005000, LENGTH = 192K /* Err.. length calculation TODO */
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 48K
  SRAM2 (rw) : ORIGIN = 0x10000000, LENGTH = 16K
}
