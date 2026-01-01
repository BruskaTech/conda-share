// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use slint::SharedString;

use conda_share_core::*;

slint::include_modules!();

// TODO: (feature) After the file is created, show a message box to inform the user of success or failure or make the button disabled again
// TODO: (feature) Make it so that every time you click the ComboBox it updates what is inside (requires making a custom ComboBox)
// TODO: (feature) Make a smarter error dialog that only shows the first error

impl OverwriteDialog {
    pub fn open(&self, env_name: &str, file_path: &Path) -> Result<(), slint::PlatformError> {
        self.set_env_name(env_name.into());
        self.set_file_path(file_path.to_string_lossy().as_ref().into());
        let file_name = match file_path.file_name() {
            Some(name) => name,
            None => return Err("Missing file name in file path".into()),
        };
        let file_name = match file_name.to_str() {
            Some(name_str) => name_str,
            None => return Err("Failed to convert file name to string".into()),
        };
        self.set_file_name(file_name.into());
        self.show()?;
        Ok(())
    }
}

impl ErrorDialog {
    pub fn show_error(&self, error_msg: &str) {
        self.set_error_msg(error_msg.into());
        self.show().expect(&format!("Critical Error: Failed to show error message: {}", error_msg));
    }
}

fn create_and_save_env(env_name: &str, output_path: &Path, error_dialog: &ErrorDialog) {
    let sharable_conda_env = match sharable_env(env_name) {
        Ok(env) => env,
        Err(e) => {
            error_dialog.show_error(&format!("Failed to generate sharable environment: {e}"));
            return;
        }
    };
    if let Err(e) = sharable_conda_env.save(output_path) {
        error_dialog.show_error(&format!("Failed to save environment file: {e}"));
        return;
    }
}

fn main() -> anyhow::Result<()> {
    let ui = AppWindow::new()?;
    let overwrite_dialog = OverwriteDialog::new()?;
    let error_dialog = ErrorDialog::new()?;

    ui.on_pick_folder({
        let ui_weak = ui.as_weak();
        move || {
            let Some(ui) = ui_weak.upgrade() else { return; };
            ui.set_picking(true);

            // Spawn an async task *on the Slint event loop thread*.
            // This keeps UI updates safe and avoids blocking.
            let ui_weak2 = ui_weak.clone();
            slint::spawn_local(async move {
                // Show native dialog asynchronously
                let picked = rfd::AsyncFileDialog::new()
                    .set_title("Select a folder")
                    .pick_folder()
                    .await;

                // Handle the response
                let Some(ui) = ui_weak2.upgrade() else { return; };
                match picked {
                    Some(file) => {
                        let path: SharedString =
                            file.path().display().to_string().into();
                        ui.set_output_folder(path);
                    }
                    None => {} // user canceled; keep existing selection
                }

                ui.set_picking(false);
            }).ok();
        }
    });

    ui.on_create_env_file({
        let ui_weak = ui.as_weak();
        let od_weak = overwrite_dialog.as_weak();
        let ed_weak = error_dialog.as_weak();
        move || {
            let Some(ui) = ui_weak.upgrade() else { return; };
            let Some(overwrite_dialog) = od_weak.upgrade() else { return; };
            let Some(error_dialog) = ed_weak.upgrade() else { return; };

            let env_name: String = ui.get_selected_env().into();
            let folder_path: String = ui.get_output_folder().into();
            let file_name: String = ui.get_output_file_name().into();
            if file_name.trim().is_empty() {
                error_dialog.show_error("Output file name cannot be empty.");
                return;
            }
            let output_path = Path::new(&folder_path).join(file_name);

            if output_path.exists() {
                if let Err(e) = overwrite_dialog.open(&env_name, &output_path) {
                    error_dialog.show_error(&format!("Failed to open overwrite dialog: {e}"));
                }
                return; // Save will be handled in the overwrite dialog
            }

            create_and_save_env(&env_name, &output_path, &error_dialog);
        }
    });

    overwrite_dialog.on_ok_clicked({
        let od_weak = overwrite_dialog.as_weak();
        let ed_weak = error_dialog.as_weak();
        move || {
            let Some(overwrite_dialog) = od_weak.upgrade() else { return; };
            let Some(error_dialog) = ed_weak.upgrade() else { return; };

            let env_name: String = overwrite_dialog.get_env_name().into();
            let output_path_string: String = overwrite_dialog.get_file_path().into();
            let output_path= Path::new(&output_path_string);

            create_and_save_env(&env_name, &output_path, &error_dialog);
            
            if let Err(e) = overwrite_dialog.hide() {
                error_dialog.show_error(&format!("Failed to hide overwrite dialog: {e}"));
            }
        }
    });

    overwrite_dialog.on_cancel_clicked({
        let od_weak = overwrite_dialog.as_weak();
        let ed_weak = error_dialog.as_weak();
        move || {
            let Some(overwrite_dialog) = od_weak.upgrade() else { return; };
            let Some(error_dialog) = ed_weak.upgrade() else { return; };

            if let Err(e) = overwrite_dialog.hide() {
                error_dialog.show_error(&format!("Failed to hide overwrite dialog: {e}"));
            }
        }
    });

    error_dialog.on_close_clicked({
        let ed_weak = error_dialog.as_weak();
        move || {
            let Some(error_dialog) = ed_weak.upgrade() else { return; };

            error_dialog.hide().expect(&format!("Critical Error: Failed to close error message"));
        }
    });

    let env_list = match conda_env_list() {
        Ok(envs) => envs,
        Err(e) => {
            error_dialog.show_error(&format!("Failed to get conda environment list.\
                \nPlease ensure that conda is installed and accessible from your PATH environment variable.\
                \n{e}"));
            return Ok(error_dialog.run()?);
        }
    };
    
    let ui_env_list = env_list
        .iter().map(|e| e.into()).collect::<Vec<slint::SharedString>>()
        .as_slice().into();
    ui.set_conda_envs(ui_env_list);

    Ok(ui.run()?)
}
