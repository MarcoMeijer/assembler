; start of program
mov rsp, 0
jmp main


function:
  mov ax, 24
  add ax, 4
  ret


main:
  call function
  out ax

