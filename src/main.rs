use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let mut count = 0;
    while count < 10 {
        count += 1;
    }
    let s = "Hello fellow Rustaceans! aoa как дела братья ";

    if count >= 10 {
        let message = String::from(format!("{s}{count}"));
        let width = message.chars().count();

        let mut writer = BufWriter::new(stdout.lock());
        say(&message, width, &mut writer).unwrap();
    }
}
