use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out1 = b"Hello other Rustaceans!";
    let out2 = b"> rustup docs --book";
    let width = 24;
    
    let mut writer = BufWriter::new(stdout.lock());

    say(out1,width, &mut writer).unwrap();

    say(out2, width, &mut writer).unwrap();
}
