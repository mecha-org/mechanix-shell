use fs_extra::dir::CopyOptions;
use mctk_core::component::{self, Component, RootComponent};
use mctk_core::layout::{Alignment, Dimension, Direction, Size};
use mctk_core::node;
use mctk_core::style::FontWeight;
use mctk_core::style::Styled;
use mctk_core::widgets::{Div, IconButton, IconType, Image, Text};
use mctk_core::widgets::{HDivider, Scrollable};
use mctk_core::{event, Node};
use mctk_core::{lay, msg, rect, size, size_pct, txt, Color};
use mctk_macros::{component, state_component_impl};
use std::fs;
use std::hash::Hash;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
struct XDGPortalPOCParams {}

#[component(State = "XDGPortalPOCState")]
#[derive(Debug, Default)]
struct XDGPortalPOC {}

#[derive(Debug, Clone)]
enum Message {
    PortalRequestGot,
}

struct XDGPortalPOCState {}

#[state_component_impl(XDGPortalPOCState)]
impl Component for XDGPortalPOC {
    fn init(&mut self) {}

    fn update(&mut self, msg: component::Message) -> Vec<component::Message> {}

    // fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
    //     self.state_ref().is_delete_modal_open.hash(hasher);
    //     self.state_ref().is_folder_options_modal_open.hash(hasher);
    //     self.state_ref().is_create_rename_modal_open.hash(hasher);
    //     self.state_ref().is_delete_modal_open.hash(hasher);
    //     self.state_ref().is_file_action_modal_open.hash(hasher);
    // }

    fn view(&self) -> Option<mctk_core::Node> {
        None
    }
}
