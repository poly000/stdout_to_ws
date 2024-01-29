use std::{
    process::{Command, Stdio},
    time::Duration,
};

use anyhow::Result;

#[cfg(target_os = "android")]
use win_socket_command::android::{FIO_PATH, WEBSOCAT_PATH};

fn main() -> Result<()> {
    let mut websocat = {
        #[cfg(target_os = "android")]
        let cmd = Command::new(WEBSOCAT_PATH.get_or_init(|| String::new()));
        #[cfg(target_os = "windows")]
        let cmd = Command::new("./websocat");
        #[cfg(not(any(target_os = "windows", target_os = "android")))]
        let cmd = Command::new("websocat");
        cmd
    };

    let mut out = websocat
        .arg("--text")
        .arg("--conncap")
        .arg("1")
        .arg("--exit-on-eof")
        .arg("ws-l:127.0.0.1:11451")
        .arg("reuse-raw:stdio:")
        .stdin(Stdio::piped())
        .spawn()?;

    #[cfg(target_os = "android")]
    let mut fio = Command::new(FIO_PATH);
    #[cfg(not(any(target_os = "android")))]
    let mut fio = Command::new("fio");
    fio.arg("--size=2048M")
        .arg("--name=read")
        .arg("--output-format=normal")
        .arg("--filename=0.txt")
        .arg("--eta=always")
        .arg("--eta-newline=1")
        .arg("--time_based")
        .arg("--runtime=30")
        .stdout(out.stdin.take().unwrap())
        .spawn()?
        .wait()?;

    std::thread::sleep(Duration::from_secs(1));
    out.kill()?;

    Ok(())
}
