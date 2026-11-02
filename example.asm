; start of program
mov rsp, 1000
jmp main


sum_of:
  push rbp
  mov rbp, rsp
  mov [rbp - 20], cx

  ; mov [rbp - 4], 0
  mov ax, 0
  mov [rbp - 4], ax

  ; mov [rbp - 8], 0
  mov ax, 0
  mov [rbp - 8], ax

  jmp L2
L3:
  mov ax, [rbp - 8]

  ; add [rbp - 4], ax
  mov bx, [rbp - 4]
  add ax, bx
  mov [rbp - 4], ax

  ; add [rbp - 8], 1
  mov ax, [rbp - 8]
  add ax, 1
  mov [rbp - 8], ax

L2:
  mov ax, [rbp - 8]
  mov bx, [rbp - 20]
  cmp ax, bx
  jng L3

  mov ax, [rbp - 4]
  pop rbp
  ret


main:
  mov cx, 5
  call sum_of
  out ax

