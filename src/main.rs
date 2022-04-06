use std::fs::File;
use std::io::Read;
use std::time::Duration;

mod constants;
mod scheduler;
mod sfc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <rom_file>", &args[0]);
    }

    let rom_path = &args[1];
    let mut file = File::open(rom_path).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    let mut s = sfc::SuperFamicom::new(&buf);

    loop {
        println!("Frame: {}", s.frame_count());
        s.run_frame();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}

#[test]
fn unimplemented() {
    let mut file = File::open("rom/HelloWorld.sfc").unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    let s = sfc::SuperFamicom::new(&buf);
    let unimplemented = s.unimplemented_opcode();
    for i in 0..256 {
        if i > 0 && i % 16 == 0 {
            println!();
        }
        if unimplemented[i] {
            print!("{:#04x}, ", i);
        }
    }
    println!();
}
