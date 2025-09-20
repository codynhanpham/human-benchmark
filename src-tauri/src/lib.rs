use tauri::{Manager, RunEvent, WindowEvent};

mod game;
mod screen;
mod ocr;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Init the Builder and configure plugins
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init());

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    // Registering Rust functions
    builder = builder.invoke_handler(tauri::generate_handler![
        exit_app,
        screen::utils::get_mouse_position,
        screen::utils::tauri_detect_play_arena,
        game::reaction_time::start_reaction_time,
        game::aim_training::start_aim_training,
        game::typing::start_typing_test,
        game::number_memory::start_number_memory,
    ]);

    let app = builder
        .setup(|app| {
            #[cfg(desktop)]
            {
                // Run command on startup and perhaps, configure the webview window
                let _app_handle = app.handle();
                let _window: tauri::WebviewWindow = app.get_webview_window("main").unwrap();
                // _window.hide().unwrap();
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // Start the application event loop
    app.run(move |_app_handle, event| {
        match event {
            RunEvent::ExitRequested { api, code, .. } => {
                // Keep the event loop running even if all windows are closed
                // This allow us to catch tray icon events when there is no window
                // if we manually requested an exit (code is Some(_)) we will let it go through
                if code.is_none() {
                    api.prevent_exit();
                }
            }

            RunEvent::WindowEvent {
                event: WindowEvent::CloseRequested { api, .. },
                label,
                ..
            } => {
                api.prevent_close();

                let app_handle = _app_handle.clone();
                let window_label = label.clone();

                // Do tasks before closing the window here
                // ...


                // Close the window after cleanup
                if let Some(window) = app_handle.get_webview_window(&window_label) {
                    let _ = window.destroy();
                }

                // End the app
                app_handle.exit(0i32);
            }

            _ => {}
        }
    });
}

#[tauri::command]
/// A quick function to exit the main app
fn exit_app(app_handle: tauri::AppHandle) {
    // app_handle.exit(0i32); // This function kill the app process without triggering the cleanup logic!

    for (_, window) in app_handle.webview_windows() {
        let _ = window.close();
    }
}