#include "types.h"
#include "gdt.h"
#include "kernel_stdio.h"

typedef void (*constructor)();
extern "C" constructor start_ctors;
extern "C" constructor end_ctors;
extern "C" void callConstructors()
{
    for (constructor *i = &start_ctors; i != &end_ctors; i++)
        (*i)();
}

extern "C" void kernelMain(const void *multiboot_structure, uint32_t multibootmagic)
{
    GlobalDescriptorTable gdt;

    for (int i = 0; i < 20; i++)
    {
        kprint("First Line\n");
    }

    for (int i = 0; i < 6; i++)
    {
        kprint("Second Line \n");
    }

    while (1)
        ;
}
