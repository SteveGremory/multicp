use filetime::FileTime;
use jwalk::WalkDir;
use std::{
    path::{self},
    sync::Arc,
};
use tokio::{fs, io, sync::Semaphore};

async fn file_eq(f1: &path::PathBuf, f2: &path::PathBuf) -> io::Result<bool> {
    let f1_meta = fs::metadata(f1).await?;
    let f2_meta = fs::metadata(f2).await?;

    if f1_meta.len() == f2_meta.len() && f1_meta.modified()? == f2_meta.modified()? {
        return Ok(true);
    }

    Ok(false)
}

pub async fn copy(from: path::PathBuf, to: path::PathBuf) -> io::Result<()> {
    let mut tasks = Vec::new();

    let semaphore = Arc::new(Semaphore::new(256));

    for entry in WalkDir::new(&from) {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let input_path = entry?.path();
        if input_path.is_dir() {
            continue;
        }

        let from = from.clone();
        let to = to.clone();
        let output_path = to.join(input_path.strip_prefix(from).unwrap());

        fs::create_dir_all(output_path.parent().unwrap())
            .await
            .expect("Failed to create dirs.");

        let handle = tokio::spawn(async {
            if output_path.exists() && file_eq(&input_path, &output_path).await.unwrap() {
                return;
            }

            let input_fp = fs::File::open(&input_path)
                .await
                .expect("Failed to open input file.");

            let input_mtime: FileTime =
                FileTime::from_last_modification_time(&input_fp.metadata().await.unwrap());
            let input_atime: FileTime =
                FileTime::from_last_access_time(&input_fp.metadata().await.unwrap());

            fs::copy(input_path, &output_path).await.unwrap();

            filetime::set_file_times(output_path, input_atime, input_mtime)
                .expect("Failed to set mtime and atime.");

            drop(permit);
        });

        tasks.push(handle);
    }

    for handle in tasks {
        handle.await.unwrap();
    }

    Ok(())
}
