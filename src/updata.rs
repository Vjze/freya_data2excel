use retry::{delay::Fibonacci, retry, OperationResult};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tokio::{fs::File, process::Command};

pub async fn downloadfile() -> tokio::io::Result<(PathBuf,PathBuf)> {
    let bin_name = "freya_data2excel.exe";
    let current_bin_path = std::env::current_exe().map_err(|_| ()).unwrap();
    let download_path = current_bin_path
        .parent()
        .ok_or(())
        .unwrap()
        .join(format!("tmp_{bin_name}"));
    let tmp_path = current_bin_path
        .parent()
        .ok_or(())
        .unwrap()
        .join(format!("tmp2_{bin_name}"));
    let mut remote_file =
        File::open("//192.168.3.250/soft/data2excel/freya_data2excel.exe").await?;
    let mut local_file = File::create(&download_path).await?;
    let _ = tokio::io::copy(&mut remote_file, &mut local_file);
    if let Err(e) = rename(&current_bin_path, &tmp_path) {
        println!("u1{}", e.to_string())
    }
    if let Err(e) = rename(&download_path, &current_bin_path) {
        println!("u1{}", e.to_string())
    }
    Ok((current_bin_path,tmp_path))
}
pub async fn updata() -> tokio::io::Result<()> {
    let (relaunch_path,cleanup_path) = downloadfile().await?;
    let mut args = std::env::args();
    args.next();
    let mut args: Vec<_> = args.collect();
    if let Some(idx) = args.iter().position(|a| a == "--self-update-temp") {
        args.remove(idx);
        // Remove path passed after this arg
        args.remove(idx);
    }

    match std::process::Command::new(relaunch_path)
        .args(args)
        .arg("--self-update-temp")
        .arg(&cleanup_path)
        .spawn()
    {
        Ok(_) => {
            if let Err(e) = remove_file(cleanup_path) {
                // error!("Could not remove temp update file: {}", e);
                println!("{}", e.to_string())
            }
            std::process::exit(0)
        }
        Err(error) => {
            if let Err(e) = remove_file(cleanup_path) {
                // error!("Could not remove temp update file: {}", e);
                println!("u1{}", e.to_string())
            }
            // error!("Failed to update UAD: {}", error);
            println!("u2{}", error.to_string())
        }
    }

    Ok(())
}

pub fn rename<F, T>(from: F, to: T) -> Result<(), String>
where
    F: AsRef<Path>,
    T: AsRef<Path>,
{
    let from = from.as_ref();
    let to = to.as_ref();

    retry(Fibonacci::from_millis(1).take(21), || {
        match fs::rename(from, to) {
            Ok(_) => OperationResult::Ok(()),
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => OperationResult::Retry(e),
                _ => OperationResult::Err(e),
            },
        }
    })
    .map_err(|e| e.to_string())
}
pub fn remove_file<P>(path: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    retry(
        Fibonacci::from_millis(1).take(21),
        || match fs::remove_file(path) {
            Ok(_) => OperationResult::Ok(()),
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => OperationResult::Retry(e),
                _ => OperationResult::Err(e),
            },
        },
    )
    .map_err(|e| e.to_string())
}
