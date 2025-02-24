// use crate::interface::file_chooser::FileChooser;
use anyhow::Error;
use gui::xdg_portal_handler::{XDGPortal, XDGPortalParams};
use interface::file_chooser::FileChooser;
use mctk_core::reexports::cosmic_text;
use mctk_core::AssetParams;
use mctk_smithay::xdg_shell::xdg_window;
use mctk_smithay::{WindowInfo, WindowOptions};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::task::JoinHandle;
use zbus::Connection;
use zbus::{blocking::connection, conn};

pub mod connections;
pub mod gui;
pub mod interface;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let file_chooser_bus = FileChooser {};
    let file_chooser_connection = Connection::session().await?;

    file_chooser_connection
        .request_name("org.mechanix.services.FileChooser")
        .await?;
    file_chooser_connection
        .object_server()
        .at("/org/freedesktop/portal/desktop", file_chooser_bus)
        .await?;

    let mut fonts = cosmic_text::fontdb::Database::new();

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let svgs: HashMap<String, String> = HashMap::new();

    let window_info = WindowInfo {
        id: "xdg-desktop-portal-filechooser".to_string(),
        title: "xdg-desktop-portal".to_string(),
        namespace: "xdg-desktop-portal".to_string(),
    };

    // Set the window options
    let window_opts = WindowOptions {
        height: 480 as u32,
        width: 480 as u32,
        scale_factor: 1.0,
    };

    // assets.insert(
    //     "fold_icon".to_string(),
    //     AssetParams::new(settings.icons.fold_icon),
    // );

    // assets.insert(
    //     "file_icon".to_string(),
    //     AssetParams::new(settings.icons.file_icon),
    // );

    // assets.insert(
    //     "arrow_icon".to_string(),
    //     AssetParams::new(settings.icons.arrow_icon),
    // );

    // assets.insert(
    //     "back_icon".to_string(),
    //     AssetParams::new(settings.icons.back_icon),
    // );

    // assets.insert(
    //     "add_icon".to_string(),
    //     AssetParams::new(settings.icons.add_icon),
    // );

    // assets.insert(
    //     "dots_icon".to_string(),
    //     AssetParams::new(settings.icons.dots_icon),
    // );

    // assets.insert(
    //     "pdf_icon".to_string(),
    //     AssetParams::new(settings.icons.pdf_icon),
    // );

    // assets.insert(
    //     "img_icon".to_string(),
    //     AssetParams::new(settings.icons.img_icon),
    // );

    // assets.insert(
    //     "unfold_dir_icon".to_string(),
    //     AssetParams::new(settings.icons.unfold_dir_icon),
    // );

    let (mut app, mut event_loop, ..) =
        xdg_window::XdgWindow::open_blocking::<XDGPortal, XDGPortalParams>(
            xdg_window::XdgWindowParams {
                window_info,
                window_opts,
                fonts,
                assets,
                svgs,
                ..Default::default()
            },
            XDGPortalParams {},
        );

    loop {
        if app.is_exited {
            break;
        }
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }

    Ok(())
}
