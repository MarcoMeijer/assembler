; start of program
mov rsp, 0
jmp main


add:
  push rbp
  mov rbp, rsp

  ; mov [rbp - 2], cx
  mov tx, 2
  add tx, rbp, tx
  mov [tx], cx

  ; mov ax, [rbp - 2]
  mov tx, 2
  add tx, rbp, tx
  mov ax, [tx]

  add ax, 2

  pop rbp
  ret


main:
  mov cx, 20
  call add
  out ax

