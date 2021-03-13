global start
extern long_mode_start

KERNEL_OFFSET EQU 0xC0000000

section .text
bits 32
start:
    ; set stack pointer
    mov esp, stack_top - KERNEL_OFFSET

    ; move multiboot information into rdi so its accessible in rust
    mov edi, ebx

    call check_multiboot ; Error code: 0
    call check_cpuid ; Error code: 1
    call check_long_mode ; Error code: 2

    call set_up_page_tables
    call enable_paging

    ; load 64-bit gdt
    lgdt [gdt64.pointer]

    ; jump into 64 bit code
    jmp gdt64.code:long_mode_start

halt:
    hlt

set_up_page_tables:
    ; map first P4 entry to P3 table
    mov eax, (p3_table - KERNEL_OFFSET)
    or eax, 0b11 ; present + writable
    mov [p4_table - KERNEL_OFFSET], eax
    ;mov [p4_table + 511 * 8], eax ; Access pyhsical memory at 0xffffff8000000000

    ; map first P3 entry to P2 table
    mov eax, p2_table - KERNEL_OFFSET
    or eax, 0b11 ; present + writable
    mov [p3_table - KERNEL_OFFSET], eax

    mov [p3_table - KERNEL_OFFSET + (3 * 8)], eax ; Map physical memory at 0xC0000000

    ;  map each P2 entry to a huge 2MiB page
    mov ecx, 0 ; counter variable

.map_p2_table:
    ; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
    mov eax, 0x200000               ; 2MiB
    mul ecx                         ; start address of exc-th page
    or eax, 0b10000011              ; present + writable + huge (2MiB size)
    mov [(p2_table - KERNEL_OFFSET) + ecx * 8], eax   ; map ecx-th entry (each entry is 8 byte long)

    inc ecx                         ; increment counter
    cmp ecx, 512                    ; if counter == 512, the whole P2 table is mapped
    jne .map_p2_table               ; else map the next entry

    ret

enable_paging:
    ; load P4 to cr3 register
    mov eax, p4_table - KERNEL_OFFSET
    mov cr3, eax

    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

; Prints "ERR: " and the given error code
; parameter: error code (in ascii) in al
error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

; Perform check if kernel was loaded by multiboot header
check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp error

; Perform check if cpuid feature is available
check_cpuid:
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
    ; in the FLAGS register. If we can flip it, CPUID is available.

    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
    ; ID bit back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit
    ; wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error

check_long_mode:
    ; test if extended processor info in available
    mov eax, 0x80000000    ; implicit argument for cpuid
    cpuid                  ; get highest supported argument
    cmp eax, 0x80000001    ; it needs to be at least 0x80000001
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; argument for extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; test if the LM-bit is set in the D-register
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error

section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 4096 * 16
stack_top:

section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
    dw $ - gdt64 - 1    ; gdt length
    dq gdt64            ; actual pointer into gdt64
