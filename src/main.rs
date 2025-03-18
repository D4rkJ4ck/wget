use {
    clap::Parser,
    wget::Args,
};

#[tokio::main]
async fn main() {
    let args: Args = match Args::try_parse() {
        Ok(raw_args) => match raw_args.validate() {
            Ok(valid_args) => valid_args,
            Err(_) => return,
        },
        Err(e) => {
            dbg!(e);
            return;
        }
    };

    println!("{args:#?}")
}
