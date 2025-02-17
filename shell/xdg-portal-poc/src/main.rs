// use crate::interface::file_chooser::FileChooser;
use anyhow::Error;
use interface::file_chooser::FileChooser;
use tokio::task::JoinHandle;
use zbus::Connection;
use zbus::{blocking::connection, conn};
pub mod interface;
#[tokio::main]

async fn main() -> Result<(), anyhow::Error> {
    let file_chooser_bus = FileChooser {};
    // let file_chooser_bus_connection = Connection::session().await?;
    // file_chooser_bus_connection
    //     .object_server()
    //     .at("/org/mechanix/services/FileChooser", file_chooser_bus)
    //     .await?;

    // file_chooser_bus_connection.request_name("org.mechanix.services.FileChooser");

    // let _file_chooser_connection = connection::Builder::system()?
    //     .name("org.mechanix.services.FileChooser")?
    //     .serve_at("/org/mechanix/services/FileChooser", file_chooser_bus)?
    //     .build();
    // .await?;

    let file_chooser_connection = Connection::session().await?;

    file_chooser_connection
        .request_name("org.mechanix.services.FileChooser")
        .await?;
    file_chooser_connection
        .object_server()
        .at("/org/mechanix/services/FileChooser", file_chooser_bus)
        .await?;

    loop {
        std::future::pending::<()>().await;
    }

    Ok(())

    // let connection = Connection::session().await?;
    // connection
    //     .request_name("org.mechanix.services.FileChooser")
    //     .await?;

    // loop {}
}
