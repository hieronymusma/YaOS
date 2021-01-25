global long_mode_start

section .text
bits 64
long_mode_start:
    ; load 0 into all data segment registers
    ; most instruction ignore these but some (iretq) expect null descriptor in those
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; call the rust main
    extern _start
    call _start

    ; should not return from here (error 3)
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], "3"
    hlt