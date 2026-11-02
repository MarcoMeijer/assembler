; start of program
mov rsp, 1000
jmp main


sum_of:
  push rbp
  mov rbp, rsp

  ; mov [rbp - 20], cx
  sub dx, rbp, 20
  mov [dx], cx

  ; mov [rbp - 4], 0
  mov ax, 0
  sub dx, rbp, 4
  mov [dx], ax

  ; mov [rbp - 8], 0
  mov ax, 0
  sub dx, rbp, 8
  mov [dx], ax

  jmp L2
L3:
  ; mov ax, [rbp - 8]
  sub dx, rbp, 8
  mov ax, [dx]

  ; add [rbp - 4], ax
  sub dx, rbp, 4
  mov bx, [dx]
  add ax, bx
  mov [dx], ax

  ; add [rbp - 8], 1
  sub dx, rbp, 8
  mov ax, [dx]
  add ax, 1
  mov [dx], ax

L2:
  ; mov ax, [rbp - 8]
  sub dx, rbp, 8
  mov ax, [dx]

  ; cmp ax, [rbp - 20]
  sub dx, rbp, 20
  mov bx, [dx]
  cmp ax, bx

  jng L3

  ; mov ax, [rbp - 4]
  sub dx, rbp, 4
  mov ax, [dx]

  pop rbp
  ret


main:
  mov cx, 5
  call sum_of
  out ax

