main:
  ; reg1=5, reg2=10
  mov reg1, 10
  mov reg2, reg1
  mov reg1, 5
  
  ; RAM[10] = 5
  mov [reg2], reg1

  ; reg1 = 5 + 10
  add reg1, reg1, reg2

