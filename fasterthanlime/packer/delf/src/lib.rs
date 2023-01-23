use derive_more::*;
use derive_try_from_primitive::TryFromPrimitive;
use enumflags2::*;
use nom::{combinator::verify, number::complete::le_u32};

mod parse;

pub struct ProgramHeader {
    pub r#type: SegmentType,
    pub flags: BitFlags<SegmentFlag>,
    pub offset: Addr,
    pub vaddr: Addr,
    pub paddr: Addr,
    pub filesz: Addr,
    pub memsz: Addr,
    pub align: Addr,
    pub data: Vec<u8>,
}

impl ProgramHeader {
    fn parse<'a>(full_input: parse::Input<'_>, i: parse::Input<'a>) -> parse::Result<'a, Self> {
        use nom::sequence::tuple;
        let (i, (r#type, flags)) = tuple((SegmentType::parse, SegmentFlag::parse))(i)?;

        let ap = Addr::parse;
        let (i, (offset, vaddr, paddr, filesz, memsz, align)) = tuple((ap, ap, ap, ap, ap, ap))(i)?;

        let res = Self {
            r#type,
            flags,
            offset,
            vaddr,
            paddr,
            filesz,
            memsz,
            align,
            // `to_vec()` turns a slice into an owned Vec (this works because u8 is Clone+Copy)
            data: full_input[offset.into()..][..filesz.into()].to_vec(),
        };
        Ok((i, res))
    }

    /**
     * File range where segment is stored
     */
    fn file_range(&self) -> std::ops::Range<Addr> {
        self.offset..self.offset + self.filesz
    }

    /**
     * Mem range where segment is mapped
     */
    pub fn mem_range(&self) -> std::ops::Range<Addr> {
        self.vaddr..self.vaddr + self.memsz
    }
}

impl fmt::Debug for ProgramHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "file {:?} | mem {:?} | align {:?} | {} {:?}",
            self.file_range(),
            self.mem_range(),
            self.align,
            // Default Debug fmt for `enumflags2` is quite verbose, we print `RWX` instead.
            &[
                (SegmentFlag::Read, "R"),
                (SegmentFlag::Write, "W"),
                (SegmentFlag::Execute, "X"),
            ]
            .iter()
            .map(|&(flag, letter)| {
                if self.flags.contains(flag) {
                    letter
                } else {
                    "."
                }
            })
            .collect::<Vec<_>>()
            .join(""),
            self.r#type,
        )
    }
}

#[derive(Debug)]
pub struct File {
    pub r#type: Type,
    pub machine: Machine,
    pub entry_point: Addr,
    pub program_headers: Vec<ProgramHeader>,
}

impl File {
    pub fn parse_or_print_error(i: parse::Input) -> Option<Self> {
        match Self::parse(i) {
            Ok((_, file)) => Some(file),
            Err(nom::Err::Failure(err)) | Err(nom::Err::Error(err)) => {
                eprintln!("Parsing failed:");
                for (input, err) in err.errors {
                    use nom::Offset;
                    let offset = i.offset(input);
                    eprintln!("{:?} at position {}:", err, offset);
                    eprintln!("{:>08x}: {:?}", offset, HexDump(input));
                }
                None
            }
            Err(_) => panic!("unexpected nom error"),
        }
    }

    // 0x7F' 'E' 'L' 'F' at the very start:
    const MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];

    fn parse(i: parse::Input) -> parse::Result<Self> {
        let full_input = i;

        use nom::{
            bytes::complete::{tag, take},
            error::context,
            sequence::tuple,
        };

        let (i, _) = tuple((
            context("Magic", tag(Self::MAGIC)),
            // the 64-bit class:
            context("Class", tag(&[0x2])),
            // endianness - 1 is little, 2 big:
            context("Endianness", tag(&[0x1])),
            context("Version", tag(&[0x1])),
            // field unused since linux 2.6:
            context("OS ABI", nom::branch::alt((tag(&[0x0]), tag(&[0x3])))),
            context("Padding", take(8_usize)),
        ))(i)?;

        let (i, (r#type, machine)) = tuple((Type::parse, Machine::parse))(i)?;
        // This 32-bit integer should always be set to 1 in the current version
        // of ELF (see diagram). We don't *have* to check it, but as it's so
        // easy, let's anyway.
        let (i, _) = context("Version (bis)", verify(le_u32, |&x| x == 1))(i)?;
        let (i, entry_point) = Addr::parse(i)?;

        use nom::{combinator::map, number::complete::le_u16};
        // Some values are stored as u16, but as they are offsets or counts,
        // we want `usize` in Rust.
        let u16_usize = map(le_u16, |x| x as usize);

        // ph = program hdr, sh = section hdr
        let (i, (ph_offset, _sh_offset)) = tuple((Addr::parse, Addr::parse))(i)?;
        let (i, (_flags, _hdr_size)) = tuple((le_u32, le_u16))(i)?;
        let (i, (ph_entsize, ph_count)) = tuple((&u16_usize, &u16_usize))(i)?;
        let (i, (_sh_entsize, _sh_count, _sh_nidx)) =
            tuple((&u16_usize, &u16_usize, &u16_usize))(i)?;

        // `chunks()` divides a slice into chunks of equal size - perfect, as we know the entry size.
        let ph_slices = (&full_input[ph_offset.into()..]).chunks(ph_entsize);
        let mut program_headers = Vec::new();
        for ph_slice in ph_slices.take(ph_count) {
            let (_, ph) = ProgramHeader::parse(full_input, ph_slice)?;
            program_headers.push(ph);
        }

        let res = Self {
            machine,
            r#type,
            entry_point,
            program_headers,
        };
        Ok((i, res))
    }
}

// A helper to write dumps in case of parse errors
pub struct HexDump<'a>(&'a [u8]);

use std::fmt;
impl<'a> fmt::Debug for HexDump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &x in self.0.iter().take(32) {
            write!(f, "{:02x} ", x)?;
        }
        Ok(())
    }
}

// ELF type at offset 16
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Type {
    None = 0x0,
    Rel = 0x1,
    Exec = 0x2,
    Dyn = 0x3,
    Core = 0x4,
}

impl_parse_for_enum!(Type, le_u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum Machine {
    X86 = 0x03,
    Arm = 0x28,
    X86_64 = 0x3e,
    AArch64 = 0xb7,
}
// So cool, instead of implementing from_u16 we take ^^ try_from via crate :D
impl_parse_for_enum!(Machine, le_u16);

// Segment type in ELF64 program  header
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum SegmentType {
    Null = 0x0,
    Load = 0x1,
    Dynamic = 0x2,
    Interp = 0x3,
    Note = 0x4,

    // -- Hunted down by Amos --
    ShLib = 0x5,
    PHdr = 0x6,
    TLS = 0x7,
    LoOS = 0x6000_0000,
    HiOS = 0x6FFF_FFFF,
    LoProc = 0x7000_0000,
    HiProc = 0x7FFF_FFFF,
    GnuEhFrame = 0x6474_E550,
    GnuStack = 0x6474_E551,
    GnuRelRo = 0x6474_E552,
    GnuProperty = 0x6474_E553,
    // ------------------------
}

impl_parse_for_enum!(SegmentType, le_u32);

// Segment flag in ELF64 program  header
#[derive(Debug, Clone, Copy, PartialEq, Eq, BitFlags)]
#[repr(u32)]
pub enum SegmentFlag {
    Execute = 0x1,
    Write = 0x2,
    Read = 0x4,
}

impl_parse_for_enumflags!(SegmentFlag, le_u32);

// "Add" and "Sub" are in `derive_more`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
pub struct Addr(pub u64);

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08x}", self.0)
    }
}

// This will come in handy when serializing.
impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

// This will come in handy when indexing/ sub-slicing.
impl Into<usize> for Addr {
    fn into(self) -> usize {
        self.0 as usize
    }
}

// This will come in handy when parsing.
impl From<u64> for Addr {
    fn from(x: u64) -> Self {
        Self(x)
    }
}

impl Addr {
    fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{combinator::map, number::complete::le_u64};
        map(le_u64, From::from)(i)
    }
}

#[cfg(test)]
mod tests {
    use enumflags2::BitFlags;

    #[test]
    fn type_to_u16() {
        use super::{Machine, Type};

        assert_eq!(Machine::X86_64 as u16, 0x3E);

        assert_eq!(Type::Dyn as u16, 0x3);
    }

    #[test]
    fn type_try_from() {
        use super::{Machine, Type};

        assert_eq!(Machine::try_from(0x3E), Ok(Machine::X86_64));
        assert_eq!(Machine::try_from(0xFA), Err(0xFA));

        assert_eq!(Type::try_from(0x2), Ok(Type::Exec));
        assert_eq!(Type::try_from(0xF00D), Err(0xF00D));
    }

    #[test]
    fn try_bitflag() {
        use super::SegmentFlag;

        // This is a value we could've read straight from an ELF file.
        let flags_integer = 6u32;
        let flags = BitFlags::<SegmentFlag>::from_bits(flags_integer).unwrap();
        assert_eq!(flags, SegmentFlag::Read | SegmentFlag::Write);
        assert_eq!(flags.bits(), flags_integer);

        // this does not correspond to any flags
        assert!(BitFlags::<SegmentFlag>::from_bits(1992).is_err());
    }
}
