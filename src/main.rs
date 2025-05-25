use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("numbers.txt")?;
    let mut writer = BufWriter::new(file);
    let mut num: f32 = 1.0;
    let mut num_2: f32 = num / 2.0 + 1.0;
    let mut iscom: f32;
    loop {
        iscom = num / num_2;
        if iscom.fract() == 0.0 {
            num += 1.0;
            num_2 = num - 1.0;
        } else if num_2 == 2.0 {
            writeln!(writer, "{}", num)?;
            println!("{}", num);
            num += 1.0;
            num_2 = num - 1.0;
        } else {
            num_2 -= 1.0;
        }
    }
}
