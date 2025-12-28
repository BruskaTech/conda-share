// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use slint::SharedString;

use conda_share_core::*;

slint::include_modules!();

// TODO: Add output file name option (default to the file name)
// TODO: When there is already a file with the same name, ask the user to confirm overwriting
// TODO: Write the callback for "Create Env File" button to generate and save the file
//         - After the file is created, show a message box to inform the user of success or failure or make the button disabled again
// TODO: Make it so that every time you click the ComboBox it updates what is inside

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let env_list = conda_env_list().unwrap();
    let ui_env_list = env_list
        .iter().map(|e| e.into()).collect::<Vec<slint::SharedString>>()
        .as_slice().into();
    ui.set_conda_envs(ui_env_list);

    ui.on_pick_folder({
        let ui_weak = ui.as_weak();
        move || {
            // Mark UI busy immediately (we're on the UI thread here).
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_picking(true);
            }

            // Spawn an async task *on the Slint event loop thread*.
            // This keeps UI updates safe and avoids blocking.
            let ui_weak2 = ui_weak.clone();
            slint::spawn_local(async move {
                // Show native dialog asynchronously
                let picked = rfd::AsyncFileDialog::new()
                    .set_title("Select a folder")
                    .pick_folder()
                    .await;

                if let Some(ui) = ui_weak2.upgrade() {
                    match picked {
                        Some(file) => {
                            let path: SharedString =
                                file.path().display().to_string().into();
                            ui.set_output_folder(path);
                        }
                        None => {
                            // user canceled; keep existing selection (or clear it if you prefer)
                        }
                    }
                    ui.set_picking(false);
                }
            }).ok();
        }
    });

    ui.on_create_env_file({
        let ui_weak = ui.as_weak();
        move || {
            // if let Some(ui) = ui_weak.upgrade() {
            //     ui.set_picking(true);
            // }
            let ui = ui_weak.unwrap();

            let env_name: String = ui.get_selected_env().into();
            let sharable_conda_env = sharable_env(&env_name).unwrap();
            let folder_path: String = ui.get_output_folder().into();
            let file_name: String = env_name.to_owned() + ".yml";
            let output_path = Path::new(&folder_path).join(file_name);
            sharable_conda_env.save(&output_path).unwrap();
        }
    });

    ui.run()
}
