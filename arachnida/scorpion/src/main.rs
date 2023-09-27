use std::{env, fs, io};

fn main() {
    for path in env::args().skip(1) {
        let file = match fs::File::open(&path) {
            Err(e) => {
                eprintln!("Error {}: {e}", &path);
                continue;
            }
            Ok(file) => file,
        };
        let mut bufreader = io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = match exifreader.read_from_container(&mut bufreader) {
            Err(e) => {
                eprintln!("Error on {}: {e}", &path);
                continue;
            }
            Ok(exif) => exif,
        };
        println!("\n{path}:");
        for f in exif.fields() {
            println!(
                "{} {} {}",
                f.tag,
                f.ifd_num,
                f.display_value().with_unit(&exif)
            );
        }
    }
}
