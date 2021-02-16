.text
.code64

.global _x86_64_asm_get_cs
.p2align 4
_x86_64_asm_get_cs:
    mov %cs, %ax
    retq

.global _x86_64_asm_lidt
.p2align 4
_x86_64_asm_lidt:
    lidt (%rdi)
    retq