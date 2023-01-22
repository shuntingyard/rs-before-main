use std::{env, error::Error, fs};

use mmap::{MapOption, MemoryMap};
use region::{protect, Protection};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("usage: elk FILE");
    let input = fs::read(&input_path)?;

    println!("Analyzing: {:?}", input_path);

    let file = match delf::File::parse_or_print_error(&input[..]) {
        Some(f) => f,
        None => std::process::exit(1),
    };

    println!("{:#?}", file);

    println!("Disassembling: {:?}", input_path);

    let code_ph = file
        .program_headers
        .iter()
        .find(|ph| ph.mem_range().contains(&file.entry_point))
        .expect("segment with entry point not found");

    ndisasm(&code_ph.data[..], file.entry_point)?;

    println!("Mapping {:?} in memory...", input_path);

    // We'll need to hold onto our "mmap::MemoryMap", because dropping them
    // unmaps them!
    let mut mappings = Vec::new();

    // We're only interested in "Load" segments.
    for ph in file
        .program_headers
        .iter()
        .filter(|ph| ph.r#type == delf::SegmentType::Load)
    {
        println!("Mapping segment @ {:?} with {:?}", ph.mem_range(), ph.flags);
        // Note: mmap-ing would fail without segmeents being aligned on pages!
        let mem_range = ph.mem_range();
        let len: usize = (mem_range.end - mem_range.start).into();

        // Note: `as` is the "cast" operator, and `_` is a placeholder to force rustc
        // to infer the type based on other hints (here, the left-hand-side declaration).
        let addr: *mut u8 = mem_range.start.0 as _;

        // At first, we want the memory area to be writable, so we can copy to it.
        // Permissions come later.
        let map = MemoryMap::new(len, &[MapOption::MapWritable, MapOption::MapAddr(addr)])?;

        println!("Copying segment data...");
        {
            let dst = unsafe { std::slice::from_raw_parts_mut(addr, ph.data.len()) };
            dst.copy_from_slice(&ph.data[..]);
        }

        println!("Adjusting permissions...");
        // The `region` crate and our `delf` crate have two different enums (and bit flags)
        // for protection, so we need to map from delf's to region's.
        let mut protection = Protection::NONE;
        for flag in ph.flags.iter() {
            protection |= match flag {
                delf::SegmentFlag::Read => Protection::READ,
                delf::SegmentFlag::Write => Protection::WRITE,
                delf::SegmentFlag::Execute => Protection::EXECUTE,
            }
        }
        unsafe {
            protect(addr, len, protection)?;
        }
        mappings.push(map);
    }

    let code = &code_ph.data;
    unsafe {
        protect(code.as_ptr(), code.len(), Protection::READ_WRITE_EXECUTE)?;
    }

    println!("Jumping to entry point @ {:?}", file.entry_point);
    println!("Press enter to jmp...");
    {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s)?;
    }

    unsafe {
        // Note that we don't have to do pointer arithmetic here,
        // as the entry point is indeed mapped in memory at the right place.
        jmp(file.entry_point.0 as _);
    }

    Ok(())
}

fn ndisasm(code: &[u8], origin: delf::Addr) -> Result<(), Box<dyn Error>> {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    let mut child = Command::new("ndisasm")
        .arg("-b")
        .arg("64")
        .arg("-o")
        .arg(format!("{}", origin.0))
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child.stdin.as_mut().unwrap().write_all(code)?;
    let output = child.wait_with_output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}

unsafe fn jmp(addr: *const u8) {
    let fn_ptr: fn() = std::mem::transmute(addr);
    fn_ptr();
}
