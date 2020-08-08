.intel_syntax noprefix
.global main
main:
   push 7

   push 5

   pop rdi
   pop rax
   add rax, rdi
   push rax

   pop rax
   ret
