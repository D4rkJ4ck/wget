use {clap::Parser, wget::Args};

#[tokio::main]
async fn main() {
    let args: Args = match Args::try_parse() {
        Ok(args) => args,
        Err(error) => {
            eprint!("{error}");
            return;
        }
    };

    println!("{args:#?}")
}
