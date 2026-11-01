; start of program
mov reg1, 0
mov rsp, reg1
jmp main


function:
  mov reg1, 24
  mov reg3, reg1
  ret


main:
  call function
  out reg3

  call function
  out reg3
  
