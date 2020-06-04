target remote :3333
set print asm-demangle on
set print pretty on
monitor tpiu config internal itm.txt uart off 8000000
monitor arm semihosting enable
monitor itm port 0 on
load
break DefaultHandler
break UserHardFault
break main
break rust_begin_unwind
continue
continue
