https://www.cs.cmu.edu/afs/cs/academic/class/15213-f00/docs/elf.pdf - the elf file structure

https://www.youtube.com/watch?v=1VnnbpHDBBA - elf file structure and readelf usage

magic bytes: 0x7f, 'E', 'L', 'F'
               7f,  45,  4c,  46
              127,  69,  76,  70

compile a main.c program using:
`gcc -static -static-libgcc -m32 -o main.elf main.c`
the `-m32` flag is used for making it compile x32 (not x64)
and `-static -static-libgcc` flags are for making it a static executable

`readelf -le main.elf`
```
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 03 00 00 00 00 00 00 00 00 
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - GNU
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           Intel 80386
  Version:                           0x1
  Entry point address:               0x8049b70
  Start of program headers:          52 (bytes into file)
  Start of section headers:          707916 (bytes into file)
  Flags:                             0x0
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         9
  Size of section headers:           40 (bytes)
  Number of section headers:         30
  Section header string table index: 29

Section Headers:
  [Nr] Name              Type            Addr     Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            00000000 000000 000000 00      0   0  0
  [ 1] .note.gnu.build-i NOTE            08048154 000154 000024 00   A  0   0  4
  [ 2] .note.gnu.propert NOTE            08048178 000178 00001c 00   A  0   0  4
  [ 3] .note.ABI-tag     NOTE            08048194 000194 000020 00   A  0   0  4
  [ 4] .rel.plt          REL             080481b4 0001b4 000070 08  AI  0  19  4
  [ 5] .init             PROGBITS        08049000 001000 000024 00  AX  0   0  4
  [ 6] .plt              PROGBITS        08049030 001030 0000e0 00  AX  0   0 16
  [ 7] .text             PROGBITS        08049110 001110 069581 00  AX  0   0 16
  [ 8] __libc_freeres_fn PROGBITS        080b26a0 06a6a0 000b8f 00  AX  0   0 16
  [ 9] .fini             PROGBITS        080b3230 06b230 000018 00  AX  0   0  4
  [10] .rodata           PROGBITS        080b4000 06c000 01b324 00   A  0   0 32
  [11] .eh_frame         PROGBITS        080cf324 087324 012e64 00   A  0   0  4
  [12] .gcc_except_table PROGBITS        080e2188 09a188 0000b1 00   A  0   0  1
  [13] .tdata            PROGBITS        080e36a0 09a6a0 000010 00 WAT  0   0  4
  [14] .tbss             NOBITS          080e36b0 09a6b0 000020 00 WAT  0   0  4
  [15] .init_array       INIT_ARRAY      080e36b0 09a6b0 000008 04  WA  0   0  4
  [16] .fini_array       FINI_ARRAY      080e36b8 09a6b8 000008 04  WA  0   0  4
  [17] .data.rel.ro      PROGBITS        080e36c0 09a6c0 001914 00  WA  0   0 32
  [18] .got              PROGBITS        080e4fd4 09bfd4 000024 00  WA  0   0  4
  [19] .got.plt          PROGBITS        080e5000 09c000 000044 04  WA  0   0  4
  [20] .data             PROGBITS        080e5060 09c060 000ec0 00  WA  0   0 32
  [21] __libc_subfreeres PROGBITS        080e5f20 09cf20 000024 00  WA  0   0  4
  [22] __libc_IO_vtables PROGBITS        080e5f60 09cf60 000354 00  WA  0   0 32
  [23] __libc_atexit     PROGBITS        080e62b4 09d2b4 000004 00  WA  0   0  4
  [24] .bss              NOBITS          080e62c0 09d2b8 000d1c 00  WA  0   0 32
  [25] __libc_freeres_pt NOBITS          080e6fdc 09d2b8 000014 00  WA  0   0  4
  [26] .comment          PROGBITS        00000000 09d2b8 00002b 01  MS  0   0  1
  [27] .symtab           SYMTAB          00000000 09d2e4 008a90 10     28 1159  4
  [28] .strtab           STRTAB          00000000 0a5d74 006e9c 00      0   0  1
  [29] .shstrtab         STRTAB          00000000 0acc10 00013a 00      0   0  1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  p (processor specific)

Program Headers:
  Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align
  LOAD           0x000000 0x08048000 0x08048000 0x00224 0x00224 R   0x1000
  LOAD           0x001000 0x08049000 0x08049000 0x6a248 0x6a248 R E 0x1000
  LOAD           0x06c000 0x080b4000 0x080b4000 0x2e239 0x2e239 R   0x1000
  LOAD           0x09a6a0 0x080e36a0 0x080e36a0 0x02c18 0x03950 RW  0x1000
  NOTE           0x000154 0x08048154 0x08048154 0x00060 0x00060 R   0x4
  TLS            0x09a6a0 0x080e36a0 0x080e36a0 0x00010 0x00030 R   0x4
  GNU_PROPERTY   0x000178 0x08048178 0x08048178 0x0001c 0x0001c R   0x4
  GNU_STACK      0x000000 0x00000000 0x00000000 0x00000 0x00000 RW  0x10
  GNU_RELRO      0x09a6a0 0x080e36a0 0x080e36a0 0x01960 0x01960 R   0x1

 Section to Segment mapping:
  Segment Sections...
   00     .note.gnu.build-id .note.gnu.property .note.ABI-tag .rel.plt 
   01     .init .plt .text __libc_freeres_fn .fini 
   02     .rodata .eh_frame .gcc_except_table 
   03     .tdata .init_array .fini_array .data.rel.ro .got .got.plt .data __libc_subfreeres __libc_IO_vtables __libc_atexit .bss __libc_freeres_ptrs 
   04     .note.gnu.build-id .note.gnu.property .note.ABI-tag 
   05     .tdata .tbss 
   06     .note.gnu.property 
   07     
   08     .tdata .init_array .fini_array .data.rel.ro .got
```

https://github.com/msvisser/ELF-Loader/blob/master/elfloader.c - an elf loader using c

https://ocw.cs.pub.ro/courses/cns/labs/lab-02 - how to make use of readelf and some more about them


https://github.com/gimli-rs/object/blob/81767fde32a70c3d9987085e8fc11bed65605857/crates/examples/src/readobj/elf.rs - object crate utilization for ELFs


https://stackoverflow.com/questions/77372088/trying-to-implement-a-simple-loader-for-32-bit-elf-files-which-do-not-contain-an - simple 32bit elf loader (not all sections)


p_type (ProgramHeader32 Type) = 0x6474e552 => https://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/progheader.html


how tf does p_offset and p_vaddr differ and what do they specifically mean? => https://stackoverflow.com/questions/52533193/how-to-understand-the-difference-between-offset-and-viraddr-in-program-headers-i