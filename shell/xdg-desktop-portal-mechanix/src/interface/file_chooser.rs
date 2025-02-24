use crate::connections::PortalResponse;
use tokio::time::{self, Duration};
use zbus::zvariant;
use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    SignalContext,
};
// use zvariant::DynamicType;
type Filter = (String, Vec<(u32, String)>);

#[derive(Clone, Copy)]
pub struct FileChooser {}

#[derive(zvariant::SerializeDict, zvariant::Type)]
#[zvariant(signature = "a{sv}")]
pub struct FileChooserResult {
    uris: Vec<String>,
    choices: Vec<(String, String)>,
    current_filter: Option<Filter>,
}
#[derive(Clone, Debug)]
pub enum FileChooserOptions {
    OpenFile(OpenFileOptions),
    SaveFile(SaveFileOptions),
    SaveFiles(SaveFilesOptions),
}

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
    ) -> PortalResponse<FileChooserResult> {
        dbg!("open file has been called");
        println!("Open File has been called");

        self.run(
            handle,
            app_id,
            parent_window,
            title,
            FileChooserOptions::OpenFile(options),
        )
        .await
    }

    async fn save_file(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: SaveFileOptions,
    ) -> PortalResponse<FileChooserResult> {
        dbg!("save file has been called");
        println!("Save File has been called");
        self.run(
            handle,
            app_id,
            parent_window,
            title,
            FileChooserOptions::SaveFile(options),
        )
        .await
    }

    async fn save_files(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: SaveFilesOptions,
    ) -> PortalResponse<FileChooserResult> {
        dbg!("save files has been called");
        println!("Save Files has been called");
        self.run(
            handle,
            app_id,
            parent_window,
            title,
            FileChooserOptions::SaveFiles(options),
        )
        .await
    }
}

impl FileChooser {
    async fn run(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        options: FileChooserOptions,
    ) -> PortalResponse<FileChooserResult> {
        dbg!("file chooser {handle}, {app_id}, {parent_window}, {title}, {options:?}");

        //send msg! regarding what to call
        //need to write logic for that

        // let msging: Result<(), ()> = Err(()); //this should be replaced by code that sends a msg![]

        match options {
            FileChooserOptions::OpenFile(_) => {
                let selected_files = FileChooserResult {
                    uris: vec!["file:///home/vrn21/p.json".to_string()],
                    choices: vec![],
                    current_filter: None,
                };

                PortalResponse::Success(selected_files)
            }
            FileChooserOptions::SaveFiles(_) => {
                todo!()
            }
            FileChooserOptions::SaveFile(_) => {
                todo!()
            }
        }
    }
}
