; start of program
mov tx, 0
mov rsp, tx
jmp main


function:
  mov tx, 24
  mov ax, tx
  ret


main:
  call function
  out ax

