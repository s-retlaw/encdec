use std::{error::Error, io::Write};
use std::io;
use std::io::Read;
use clap::{Parser, Subcommand};

// #[derive(Subcommand)]
// enum Command{
//     Enc,
//     Dec
// }
//
// enum Enc{
//     B64,
//     Percent,
// }
//
// enum Dec{
//     B64,
//     Percent
// }
//
#[derive(Subcommand)]
enum Command{
    #[clap(name="b64e")]
    Base64Encode,
    #[clap(name="b64d")]
    Base64Decode,
    #[clap(name="pe")]
    PercentEncode,
    #[clap(name="pd")]
    PercentDecode
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli{
    #[clap(subcommand)]
    command : Command
}

//fn base64_encode(buf : &(impl AsRef<[u8]> + ?Sized)) -> Result<String, Box<dyn Error>> {
fn base64_encode(buf : &(impl AsRef<[u8]> + ?Sized)) -> Result<String, Box<dyn Error>> {
    Ok(base64::encode_config(buf, base64::STANDARD))
}

fn base64_decode(buf : &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let new_buf : Vec<u8> = buf.iter().filter_map(|i| if (*i != 10_u8) && (*i != 13_u8) {Some(*i)}else{None}).collect();
    let decoded_buf = base64::decode_config(new_buf, base64::STANDARD)?;
    Ok(decoded_buf)
}

fn percent_encode(buf :&(impl AsRef<[u8]> + ?Sized)) -> Result<String, Box<dyn Error>> {
    Ok(percent_encoding::percent_encode(buf.as_ref(), percent_encoding::NON_ALPHANUMERIC).to_string())
}

fn percent_decode(buf : &(impl AsRef<[u8]> + ?Sized)) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(percent_encoding::percent_decode(buf.as_ref()).collect())
}

fn main() -> Result<(), Box<dyn Error>>{
    let cli = Cli::parse();

    let mut buf : Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    match cli.command {
        Command::Base64Decode => io::stdout().lock().write_all(&base64_decode(&buf)?)?,
        Command::Base64Encode => io::stdout().lock().write_all(base64_encode(&buf)?.as_bytes())?,
        Command::PercentDecode => io::stdout().lock().write_all(&percent_decode(&buf)?)?,
        Command::PercentEncode => io::stdout().lock().write_all(percent_encode(&buf)?.as_bytes())?,
    };
    Ok(())
}

#[cfg(test)]
mod tests{
    use crate::*;
    
    #[test]
    fn test_base_64_empty(){
        let data = "";

        let expected = "".to_string();
        let actual = base64_encode(data).unwrap();
        assert_eq!(expected, actual, "encode empty");

        let expected : Vec<u8> = vec![];
        let actual = base64_decode(&vec![]).unwrap();
        assert_eq!(expected, actual, "decode empty");
    }

    #[test]
    fn test_base_64_simple(){
        let data = "Hello World!!";

        let expected = "SGVsbG8gV29ybGQhIQ==".to_string();
        let actual = base64_encode(data).unwrap();
        assert_eq!(expected, actual, "encode");

        let expected = data.as_bytes();
        let mut data : Vec<u8> = actual.as_bytes().into();
        let actual = base64_decode(&data).unwrap();
        assert_eq!(expected, actual, "decode");
       
        //this is testing for removing of carriage return and new lines
        data.insert(5, 10);
        data.insert(7, 13);
        let actual = base64_decode(&data).unwrap();
        assert_eq!(expected, actual, "decode");
    }

    #[test]
    fn test_percent_empty(){
        let data = "";

        let expected = "".to_string();
        let actual = percent_encode(data).unwrap();
        assert_eq!(expected, actual, "encode empty");

        let expected : Vec<u8> = vec![];
        let actual = percent_decode(data).unwrap();
        assert_eq!(expected, actual, "decode empty");
    }
    
    #[test]
    fn test_percent_simple(){
        let data = "!@#$%^&*(){}[];':\"Hello World!!";

        let expected = "%21%40%23%24%25%5E%26%2A%28%29%7B%7D%5B%5D%3B%27%3A%22Hello%20World%21%21".to_string();
        let actual = percent_encode(data).unwrap();
        assert_eq!(expected, actual, "encode");

        let expected = data.as_bytes();
        let data = actual;
        let actual = percent_decode(&data).unwrap();
        assert_eq!(expected, actual, "decode");
    }
}
