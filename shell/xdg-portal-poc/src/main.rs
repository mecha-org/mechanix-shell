// use crate::interface::file_chooser::FileChooser;
use anyhow::Error;
use tokio::task::JoinHandle;
use zbus::Connection;
use zbus::{blocking::connection, conn};

pub mod interface;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let file_chooser_bus = FileChooser {};

    let _file_chooser_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.FileChooser")?
        .serve_at("/org/mechanix/services/FileChooser", file_chooser_bus)?
        .build();

    Ok(())

    // let connection = Connection::session().await?;
    // connection
    //     .request_name("org.mechanix.services.FileChooser")
    //     .await?;

    // loop {}
}
