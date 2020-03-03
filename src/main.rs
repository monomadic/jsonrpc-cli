use std::path::PathBuf;

mod rpc;

static USAGE:&str = r#"
usage: rpc <hostname> <method> [<json_file>]
"#;

pub fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("Error: {:?}", e),
    };
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let hostname: &str = args.get(1).expect(USAGE);
    let method: &str = args.get(2).expect(USAGE);

    let json: String = match args.get(3) {
        Some(filename) => read_file(PathBuf::from(filename))?,
        None => "".to_string(),
    };

    let json_result = rpc::rpc_request(hostname, method, serde_json::json!(json))?;
    let result = serde_json::from_str(&json_result)?;
    // Ok(result) // fix

    println!("{:?}", result);

    Ok(())
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