use std::fs::{copy, create_dir, remove_file, rename};
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

pub fn take<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dest: Q, amount: u64) -> io::Result<()> {
    if !src.as_ref().is_dir() {
        return Err(io::Error::new(
            ErrorKind::NotADirectory,
            "src is not a directory",
        ));
    }

    if !dest.as_ref().exists() {
        create_dir(dest.as_ref())?;
    }

    if !dest.as_ref().is_dir() {
        return Err(io::Error::new(
            ErrorKind::NotADirectory,
            "dest is not a directory",
        ));
    }

    let mut limit = 0;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let from = entry.path();
        let to = dest.as_ref().join(entry.file_name());

        if to.exists() {
            return Err(io::Error::new(
                ErrorKind::AlreadyExists,
                "destination file already exists",
            ));
        }

        move_file(&from, &to)?;

        limit += 1;
        if amount <= limit {
            break;
        }
    }

    Ok(())
}

fn move_file(src: &Path, dst: &Path) -> io::Result<()> {
    move_file_with(src, dst, |src, dst| rename(src, dst))
}

fn move_file_with<R: FnOnce(&Path, &Path) -> io::Result<()>>(
    src: &Path,
    dst: &Path,
    rename: R,
) -> io::Result<()> {
    match rename(src, dst) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::CrossesDevices => {
            copy(src, dst)?;
            remove_file(src)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn move_file_with_test() {
        let temp = tempfile::tempdir().unwrap();

        let src = temp.path().join("src.txt");
        {
            let mut src_file = File::create_new(&src).unwrap();
            writeln!(src_file, "test!").unwrap();
        }
        let dest = temp.path().join("dest.txt");

        move_file(src.as_path(), dest.as_path()).unwrap();

        assert!(dest.exists());
        assert_eq!(fs::read_to_string(&dest).unwrap(), "test!\n");
    }
    // TODO test fallback on CrossesDevices
}
