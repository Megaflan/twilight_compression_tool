mod compression;

use std::fs::File;
use std::io::Write;
use std::{env, fs, io};
use std::path::Path;

use compression::{decompress, compress, read_file};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <mode> <file>", args[0]);
        eprintln!("Modes:");
        eprintln!("-d: Decompress the file in tiny blobs (Default).");
        eprintln!("-c: Compress the file.");        
        std::process::exit(1);
    }

    let path = Path::new(&args[2]);
    let mode = &args[1];

    match mode.as_str() {
        "-d" => {
            let stream = read_file(&path)?;
            let decomp = decompress(stream);

            let filename_no_ext = Path::new(&path).file_stem().unwrap().to_str().unwrap();
            let dir_name = format!("{}_dec", filename_no_ext);
    
            let dir_path = Path::new(&dir_name);
            fs::create_dir_all(&dir_path).expect("Failed to create directory");            

            let mut i = 0;
            for entry in decomp {  
                let output_path = dir_path.join(format!("{}_dec.bin.{:04}", filename_no_ext, i));
                let mut output_file = File::create(&output_path).expect("Failed to create output file");

                output_file.write_all(&entry).expect("Failed to write to output file");
                i += 1;
            }
        }
        "-c" => {
            let stream = read_file(&path)?;
            let comp = compress(stream);
            
            let filename_no_ext = Path::new(&path).file_stem().unwrap().to_str().unwrap();
            let output_path = format!("{}_cmp.bin", filename_no_ext);
            
            let mut output_file = File::create(output_path).expect("Failed to create output file");
            output_file.write_all(&comp).expect("Failed to write to output file");
        }
        _ => {
            eprintln!("Unknown mode: {}", mode);
        }
    }
    Ok(())
}
