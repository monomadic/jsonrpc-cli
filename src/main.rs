use std::path::PathBuf;

mod rpc;

static USAGE:&str = r#"
usage: rpc <hostname> <method> [<json_file>]
"#;

pub fn main() {
    match run() {
        Ok(json) => println!("{}", json),
        Err(e) => println!("Error: {:?}", e),
    };
}

pub fn run() -> Result<String, Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let hostname: &str = args.get(1).expect(USAGE);
    let method: &str = args.get(2).expect(USAGE);

    let json: serde_json::Value = match args.get(3) {
        Some(filename) => read_json(filename)?,
        None => serde_json::from_str("{}")?,
    };

    Ok(rpc::rpc_request(hostname, method, json)?)
}

pub fn read_json(filename: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let file_contents: String = read_file(PathBuf::from(filename))?;
    Ok(serde_json::from_str(&file_contents)?)
}

pub fn read_file(pathbuf: PathBuf) -> Result<String, failure::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(pathbuf.clone()).map_err(|_| {
        failure::format_err!(
            "Could not open or read file: {}",
            pathbuf.to_str().unwrap_or("")
        )
    })?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}