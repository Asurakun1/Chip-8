use std::{env, fs::File, io::Read};

pub fn load_rom() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];

        println!("Attempting to load ROM: {}", filename);
        let mut file = File::open(filename).expect("Failed to load ROM file");
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .expect("Failed to read ROM file");

        return buffer;
    }
    include_bytes!("IBM Logo.ch8").to_vec()
}
