#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

// use std::sync::Mutex;
use tauri::{api::process::{Command, CommandEvent}, Manager, Window, App, Wry};

// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      println!("Setting up the app...");
      let window = app.get_window("main").unwrap();
      window.open_devtools();

      let id = app.listen_global("messageForRustServer", |event| {
        println!("got messageForRustServer with payload {:?}", event.payload());
      });

      // unlisten to the event using the `id` returned on the `listen_global` function
      // an `once_global` API is also exposed on the `App` struct
      // app.unlisten(id);

      // create_child_process(window);
      tauri::async_runtime::spawn(async move {
        let (mut receiver, mut child) = Command::new_sidecar("app")
          .expect("failed to setup `app` sidecar")
          .spawn()
          .expect("Failed to spawn packaged node");

//         let mut i = 0;
//         while let Some(event) = receiver.recv().await {
//           if let CommandEvent::Stdout(line_from_child_process_std_out) = event {
//             window
//               .emit("messageForUi", Some(format!("(RustServer1): We have just received a message from the node app...'")))
//               .expect("failed to emit event");
//             window
//               .emit("messageForUi", Some(format!("(RustServer1): '{}'", line_from_child_process_std_out)))
//               .expect("failed to emit event");
//             i += 1;
//             if i == 4 {
//               child.write("(RustServer1): Something has happened in the Rust Server!\n".as_bytes()).unwrap();
//               i = 0;
//             }
//           }
//         }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
