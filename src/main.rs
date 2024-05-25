use nix::libc::siginfo_t;
use nix::sys::mman::{mmap, mprotect, munmap, MapFlags, ProtFlags};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use nix::unistd::SysconfVar::PAGE_SIZE;
use nix::unistd::{getpid, sysconf};
use std::io::Read;
use std::os::fd::AsRawFd;
use std::os::raw::{c_int, c_void};
use std::path::Path;

use object::read::elf::{FileHeader, Sym};
use object::{elf, Endian, Endianness, LittleEndian};
use std::error::Error;
use std::fs;

use object::elf::*;
use object::read::elf::*;
use object::read::{SectionIndex, StringTable, SymbolIndex};

mod runner;

extern "C" fn sigsegv_handler(_signal: c_int, siginfo: *mut siginfo_t, _extra: *mut c_void) {
    let address = unsafe { (*siginfo).si_addr() } as usize;
    // map pages

    println!("Page fault at address: 0x{:x}", address);

    std::process::exit(0);
}

// fn parse_elf<'a>(filename: &str) -> Result<(u64, Vec<object::Segment<'a, '_>>), Box<dyn Error>> {
//     let path = Path::new(filename);
//     let mut file = File::open(path)?;
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer)?;

//     let object_file = object::File::parse(&*buffer)?;
//     let entry_point = object_file.entry();
//     let segments: Vec<Segment<&[u8]>> = object_file.segments()
//     .map(|seg| {
//         let data = &buffer[seg.file_range().start as usize..seg.file_range().end as usize];
//         // Segment(seg.address(), data)
//     })
//     .collect();
//     Ok((entry_point, segments))
// }

// fn print_segments(segments: &[object::Segment<'_, '_>]) {
//     eprintln!("Segments");
//     eprintln!("#\taddress\t\tsize\toffset\tlength\tflags");
//     for (i, segment) in segments.iter().enumerate() {
//         let address = segment.address();
//         let size = segment.size();
//         let (offset, length) = segment.file_range();
//         let flags = segment.flags();
//         eprintln!(
//             "{}\t0x{:x}\t{}\t0x{:x}\t{}\t{:?}",
//             i, address, size, offset, length, flags
//         );
//     }
// }

fn segment_to_string(segment: &ProgramHeader32<Endianness>) -> String {
    format!(
        "ProgramHeader32 {}
  p_type: 0x{:x},
  p_offset: 0x{:x},
  p_vaddr: 0x{:x},
  p_paddr: 0x{:x},
  p_filesz: 0x{:x},
  p_memsz: 0x{:x},
  p_flags: 0x{:x},
  p_align: 0x{:x},
{}",
        "{",
        segment.p_type.get(object::Endianness::Little),
        segment.p_offset.get(object::Endianness::Little),
        segment.p_vaddr.get(object::Endianness::Little),
        segment.p_paddr.get(object::Endianness::Little),
        segment.p_filesz.get(object::Endianness::Little),
        segment.p_memsz.get(object::Endianness::Little),
        segment.p_flags.get(object::Endianness::Little),
        segment.p_align.get(object::Endianness::Little),
        "}"
    )
}

fn exec(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(filename)?;
    let fd = file.as_raw_fd();
    let data = fs::read(filename)?;
    let elf = elf::FileHeader32::<object::Endianness>::parse(&*data)?;
    let endian = elf.endian()?;
    // println!("elf: {:#?}", elf);

    let virtual_address_entry = elf.e_entry;

    let entry_point = virtual_address_entry.get(object::Endianness::Little) as usize;
    println!("{:#x}", entry_point);
    let program_headers = elf.program_headers(endian, data.as_slice()).unwrap();

    let mut load_segments: Vec<ProgramHeader32<Endianness>> = vec![];

    for segment in program_headers {
        if segment.p_type.get(object::Endianness::Little) == 1
        /* is LOAD */
        {
            load_segments.push(segment.clone());
            println!("{}", segment_to_string(segment));
        }
    }

    let mut allocated_memory_ptrs: Vec<*mut c_void> = vec![];
    let mut base_address: usize = usize::MAX;

    let page_size = sysconf(PAGE_SIZE).unwrap().unwrap();
    for load_segment in load_segments {
        let p_offset = load_segment.p_offset(object::Endianness::Little) as *mut c_void;
        let p_vaddr = load_segment.p_vaddr(object::Endianness::Little) as usize;
        let p_memsz = load_segment.p_memsz(object::Endianness::Little) as usize;
        let p_flags = load_segment.p_flags(object::Endianness::Little);


        let actual_addr = p_vaddr - (p_vaddr % page_size as usize);
        let actual_offset = p_offset as usize - (p_offset as usize % page_size as usize);

        println!(
            "mmap at offset {:#?} for vaddr 0x{:x} with memsz 0x{:x} - unused: flags: 0x{:x}",
            p_offset, p_vaddr, p_memsz, p_flags
        );
        let p = unsafe {
            mmap(
                // ((p_offset as usize) + p_vaddr) as *mut c_void,
                actual_addr as *mut c_void,
                // pagesize as usize,
                p_memsz,
                ProtFlags::PROT_NONE ,
                // if p_flags & 0x01 == 0x01 {
                //   ProtFlags::PROT_EXEC
                // } else {
                //   ProtFlags::empty()
                // } 
                
                // |
                
                // if p_flags & 0x02 == 0x02 {
                //   ProtFlags::PROT_WRITE
                // } else {
                //   ProtFlags::empty()
                // } 
                
                // |
                
                // if p_flags & 0x04 == 0x04 {
                //   ProtFlags::PROT_READ
                // } else {
                //   ProtFlags::empty()
                // } ,
                MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS | MapFlags::MAP_FIXED,
                -1,//fd,
                actual_offset as i32//p_offset as i32,
            )
        };

        if p.is_err() {
          println!("{}", p.err().unwrap());
          std::process::exit(-1);
        }
        let p = p.unwrap();

        println!("{:#?}", p);

        let vaddr = load_segment.p_vaddr.get(object::Endianness::Little) as usize;
        if vaddr < base_address {
            base_address = vaddr;
        }

        allocated_memory_ptrs.push(p);
    }

    // // determine entry point
    eprintln!("Entry point 0x{:x}", entry_point);
    eprintln!("Base address 0x{:x}", base_address);

    // // register SIGSEGV handler
    let handler = SigHandler::SigAction(sigsegv_handler);
    let action = SigAction::new(handler, SaFlags::SA_SIGINFO, SigSet::empty());
    unsafe { sigaction(Signal::SIGSEGV, &action) }.expect("Failed to set signal handler");

    // // run ELF using runner::exec_run
    // runner::exec_run(base_address as usize, entry_point as usize);

    Ok(runner::exec_run(base_address, entry_point))
    // Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // load ELF provided within the first argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <executable>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    exec(filename).unwrap();
    Ok(())
}
