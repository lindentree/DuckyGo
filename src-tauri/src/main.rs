#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate gtp;

// use gtp::controller::Engine;
// use gtp::Command;

use std::io::Write;
use std::process::{Command, Output};
//use tokio::process::Command;
use std::{process::Stdio, time::Duration};
use tauri::plugin::Builder;

mod katago;

// let mut ctrl = Engine::new("/usr/local/Cellar/katago/1.13.2/bin/katago", &["--mode", "gtp"]);
//       assert!(ctrl.start().is_ok());

//       ctrl.send(Command::cmd("name", |e| e));
//       let resp = ctrl.wait_response(Duration::from_millis(500)).unwrap();
//       let ev = resp.entities(|ep| ep.s().s()).unwrap();
//       println!("testing {:?}", ev);
#[tokio::main]
async fn main() {
    // async fn kata_go_call() -> Output {
    //     let mut engine = Command::new("/usr/local/Cellar/katago/1.13.2/bin/katago")
    //         .arg("gtp")
    //         .arg("-model ")
    //         .arg("/Users/lindentree/.katrain/kata1-b18c384nbt-s6857212928-d3491594038.bin.gz")
    //         .arg("-config ")
    //         .arg("/Users/lindentree/Desktop/gtp_example.cfg")
    //         .output()
    //         .expect("failed to execute process");
    // }

    let mut engine = Command::new("/usr/local/Cellar/katago/1.13.2/bin/katago")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("gtp")
        .arg("-model ")
        .arg("/Users/lindentree/.katrain/kata1-b18c384nbt-s6857212928-d3491594038.bin.gz")
        .arg("-config ")
        .arg("/Users/lindentree/Desktop/gtp_example.cfg")
        .spawn()
        .expect("Failed to spawn child process");

    println!("{:?}", engine);

    let cmd = Command::new("name");

    let mut child_stdin = engine.stdin.take().expect("Failed to open stdin");

    child_stdin
        .write_all(b"showboard\n")
        .expect("Failed to write");

    drop(child_stdin);

    let output = engine.wait_with_output();

    println!("output = {:?}", output.unwrap().stdout);

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
