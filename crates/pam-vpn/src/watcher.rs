use std::{
    fs::File,
    path::Path,
    time::{Instant, Duration}, thread, io::{BufReader, BufRead},
};

pub fn wait_until_file_created<T: AsRef<Path>>(file_path: T, timeout: Duration, sleep: Duration) -> anyhow::Result<File> {
    let start = Instant::now();

    let file = loop {
        match File::open(&file_path) {
            Ok(file) => {
                break Ok(file);
            },
            Err(e) => {
                if (Instant::now() - start) >= timeout {
                    break Err(e);
                }
                thread::sleep(sleep);
            },
        }
    }?;

    Ok(file)
}

pub fn wait_until_string_found(file: File, sub_string: &str, timeout: Duration, sleep: Duration) -> anyhow::Result<Option<String>> {
    let start = Instant::now();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    loop {
        if (Instant::now() - start) >= timeout {
            anyhow::bail!("timeout: string not found.")
        }
        match reader.read_line(&mut buf) {
            Ok(_) => {
                if buf.contains(sub_string) {
                    return Ok(Some(buf));
                }
            },
            Err(e) => {
                if (Instant::now() - start) >= timeout {
                    break Err(e);
                }
                thread::sleep(sleep);
            }
        }
    }?;

    Ok(None)
}
