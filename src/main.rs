use std::{
    ffi::OsString,
    os::windows::ffi::OsStringExt,
    process::{Command, Stdio},
};

use anyhow::Result;

use linereader::LineReader;
use websocket::{server::WsServer, Message};
#[cfg(target_os = "android")]
use win_socket_command::android::FIO_PATH;

fn main() -> Result<()> {
    #[cfg(target_os = "android")]
    let mut fio = Command::new(FIO_PATH);
    #[cfg(not(any(target_os = "android")))]
    let mut fio = Command::new("fio");

    let mut fio = fio
        .arg("--size=2048M")
        .arg("--name=read")
        .arg("--output-format=normal")
        .arg("--filename=0.txt")
        .arg("--eta=always")
        .arg("--eta-newline=1")
        .arg("--time_based")
        .arg("--runtime=30")
        .stdout(Stdio::piped())
        .spawn()?;

    let fio_out = fio.stdout.take().unwrap();

    let ws_server = WsServer::bind("127.0.0.1:11451")?.accept().unwrap();
    let mut client = ws_server.accept().unwrap();

    client.send_message(&Message::text("Hello world!"))?;

    let mut fio_out_reader = linereader::LineReader::new(fio_out);
    while let Some(Ok(s)) = fio_out_reader.next_line() {
        client.send_message(&Message::text(String::from_utf8_lossy(s)))?;
    }

    Ok(())
}
