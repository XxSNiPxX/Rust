extern crate regex;
extern crate hex;
extern crate serde_json;
extern crate memmem;
#[macro_use]
extern crate enum_map;
extern crate csv;


#[macro_use]

extern crate arrayref;
use std::{thread, time};



use memmem::{Searcher, TwoWaySearcher};
use std::io::Cursor;
// use byteorder::{BigEndian, ReadBytesExt};
use std::collections::HashMap;
use enum_map::EnumMap;

use serde_json::{Map, Value};

use serde::Serialize;
use serde::Deserialize;
use simple_user_input::get_input;
use std::{
    fs,i32,i16,u32,f32,i8,

    path::{Path, PathBuf},
    slice,
  
    str,
};
 
use std::io::{
    self,
    BufRead,
    Write,
};
use regex::bytes::Regex;

use std::convert::TryInto;
use bstr::ByteSlice;

//  use regex::RegexSet;
//  use std::collections::HashSet;


// pub fn to_hex_string(bytes: Vec<u8>) -> String {
//     let strs: Vec<String> = bytes.iter()
//                                  .map(|b| format!("{:02X}", b))
//                                  .collect();
//     strs.connect(" ")
//   }
#[derive(Serialize, Deserialize, Debug,Clone)]
struct Decoder {
    id: String,
    dp: String,
    size: i32,
    multiplier: f32,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
struct Packet {
    id: String,
    dp: String,
    size: i32,
    multiplier: f32,
}




fn main(){




    let mut should_data_link=true;
    let mut should_space_packet=true;
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut decodex_ = String::new();

    let extensions_to_search=["Tlm"];
    let mut files_to_parse:Vec<String>=Vec::new();


    'outer:loop {
        println!("Please type a relative directory where the data resides in...");
        let mut buf = String::new();
        stdin.read_line(&mut buf);
  
        let path = Path::new(&buf);
        let v2=read_filenames_from_dir(buf.trim());
   
        for (pos, e) in v2.iter().enumerate() {
        for extension in extensions_to_search{
            let mut checker=false;
            for file in e{
           
        
                match file.to_str(){
                        Some(p) if p.contains(extension) => {
                            checker=true;
                            files_to_parse.push(p.to_string());


                        }
                        _ => {
                          
                        }
                                }


                        }
                        if checker==true{
                            break 'outer

                        }else{
                            println!("No file found to parse within the path.");

                        }
                    }
                }
            }            



            loop {
                println!("Please enter the decoder json location");
                let mut buf = String::new();
        
                stdin.read_line(&mut buf);
                decodex_=buf.trim().to_string();
                break
            }
        



    loop {
        println!("Do you want to use a Data Link Protocol (yes/no)?");
        let mut buf = String::new();

        stdin.read_line(&mut buf);

        let yes = String::from("yes");

        match buf.trim() {
            "yes" => {
                should_data_link=true;
                break},
            "no"=>{
                should_data_link=false;
                break},
             _ => println!("Please enter yes or no")
        };
    }


  
    loop {
        println!("Do you want to use a Space Packet Protocol (yes/no)?");
        let mut buf = String::new();

        stdin.read_line(&mut buf);

        let yes = String::from("yes");

        match buf.trim() {
            "yes" => {
                should_space_packet=true;
                break},
            "no"=>{
                should_space_packet=false;
                break},
             _ => println!("Please enter yes or no")
        };
    }



    for file in files_to_parse{

        match std::fs::read(file.clone()) {
            Ok(bytes) => { 
                let mut lol=&bytes.clone();
                let mut data_vec: Vec<Vec<u8>>=Vec::new();
                let mut temp: Vec<u8>=Vec::new();
                let mut first_time=true;
                let mut ids = lol
                .windows(4)
                .enumerate()
                .filter(|(_pos, window)| *window == &[0x50u8, 0x44, 0x4f, 0x54])
                .map(|(pos, _window)| pos);
    
    
    
                    let mut prev_id = 0;
                    let lol2 = &lol;
                    let slicer = move || {
                        if let Some(id) = ids.next() {
                            let r = Some(&lol2[prev_id..id]);
                            prev_id = id;
                            return r;
                        }
                        if prev_id != 0 {
                            let r = Some(&lol2[prev_id..]);
                            prev_id = 0;
                            return r;
                        }
                        return None;
                    };
    
                    let mut slicer = std::iter::from_fn(slicer);
                    let mut results = slicer.collect::<Vec<_>>();
  
                    let s = format!("{:?}", &[80,68,79,84]);

                    let original_x = i16::from_be_bytes([143,10]);   // original_x = 65535 = x
      
                results.remove(0);
                
                let data = fs::read_to_string(&decodex_).expect("Unable to read file");
            
                let space_packet_decoder=r#"
                [
                    {
                        "id":"Version",
                        "dp":"uint8",
                        "size":1,
                        "multiplier":1
                    },
                    {
                        "id":"Ap ID",
                        "dp":"uint8",
                        "size":1,
                        "multiplier":1
                    },
                    {
                        "id":"Seq Number",
                        "dp":"uint16LE",
                        "size":2,
                        "multiplier":1
                    },
                    {
                        "id":"Packet Size",
                        "dp":"uint16LE",
                        "size":2,
                        "multiplier":1
                    }
                ]"#;
        
        
        
                let data_link_decoder=r#"
                [
                    {
                        "id":"Spacecraft ID",
                        "dp":"uint16LE",
                        "size":2,
                        "multiplier":1
                    },
                    {
                        "id":"MC Frame Count",
                        "dp":"uint8",
                        "size":1,
                        "multiplier":1
                    },
                    {
                        "id":"VC Frame Count",
                        "dp":"uint8",
                        "size":1,
                        "multiplier":1
                    },
                    {
                        "id":"TF Datafield Status",
                        "dp":"uint16LE",
                        "size":2,
                        "multiplier":1
                    }
                ]"#;
                
                let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
                let mut whole_decoder:Vec<Decoder>=Vec::new();   
                let mut packet_header_csv:Vec<String>=Vec::new();     
                let mut _pos=4;
                if should_data_link==true {
                    let data_link_protocol: Vec<Decoder> = serde_json::from_str(&data_link_decoder).expect("Unable to Data Link Packet");
                    whole_decoder.extend(data_link_protocol);
            
                }
                if should_space_packet==true {
                    let space_packet_protocol: Vec<Decoder> = serde_json::from_str(&space_packet_decoder).expect("Unable to Space packet");
                    whole_decoder.extend(space_packet_protocol);
            
                }
                let json_decoder: Vec<Decoder> = serde_json::from_str(&data).expect("Unable to parse");
                whole_decoder.extend(json_decoder.clone());
       
                let mut output_path = format!("{}{}", file.clone(), ".csv");

                let mut wtr = csv::Writer::from_path(output_path).unwrap();
                for element in whole_decoder.clone(){
                    packet_header_csv.push(element.id);
                }
                wtr.write_record(&packet_header_csv);
                let delay = time::Duration::from_millis(100);


                for data in results.clone(){
                    let mut vectored_packet=parse_frame(data,whole_decoder.clone());
                    wtr.write_record(vectored_packet);
            
    
                }
                wtr.flush();
                
        }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    eprintln!("please run again with appropriate permissions.");
                    return;
                }
                panic!("{}", e);
            }
        }

        println!("PARSED")



    }

    





        fn first4(bytes: &[u8]) -> &[u8; 4] {
            array_ref![bytes, 0, 4]
       }

       fn first2(bytes: &[u8]) -> &[u8; 2] {
        array_ref![bytes, 0, 2]
   }

   fn first1(bytes: &[u8]) -> &[u8; 1] {
    array_ref![bytes, 0, 1]
}
   fn first8(bytes: &[u8]) -> &[u8; 8] {
    array_ref![bytes, 0, 8]
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}






//Parses an individual frame with u8 as input and vector of elemnts for csv conversion as output
fn parse_frame(mut frame:&[u8],mut whole_decoder:Vec<Decoder>)->Vec<String> {



    let frame_vec: Vec<u8> = frame.iter().cloned().collect();
    let mut packet_vec_for_csv: Vec<String>=Vec::new();


    
    let mut size_=0;
    for i in 0..whole_decoder.len(){
        size_=size_+whole_decoder[i].size;
    }

    let mut _pos=4;
    for i in 0..whole_decoder.len(){
 
        if whole_decoder[i].dp=="int32LE"{
            
            let arr=first4(&frame_vec[_pos.._pos+4]);
            let te=arr.clone();
            let mut k=i32::from_ne_bytes(te);
            let temp=&whole_decoder[i].multiplier;
            packet_vec_for_csv.push((temp*k as f32).to_string());
         
            _pos=_pos+4
            // frame.drain(0..5);

        }
        else if whole_decoder[i].dp=="uint16LE"{
            let arr=first2(&frame_vec[_pos.._pos+2]);
            let te=arr.clone();

            _pos=_pos+2;
            let mut k=i16::from_ne_bytes(te);
            let temp=&whole_decoder[i].multiplier;
            packet_vec_for_csv.push((temp*k as f32).to_string());

      
            // frame.drain(0..3);

        }
        else if whole_decoder[i].dp=="FloatLE"{
            let arr=first4(&frame_vec[_pos.._pos+4]);
            let te=arr.clone();       
            let mut k=f32::from_ne_bytes(te);
            let temp=&whole_decoder[i].multiplier;
            packet_vec_for_csv.push((temp*k as f32).to_string());
            _pos=_pos+4
            // frame.drain(0..5);
        }
        else if whole_decoder[i].dp=="FloatBE"{
            let arr=first4(&frame_vec[_pos.._pos+4]);
            let te=arr.clone();
            let mut k=f32::from_ne_bytes(te);
            let temp=&whole_decoder[i].multiplier;
            packet_vec_for_csv.push((temp*k as f32).to_string());
            _pos=_pos+4
            // frame.drain(0..5);
        }
        else if whole_decoder[i].dp=="DoubleLE"{
            let arr=first8(&frame_vec[_pos.._pos+8]);
            let te=arr.clone();
            let mut k=i64::from_ne_bytes(te);
            let temp=&whole_decoder[i].multiplier;
            packet_vec_for_csv.push((temp*k as f32).to_string());

       
            _pos=_pos+8
            // frame.drain(0..9);
        }
        else if whole_decoder[i].dp=="uint8"{
            let arr=first1(&frame_vec[_pos..]);
            let te=arr.clone();
            let mut k=i8::from_ne_bytes(te);
            let temp=&whole_decoder[i].multiplier;
            packet_vec_for_csv.push((temp*k as f32).to_string());

       
            _pos=_pos+1
            // frame.drain(0..9);
        }
        else if whole_decoder[i].dp=="hex"{
            let mut temp_values=hex::encode(&frame_vec[_pos.._pos+2]);

       
            _pos=_pos+2
            // frame.drain(0..9);
        }

    }
    

    packet_vec_for_csv



}

fn insert_in_place<T>(array: &mut [T], value: T, index: usize) {
    array[index..].rotate_right(1);
    array[index] = value;
  }



}
pub fn read_filenames_from_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}








