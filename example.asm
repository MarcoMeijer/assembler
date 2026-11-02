; start of program
mov rsp, 1000
jmp main


sum_of:
  push rbp
  mov rbp, rsp

  ; mov [rbp - 20], cx
  sub tx, rbp, 20
  mov [tx], cx

  ; mov [rbp - 4], 0
  mov ax, 0
  sub tx, rbp, 4
  mov [tx], ax

  ; mov [rbp - 8], 0
  mov ax, 0
  sub tx, rbp, 8
  mov [tx], ax

  jmp L2
L3:
  ; mov ax, [rbp - 8]
  sub tx, rbp, 8
  mov ax, [tx]

  ; add [rbp - 4], ax
  sub tx, rbp, 4
  mov bx, [tx]
  add ax, bx
  mov [tx], ax

  ; add [rbp - 8], 1
  sub tx, rbp, 8
  mov ax, [tx]
  add ax, 1
  sub tx, rbp, 8
  mov [tx], ax

L2:
  ; mov ax, [rbp - 8]
  sub tx, rbp, 8
  mov ax, [tx]

  ; cmp ax, [rbp - 20]
  sub tx, rbp, 20
  mov bx, [tx]
  cmp ax, bx

  jng L3

  ; mov ax, [rbp - 4]
  sub tx, rbp, 4
  mov ax, [tx]

  pop rbp
  ret


main:
  mov cx, 5
  call sum_of
  out ax

