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
  cmp dx, ax ; compare n to 1 dx=n, ax=1
  jg normal_case

  ; n is <= 1
  mov ax, dx ; return n
  ret

  normal_case:
    ; call sum(n - 1)
    push dx
    sub dx, dx, 1
    call sum
    pop dx

    ; ax = sum(n-1)
    ; dx = n
    ; return sum(n - 1) + n;
    add ax, dx
    ret


main:
  mov dx, 10
  call sum
  out ax


