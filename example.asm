main:
  ; reg3=0, reg2 = 10
  ; i = 0, n = 10
  mov reg1, 10
  mov reg2, reg1
  mov reg1, 0
  mov reg3, reg1
  
loop:
  ; print i
  out reg3

  ; i++
  mov reg1, 1
  add reg3, reg1

  ; jmp loop if i < n (10)
  cmp reg3, reg2
  mov reg1, loop
  jl

