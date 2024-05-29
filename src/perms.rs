use nix::sys::mman::ProtFlags;

pub struct SegmentPerms {
    pub read: bool,
    pub write: bool,
    pub exec: bool,
}

impl SegmentPerms {
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            if self.read { "r" } else { "-" },
            if self.write { "w" } else { "-" },
            if self.exec { "x" } else { "-" }
        )
    }

    pub fn from_number(num: usize) -> SegmentPerms {
        SegmentPerms {
            read: num & 0x04 == 0x04,
            write: num & 0x02 == 0x02,
            exec: num & 0x01 == 0x01,
        }
    }

    pub fn to_flags(&self) -> ProtFlags {
      let mut flags = ProtFlags::empty();

      if self.read {flags = flags | ProtFlags::PROT_READ;}
      if self.write {flags = flags | ProtFlags::PROT_WRITE;}
      if self.exec {flags = flags | ProtFlags::PROT_EXEC;}

      flags
    }
}
