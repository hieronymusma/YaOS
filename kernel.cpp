#include "types.h"
#include "gdt.h"

uint16_t * const VideoMemory = (uint16_t *)0xb8000;

void scroll(void) {
    for(int y = 1; y < 25; y++) {
        for (int x = 0; x < 80; x++) {
            VideoMemory[80 * (y - 1) + x] = VideoMemory[80 * y + x];
        }
    }
}

void kprint(const char *str)
{
    static uint8_t x = 0, y = 0;

    for (int i = 0; str[i] != '\0'; ++i)
    {

        switch (str[i])
        {
        case '\n':
            y++;
            x = 0;
            break;
        default:
            VideoMemory[80 * y + x] = (VideoMemory[80 * y + x] & 0xFF00) | str[i];
            x++;
        }

        if (x >= 80)
        {
            y++;
            x = 0;
        }

        if (y >= 25)
        {
            scroll();
            y = 24;
        }
    }
}

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

    for (int i = 0; i < 20; i++) {
        kprint("First Line\n");
    }

    for (int i = 0; i < 6; i++) {
        kprint("Second Line \n");
    }

    while (1)
        ;
}
