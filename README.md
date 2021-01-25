# YaOS
Creating a hobby operating system in Rust using following blog series: [Writing an OS in Rust](https://os.phil-opp.com/)

# Build

Just run
- ```./configure``` to install the dependencies
- ```make run``` to start the OS

# Requirements
- ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` (to install Rust)

All the other requirements gets installed by ```./configure```. But if you're interested, those are:
- qemu-system-x86 (for emulation of OS)
- nasm (for compilation of assembly files)
- xorriso (for building the grub-iso)
- rust-src (needed to build core crate)