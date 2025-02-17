use tokio::time::{self, Duration};
use zbus::zvariant;
use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    SignalContext,
};
// use zvariant::DynamicType;

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

#[zbus::interface(name = "org.freedesktop.impl.portal.FileChooser")]
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
