#!/bin/bash

openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg &
openocd_pid=$!
gdb-multiarch -ex "target remote localhost:3333" -ex "load" -ex "b main" -ex "continue" $*
kill $openocd_pid