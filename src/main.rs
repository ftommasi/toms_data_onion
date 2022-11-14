/* 
    copyright
*/
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
fn main() {
   //env::set_var("RUST_BACKTRACE","1"); //Set backtrace for debug purposes. Enable this when needed for debugging errors
   let filename = String::from("layer1.txt");
   let file_contents = read_layer_file(filename);
   let decoded = match file_contents.ok(){
    Some(p) => decode_ascii85(p ),
    None => panic!("Error with file_contents data after reading from file before decoding"),
   };
   println!("{}",decoded);
}

fn read_layer_file(filename : String) -> io::Result<Vec<u8>>{
    let path = Path::new(&filename);
    let display = path.display();
    let mut f = match File::open(&path){
        Err(why) => panic!("Could not read {} : {}", display,why),
        Ok(file) => file,
    };

    let mut buffer : Vec<u8> = Vec::new();

    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}


fn decode_ascii85(input : Vec<u8>) -> String{
    //println!("decoding : {} | {}",input[input.len()-1] as char, input.len()); //fuck you rustc borrow checker 
    /* First we need to take 5 chars and multiply by n[i] * 85 ^i  */
    let mut ret_val = String::new();
    let mut val : u64 = 0;
    let mut byte_counter : i8 = 4;
    for byte in input{
        if byte < 33{
            //need to account for hidden ASCII control characters that should be ignored
            continue;
        } 
        let minus33 = (byte as u32) - (33 as u32);
        let next_val = minus33 * u32::from(85 as u32).pow(byte_counter as u32);
        val += next_val as u64 ; 

        byte_counter -= 1;
        
        //process 5 bytes at a time
        if byte_counter < 0{
            //5 ascii85 bytes decode to 4 ASCII bytes
            let mut bytes : [u8;4] = [0;4];
            bytes[0] = ((val & 0xFF000000) >> 24) as u8;
            bytes[1] = ((val & 0x00FF0000) >> 16) as u8;
            bytes[2] = ((val & 0x0000FF00) >> 8 ) as u8;
            bytes[3] =  (val & 0x000000FF) as u8;
            
            for i in 0..4{
                //The output should be only in human legible ascii. All control characters and extended ascii results should be considered padding
                if bytes[i] > 126 {
                    bytes[i] = 33;//use ! character as a padding indicator. FIXME: Lets use a better padding indicator
                }
            }
            
            //from utf8 bytes to string
            let push_str = match String::from_utf8(bytes.to_vec()){
                Err(why) => panic!("Issue appending byte vector to string {} : {:?}", why, bytes),
                Ok(str) => str
            };
            ret_val.push_str(&push_str);
            
            byte_counter = 4;
            val = 0;
        }
    }
    ret_val
}