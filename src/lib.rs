use filetime::FileTime;
use jwalk::WalkDir;
use std::path::{self};
use std::{fs, io, thread};

fn file_eq(f1: &path::PathBuf, f2: &path::PathBuf) -> io::Result<bool> {
    let f1_meta = fs::metadata(f1)?;
    let f2_meta = fs::metadata(f2)?;

    if f1_meta.len() == f2_meta.len() && f1_meta.modified()? == f2_meta.modified()? {
        return Ok(true);
    }

    Ok(false)
}

pub fn copy(from: path::PathBuf, to: path::PathBuf) -> io::Result<()> {
    let mut write_handles = Vec::new();

    for entry in WalkDir::new(&from) {
        let input_path = entry?.path();
        if input_path.is_dir() {
            continue;
        }

        let from = from.clone();
        let to = to.clone();
        let output_path = to.join(input_path.strip_prefix(from).unwrap());

        let handle = thread::spawn(|| {
            if output_path.exists() && file_eq(&input_path, &output_path).unwrap() {
                return;
            }

            fs::create_dir_all(output_path.parent().unwrap()).expect("Failed to create dirs");

            let input_fp = fs::File::open(input_path).expect("Failed to open input file.");
            let output_fp = fs::File::create(&output_path)
                .unwrap_or_else(|_| panic!("Failed to open output file {}", output_path.display()));

            let input_modtime =
                FileTime::from_last_modification_time(&input_fp.metadata().unwrap());

            let mut input_reader = io::BufReader::new(input_fp);
            let mut output_writer = io::BufWriter::new(output_fp);

            io::copy(&mut input_reader, &mut output_writer).expect("Failed to copy files.");

            filetime::set_file_mtime(output_path, input_modtime)
                .expect("Failed to set last modified time of the file.");
        });

        write_handles.push(handle);
    }

    for handle in write_handles {
        handle.join().unwrap();
    }

    Ok(())
}
