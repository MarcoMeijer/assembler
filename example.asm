; start of program
mov rsp, 1000
jmp main


assign:
  push rbp
  mov rbp, rsp
  mov ax, 40
  mov [rbp - 4], ax
  mov ax, [rbp - 4]
  pop rbp
  ret


adder:
  push rbp
  mov rbp, rsp
  mov ax, [rbp - 4]
  add ax, 2
  pop rbp
  ret


main:
  push rbp
  mov rbp, rsp
  sub rsp, 16
  call assign
  call adder
  mov [rbp - 4], ax
  mov ax, [rbp - 4]
  out ax
  mov rsp, rbp
  pop rbp


