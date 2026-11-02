
# How to run

Make sure you have rust installed: https://rust-lang.org/tools/install/

## 16 bit machine (homework 5)
To run the code in a virtual 16bit machine run this:
```
cargo run -- --run
```
This will run `example.asm` (modify this file with your own code).

Available registers:
```
ax, bx, cx, dx - general purpose registers
rbp - Register Base Pointer
rsp - Register Stack Pointer
rip - Register Instruction Pointer
tx - Temporary register, avoid using it as much as possible as other instructions use it
```

Available operations:
```asm
; move operations
mov register, constant ; move constant into register
mov a, b ; move register b to register a
mov [address], register ; write register to ram at address 
mov register, [address] ; read from ram at address into register
mov [address - const], register ; write register to ram at [address - const]
mov register, [address - const] ; read from ram at [address - const] into register

out register ; prints register to the console (Use this for debugging!)

push register ; push register to stack
pop register ; pop from stack and store value in register
jmp label ; jump to label
call label ; call function with label
ret ; return from function

cmp a, b ; compare register a to register b
je label ; jump to label if last comparison was equal
jne label ; jump to label if last comparison was NOT equal
jl label ; jump to label if last comparison was a < b
jnl label ; jump to label if last comparison was NOT a < b
jg label ; jump to label if last comparison was a > b
jng label ; jump to label if last comparison was a NOT > b

add a, b ; a + b and store result into a
sub a, b ; a - b and store result into a
or a, b ; a | b and store result into a
nor a, b ; ~(a | b) and store result into a
and a, b ; a & b and store result into a
nand a, b ; ~(a & b) and store result into a

add d, a, b ; a + b and store result into d
sub d, a, b ; a - b and store result into d
or d, a, b ; a | b and store result into d
nor d, a, b ; ~(a | b) and store result into d
and d, a, b ; a & b and store result into d
nand d, a, b ; ~(a & b) and store result into d
```

## 8bit machine
To compile to 8bit machine code (the machine we made in the lesson)
```
cargo run -- --bit8
```

Available registers:
```
reg1 - reg6
```

Available instructions 8bit machine:
```asm
mov A, C ; Moves constant C into register A
mov A, B ; Moves register A into register B
mov [reg2], reg1 ; Store reg1 into ram at address reg2
mov reg1, [reg2] ; Reads ram address reg2 and store value in reg1

add DEST, reg1, reg2 ; Performs reg1 + reg2 and stores result into register DEST
sub DEST, reg1, reg2 ; Performs reg1 - reg2 and stores result into register DEST
and DEST, reg1, reg2 ; Performs reg1 & reg2 (bitwise and) and stores result into register DEST
nand DEST, reg1, reg2 ; Performs ~(reg1 & reg2) (bitwise nand) and stores result into register DEST
or DEST, reg1, reg2 ; Performs reg1 | reg2 (bitwise or) and stores result into register DEST
xor DEST, reg1, reg2 ; Performs reg1 ^ reg2 (bitwise xor) and stores result into register DEST
eq DEST, reg1, reg2 ; Performs reg1 == reg2 and stores result into register DEST
```
