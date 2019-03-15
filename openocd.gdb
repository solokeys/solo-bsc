set history save on
target extended-remote :3333
set print asm-demangle on
monitor reset halt
load
# continue
