use filetime::FileTime;
use jwalk::WalkDir;
use std::path::{self};
use std::{fs, io};
use threadpool::ThreadPool;

fn file_eq(f1: &path::PathBuf, f2: &path::PathBuf) -> io::Result<bool> {
    let f1_meta = fs::metadata(f1)?;
    let f2_meta = fs::metadata(f2)?;

    if f1_meta.len() == f2_meta.len() && f1_meta.modified()? == f2_meta.modified()? {
        return Ok(true);
    }

    Ok(false)
}

pub fn copy(from: path::PathBuf, to: path::PathBuf) -> io::Result<()> {
    let num_threads = num_cpus::get();
    let pool = ThreadPool::new(num_threads);

    for entry in WalkDir::new(&from) {
        let input_path = entry?.path();
        if input_path.is_dir() {
            continue;
        }

        let from = from.clone();
        let to = to.clone();
        let output_path = to.join(input_path.strip_prefix(from).unwrap());

        pool.execute(move || {
            if output_path.exists() && file_eq(&input_path, &output_path).unwrap() {
                return;
            }

            fs::create_dir_all(output_path.parent().unwrap()).expect("Failed to create dirs.");

            let input_fp = fs::File::open(&input_path).expect("Failed to open input file.");

            let input_mtime: FileTime =
                FileTime::from_last_modification_time(&input_fp.metadata().unwrap());
            let input_atime: FileTime =
                FileTime::from_last_access_time(&input_fp.metadata().unwrap());

            fs::copy(input_path, &output_path).unwrap();

            filetime::set_file_times(output_path, input_atime, input_mtime)
                .expect("Failed to set mtime and atime.");
        });
    }

    Ok(())
}
