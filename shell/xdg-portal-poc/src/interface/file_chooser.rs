use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    SignalContext,
};

use tokio::time::{self, Duration};
use zvariant::DynamicType;

#[derive(Clone, Copy)]
pub struct FileChooser {}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct OpenFileOptions {}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct SaveFileOptions {}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "a{sv}")]
pub struct SaveFilesOptions {}

// Fix: Removed invalid syntax and ensured correct struct name
#[zbus::interface(name = "org.mechanix.services.FileChooser")] // (/* &zvariant::ObjectPath<'_> */)
impl FileChooser {
    async fn open_file(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: OpenFileOptions,
    ) -> Result<(), ZbusError> {
        println!("Open File has been called");
        Ok(())
    }

    async fn save_file(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: SaveFileOptions,
    ) -> Result<(), ZbusError> {
        println!("Save File has been called");
        Ok(())
    }

    async fn save_files(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: SaveFilesOptions,
    ) -> Result<(), ZbusError> {
        println!("Save Files has been called");
        Ok(())
    }
}
