use extism::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::{error::Error, io::Read};
use url::Url;
type Result<T> = color_eyre::eyre::Result<T, Box<dyn Error>>;

struct Storage {
    file: File,
    bytes: Vec<u8>,
}

fn read_file(filename: &str) -> Result<Storage> {
    let mut file = File::options()
        .create(true)
        .read(true)
        .write(true)
        .open(filename)?;

    let mut data = vec![];
    file.read_to_end(&mut data)?;
    let s = Storage {
        file: file,
        bytes: data,
    };
    Ok(s)
}
fn from_cache(url: &str) -> Result<Wasm> {
    let url_str = url;
    let url = Url::parse(url)?;
    let path = url.path();
    let filename = path.split('/').last().unwrap_or("");
    let filename = format!("plugins/{}", filename);
    let mut s = read_file(&filename)?;
    if s.bytes.len() == 0 {
        let resp = reqwest::blocking::get(url_str)?;
        let body = resp.bytes()?;
        s.file.write_all(&body.to_vec())?;
    }
    let wasm = Wasm::data(s.bytes);
    Ok(wasm)
}
fn greet(greeting: &str) -> Result<String> {
    let wasm = from_cache("https://github.com/extism/plugins/releases/latest/download/greet.wasm")?;
    let manifest = Manifest::new([wasm]);
    let mut plugin = Plugin::new(&manifest, [], true)?;
    let res = plugin.call::<&str, &str>("greet", greeting)?;
    Ok(res.to_string())
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let greeting = greet(name)?;
    println!("{}", greeting);
    Ok(())
}
