// use crate::interface::file_chooser::FileChooser;
use anyhow::Error;
use interface::file_chooser::FileChooser;
use mctk_core::AssetParams;
use std::collections::HashMap;
use tokio::task::JoinHandle;
use zbus::Connection;
use zbus::{blocking::connection, conn};
pub mod connections;
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

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let svgs: HashMap<String, String> = HashMap::new();

    loop {
        std::future::pending::<()>().await;
    }

    Ok(())
}
