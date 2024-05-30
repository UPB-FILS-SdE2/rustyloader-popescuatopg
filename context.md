For testing the main.c file
1. I created a main.c file
1. I compiled that file with the `-m32` (for x32 arch) and `-static` (for a static elf) executable
1. I loaded the ELF, got the e_entry (entry point) of the ELF, the offset, vaddr and flags of each LOAD segments
1. I mmap'ed memory for each LOAD segment calculating the correct vaddr and offset (so rounded down to the nearest PAGE_SIZE for each)
WE ARE HERE
|
V
1. I copied memory for each LOAD segment section in the mapped pages
-