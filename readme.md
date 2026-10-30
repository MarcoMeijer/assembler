
# How to run

Make sure you have rust installed: https://rust-lang.org/tools/install/

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
