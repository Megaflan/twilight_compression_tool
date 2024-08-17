use std::{fs::File, io::{self, Read}};

pub fn read_file(file_path: &std::path::Path) -> io::Result<Vec<u8>> {
    let mut stream: Vec<u8> = Vec::new();
    File::open(&file_path)?.read_to_end(&mut stream)?;
    Ok(stream)
}

pub fn decompress(stream: Vec<u8>) -> Vec<Vec<u8>> {
    let mut pos = 0;
    let mut entries: Vec<Vec<u8>> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();

    while pos < stream.len() {
        let value = stream[pos];        

        match value {
            //Direct Copy
            0x00..=0x7F => {
                let length = value as usize + 1;
                if pos + length <= stream.len() {
                    //We copy as many bytes as length says.
                    buffer.extend_from_slice(&stream[pos..pos + length]);
                    pos += length;
                }
            }
            //Byte Fill
            0x80..=0xBF => {      
                let length = (value as usize) - 0x7d;
                if pos < stream.len() {
                    //Read the byte, fill until length with said byte.
                    buffer.extend(vec![stream[pos]; length]);
                    pos += 1;
                }
            } 
            //Referential Copy
            0xC0..=0xDF => {  
                let length = (value as usize) - 0xbc;
                if pos + 2 < stream.len() {
                    let val1 = stream[pos];
                    // Calculate the offset using the next byte in the stream
                    let offset = ((val1 as usize) << 8 | stream[pos + 1] as usize) as usize;
                    pos += 2;
                    let out_len = buffer.len();
                    
                    if out_len >= offset {
                        // Copy the specified length of bytes from the buffer using the calculated offset
                        for i in 0..length {
                            buffer.push(buffer[out_len - offset + i]);
                        }
                    }
                }             
            }
            //Delta Encoding
            0xE0..=0xEF => {
                let length = (value as usize) - 0xdc;
                if pos + 2 < stream.len() {
                    // Read the increment value (val1) and the initial value (val2)
                    let val1 = stream[pos];
                    let mut val2 = stream[pos + 1];
                    pos += 2;

                    for _i in 0..length {
                        buffer.push(val2);                        
                        // Update val2 for the next iteration, wrapping_add is used to handle overflow, wrapping around to 0 if it exceeds 255
                        val2 = val2.wrapping_add(val1);
                    }
                }
            }
            0xFF => {
                entries.push(buffer.clone());
                buffer.clear();
            }
            _ => {}
        }
        pos += 1;
    }

    entries    
}

pub fn compress(stream: Vec<u8>) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut i = 0;

    /*while i < stream.len() {
        
    }*/

    compressed
}
