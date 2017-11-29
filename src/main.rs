extern crate curl;
extern crate zip;

use std::io::Write;
use curl::easy::Easy;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::path::Path;

fn main() {
    println!("LMP Updater v0.0.2.\nMade by The HellBox");
    let args: Vec<_> = std::env::args().collect();
    let target: String =
    if args.len() < 2 {
        "client".to_string()
    }
    else{
        args[1].clone()
    };
    let repo = "http://127.0.0.1/LMP-releases/";
    let scr = format!("{}{}.zip", repo, target);
    let mut curl = Easy::new();
    let mut data = Vec::new();
    let _ = curl.url(&scr).unwrap();
    let _ = curl.follow_location(true);
    let _ = curl.autoreferer(false);
    println!("Downloading {} from {}", target, &scr);
    {
        let mut transfer = curl.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let mut file = File::create(&format!("./{}.zip", target)).unwrap();

    let _ = file.write_all(&data);

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(data)).unwrap();

     for i in 0..archive.len() {
         let mut zipfile = archive.by_index(i).unwrap();
         println!("{}", zipfile.name());
         if zipfile.name().ends_with("/") {
             fs::create_dir(zipfile.name());
         }
         else{
             let mut newfile = File::create( &format!( "./{}", zipfile.name() ) ).unwrap();
             let mut buffer: Vec<u8> = vec![];
             let _ = zipfile.read_to_end(&mut buffer);
             let _ = newfile.write_all(&buffer);
         }
     }
}