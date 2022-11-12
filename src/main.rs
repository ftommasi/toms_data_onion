use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::string::{FromUtf16Error, FromUtf8Error};

fn main() {
    let filename = String::from("layer1.txt");
    let file_contents = read_layer_file(filename);
   // println!("{:?}",file_contents.ok());
   let decoded = match file_contents.ok(){
    Some(p) => decode_ascii85(p ),
    None => panic!("Error with file_contents data after reading from file before decoding"),
   };
   println!("Decoded is '{}'",decoded);
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
    println!("decoding : {}",input[0]); //fuck you rustc borrow checker 
    //let a85_alphabet = String::from("9jqo^BlbD-BleB1DJ+*+F(f,q/0JhKF<GL>Cj@.4Gp$d7F!,L7@<6@)/0JDEF<G%<+EV:2F!,O<DJ+*.@<*K0@<6L(Df-\\\0Ec5e;DffZ(EZee.Bl.9pF\"AGXBPCsi+DGm>@3BB/F*&OCAfu2/AKYi(DIb:@FD,*)+C]U=@3BN#EcYf8ATD3s@q?d$AftVqCh[NqF<G:8+EV:.+Cf>-FD5W8ARlolDIal(DId<j@<?3r@:F%a+D58'ATD4$Bl@l3De:,-DJs`8ARoFb/0JMK@qB4^F!,R<AKZ&-DfTqBG%G>uD.RTpAKYo'+CT/5+Cei#DII?(E,9)oF*2M7/c");
    /* First we need to take 5 chars and multiply by n[i] * 85 ^i  */
    let mut ret_val = String::new();
    let mut val : u32 = 0;
    let mut byte_counter : i8 = 4;
    for byte in input{
        let plus33 = (byte as u32) - (33 as u32);
        let next_val = plus33 * u32::from(85 as u32).pow(byte_counter as u32);
        println!("next val is {} : {} ({})", byte as char, plus33, next_val);
        val += next_val; 

        byte_counter -= 1;
        if byte_counter < 0{
            println!("val is : {}", val);
            let mut bytes : [u8;4] = [0;4];
            bytes[0] = (val & 0xFF000000) as u8;
            bytes[1] = (val & 0x00FF0000) as u8;
            bytes[2] = (val & 0x0000FF00) as u8;
            bytes[3] = (val & 0x000000FF) as u8;
           
            let push_str = match String::from_utf8(bytes.to_vec()){
                Err(why) => panic!("Issue appending byte vector to string {} : {:?}", why, bytes),
                Ok(str) => str
            }; 
            ret_val.push_str(&push_str);
            
            byte_counter = 4;
            val = 0
        }
    }
    String::from("decode_ascii85 not implemented")
}