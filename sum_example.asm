; start of program
mov rsp, 1000
jmp main


;   int sum(int n) {
;     if (n <= 1) {
;       return n;
;     } else {
;       return sum(n - 1) + n;
;     }
;   }

; Argument n is passed in as dx
; Return value is put in ax
sum:
  ; n <= 1
  mov ax, 1
  cmp dx, ax ; compare n to 1 (dx=n, ax=1)
  jg normal_case ; jump to normal_case if n > 1

  ; n is <= 1
  mov ax, dx ; set return value to n
  ret ; return from function

  normal_case:
    ; call sum(n - 1)
    push dx ; save dx on the stack so we can get it back after the function call
    sub dx, 1 ; dx is now equal to n - 1 (dx = dx - 1)
    call sum ; call sum(n - 1)
    pop dx ; get back old value of dx from the stack

    ; ax = sum(n-1)
    ; dx = n
    add ax, dx ; ax is now sum(n - 1) + n (ax = ax + dx)
    ret ; return value is ax


main:
  mov dx, 10 ; argument passed to sum function
  call sum ; call sum(10)
  out ax ; print result of sum(10)


