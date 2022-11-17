/* 
    copyright
*/
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
fn main() {
   //env::set_var("RUST_BACKTRACE","1"); //Set backtrace for debug purposes. Enable this when needed for debugging errors
    //FIXME: implement a commandline parameter or some other user level way to decide which layer processing to run  
    //process_layer1();
    //process_layer2();
    process_layer3();
}

//layer 1
fn process_layer1(){
   //env::set_var("RUST_BACKTRACE","1"); //Set backtrace for debug purposes. Enable this when needed for debugging errors
   let filename = String::from("layer1.txt");
   let file_contents = read_layer_file(filename);
   let decoded = match file_contents.ok(){
    Some(p) => decode_ascii85(p ),
    None => panic!("Error with file_contents data after reading from file before decoding"),
   };
   println!("{}",decoded);

}

//layer2
fn process_layer2(){
   let filename = String::from("layer2.txt");
   let file_contents = read_layer_file(filename);
   let mut decoded = match file_contents.ok(){
    Some(p) => _decode_ascii85_bytes(p ),
    None => panic!("Error with file_contents data after reading from file before decoding"),
   };
   let flipped_and_rotated = flip_and_rotated(&mut decoded);
   println!("{}",flipped_and_rotated);
}

fn process_layer3(){
   //env::set_var("RUST_BACKTRACE","1"); //Set backtrace for debug purposes. Enable this when needed for debugging errors
   let filename = String::from("layer3.txt");
   let file_contents = read_layer_file(filename);
   let parity = match file_contents.ok(){
    Some(p) => remove_parity_bits(p ),
    None => panic!("Error with file_contents data after reading from file before decoding"),
   };
   //let after_parity = remove_parity_bits(&mut decoded);
   let decoded = _decode_ascii85_bytes(parity);
   let push_str = match String::from_utf8(decoded.to_vec()){
                Err(why) => panic!("Issue appending byte vector to string {} : ", why),
                Ok(str) => str
   };
   println!("{}",push_str);
}

fn remove_parity_bits(_input : Vec<u8>) -> Vec<u8>{
   let mut input = _input.clone(); 
    for byte in input.iter_mut(){
    //iterate and mask for each bit to see what the parity should be
    let mut num_masked = 0;
    for i in 0..8{
       let masked = (*byte & (1<<i) >> i); //extract bit by shifting one, masking, and shifting back to lsb to get a 1 or 0 answer
        num_masked += masked;
    }
    if num_masked %2 != 0{
        println!("detected error in parity bit");
        *byte = 32;
        continue
    } 
    *byte = *byte >> 1; //read data without parity bit
   }  
   input
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

fn flip_and_rotated(input : &mut Vec<u8>)-> String{
    //println!("pre: '{:?}'",input); 
    for byte in  input.iter_mut(){
        *byte = *byte ^ 0x55;
        //need to shift in the previous bytes discarded bit
        *byte = (*byte >> 1) | ((*byte & 1) << 7);
         
    }
   let push_str = match String::from_utf8(input.to_vec()){
                Err(why) => panic!("Issue appending byte vector to string {} : ", why),
                Ok(str) => str
   };

   push_str
}

fn _decode_ascii85_bytes(input :Vec<u8>) -> Vec<u8>{

    //println!("decoding : {} | {}",input[input.len()-1] as char, input.len()); //fuck you rustc borrow checker 
    /* First we need to take 5 chars and multiply by n[i] * 85 ^i  */
    let mut ret_val : Vec<u8> = Vec::new();
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
            
            //for i in 0..4{
            //    //The output should be only in human legible ascii. All control characters and extended ascii results should be considered padding
            //    if bytes[i] > 126 {
            //        bytes[i] = 33;//use ! character as a padding indicator. FIXME: Lets use a better padding indicator
            //    }
            //}
            
            ret_val.append(&mut bytes.to_vec());
            
            byte_counter = 4;
            val = 0;
        }
    }
        ret_val
}

#[warn(dead_code)]
fn decode_ascii85(input : Vec<u8>) -> String{
   let bytes = _decode_ascii85_bytes(input);
    let push_str = match String::from_utf8(bytes){
        Err(why) => panic!("Issue appending byte vector to string {} : ", why),
        Ok(str) => str
    };
    push_str
}