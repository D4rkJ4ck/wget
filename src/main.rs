use std::env;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("The normal usage of wget example: wget https://some_url.ogr/file.zip");
    }

    dbg!(&args[1..]);

    Ok(())
}
