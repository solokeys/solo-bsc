target remote :3333
# target extended-remote localhost:4242

# print demangled symbols
set print asm-demangle on

# for poking around via `p/x rcc` and friends
set print pretty on

# detect unhandled exceptions, hard faults and panics
#break DefaultHandler
#break UserHardFault
#break rust_begin_unwind

# *try* to stop at the user entry point (it might be gone due to inlining)
# break main

# monitor arm semihosting enable

# # send captured ITM to the file itm.fifo
# # (the microcontroller SWO pin must be connected to the programmer SWO pin)
# # 8000000 must match the core clock frequency
# monitor tpiu config internal itm.txt uart off 8000000

# # OR: make the microcontroller SWO pin output compatible with UART (8N1)
# # 8000000 must match the core clock frequency
# # 2000000 is the frequency of the SWO pin
# monitor tpiu config external uart off 8000000 2000000

# # enable ITM port 0
# monitor itm port 0 on

load

set confirm off

# start the process but immediately halt the processor
# stepi
#
continue