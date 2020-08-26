GCCPARAMS = -m32 -fno-use-cxa-atexit -nostdlib -fno-builtin -fno-rtti -fno-exceptions -fno-leading-underscore
ASPARAMS = --32
LDPARAMS = -melf_i386

objects = loader.o kernel.o gdt.o

%.o: %.cpp
	gcc $(GCCPARAMS) -c -o $@ $<

%.o: %.s
	as $(ASPARAMS) -o $@ $<

yaos.bin: linker.ld $(objects)
	ld $(LDPARAMS) -T $< -o $@ $(objects)

yaos.iso: yaos.bin
	mkdir -p iso/boot/grub
	cp configuration/grub.cfg iso/boot/grub/grub.cfg
	cp yaos.bin iso/boot/yaos.bin
	grub-mkrescue --output=yaos.iso iso
	rm -rf iso

run: yaos.iso
	(killall VirtualBoxVM && sleep 1) || true
	virtualboxvm --startvm 'yaos' &

close:
	killall VirtualBoxVM

install: yaos.bin
	sudo cp $< /boot/yaos.bin

clean:
	rm -f $(objects)
	rm -f yaos.bin
	rm -f yaos.iso