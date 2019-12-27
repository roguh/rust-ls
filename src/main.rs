// ls -a -F
use chrono::offset::Utc;
use chrono::DateTime;
use std::fs::{canonicalize, metadata, symlink_metadata, read_dir};
use pwd::Passwd;
use std::io::{Error, ErrorKind};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

fn print_as_str(p:std::path::PathBuf) -> std::result::Result<(), Error> {
    match p.to_str() {
        Some(s) => {
            print!("{}", s);
            Ok(())
        }
        None => Err(Error::new(ErrorKind::Other, "failed to convert path"))
    }
}

fn print_filename_as_str(p:std::path::PathBuf) -> std::result::Result<(), Error> {
    match p.file_name() {
        Some(f) => {
            match f.to_str() {
                Some(s) => {
                    print!("{}", s);
                    Ok(())
                },
                None => Err(Error::new(ErrorKind::Other, "failed to convert path"))
            }
        },
        None => Err(Error::new(ErrorKind::Other, "failed to convert path"))
    }
}

fn main() -> std::result::Result<(), Error> {
    let r = read_dir(".")?;
    for entry in r {
        let path = entry?.path();
        let m = symlink_metadata(path.clone())?;
        let print_type = true;
        let print_attrs = true;

        if print_attrs {
            let sep = "  ";

            print!("{:x}{}", m.permissions().mode(), sep);

            let username = match Passwd::from_uid(m.uid()) {
                Some(passwd) => passwd.name,
                None => "?".to_string(),
            };
            print!("{}{}", username, sep);

            let mtime = m.modified()?;
            let datetime: DateTime<Utc> = mtime.into();
            print!("{}{}", datetime.format("%d %b %Y %I:%M"), sep);
        }

        print_filename_as_str(path.clone())?;

        if print_type {
            if metadata(path.clone())?.is_dir() {
                print!("/");
            } else {
                if m.permissions().mode() & 0o111 != 0 {
                    print!("*");
                }
            }

            if m.file_type().is_symlink() {
                let link = canonicalize(path)?;
                print!(" -> ");
                print_as_str(link)?;
            }
        }
        print!("\n");
    }
    Ok(())
}
