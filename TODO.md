## Suggestions
  - Before starting the assignment, we recommend that you familiarize yourself with specific notions and concepts, such as:
  - address space
  - page access rights
  - executable file format
  - on-demand pagination
  - page default
  - file mapping in address space - file mapping
  - Follow the resources described in the Support resources section.

## Executable file loader
  Implement an ELF executable file loader for Linux. The loader will load the executable file into memory page by page, using an on-demand paging mechanism - a page will be loaded only when it is needed. For simplicity's sake, the loader will only execute static executables - which are not linked to shared/dynamic libraries.

### To execute an executable file, the loader performs the following steps:

  - It initializes its internal structures.
  - It displays on stderr (2) the list of segments in the executable file.
  - Display on stderr (2) the base address of the executable file (minimum memory address for loading segments).
  - Display on stderr (2) the address of the entry point
  - Analyzes the binary file - the object library is available for this purpose.
  - Executes the first instruction of the executable (the entry point).
    - Throughout execution, a page fault will be generated for each access to an unmatched page in memory;
  - It will detect each access to an unmatched page and check to which segment of the executable it belongs.
    - if it is not found in a segment, it is an invalid memory access - the program exits with error -200 ;
    - if the page fault is generated in an already mapped page, an unauthorized memory access is attempted (this segment does not have the required permissions) - again, the program exits with a -200 error;
    - if the page is in a segment and has not yet been mapped, it is mapped to the corresponding address, with the permissions of that segment;
  - Use mmap functions (rust variant).
  - The page must be fixedly mapped to the address specified in the segment.

## Recommendations for implementation
  - The page fault handler is implemented via a SIGSEGV signal handling routine.
  - To implement on-demand paging logic, you need to intercept page faults produced when an invalid access to a memory area occurs. When intercepting page faults, they should be handled appropriately, depending on the segment to which they belong:
    - if not in a known segment, run the default handler;
    - if it's in an unmapped page, map it to memory, then copy the segment data to the ;
    - if it's in an already mapped page, run the default handler (since this is unauthorized access to memory);
  - Pages from two different segments cannot overlap.
  - The size of a segment is not aligned at page level; memory that is not part of a segment must not be manipulated in any way - the behavior of an access in this area is not defined.

## Details
  - To manage virtual memory, use the mmap, munmap and mprotect functions.
  - To intercept an invalid access to a memory area, you need to intercept the SIGSEGV signal using calls from the sigaction family.
  - You need to register a handler in the sa_sigaction field of the sigaction structure.
  - To determine the address that generated the page fault, use the si_addr field of the siginfo_t structure.
  - When accessing a new page in a segment, map the page where the page fault occurred using MAP_FIXED, then copy the data from the executable to page