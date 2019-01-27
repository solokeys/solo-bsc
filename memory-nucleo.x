MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* 14K offset for bootloader, 38K at end for data */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K /* maybe 238K, leaving out 18K for data */
  RAM   : ORIGIN = 0x20000000, LENGTH = 48K
  SRAM2 : ORIGIN = 0x10000000, LENGTH = 16K
}


/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */
