mod components;
mod errors;
mod gui;
mod settings;
mod theme;

use std::collections::HashMap;
use std::time::Duration;

use gui::Homescreen;
use mctk_core::{
    msg,
    reexports::{
        cosmic_text,
        smithay_client_toolkit::{
            reexports::calloop::{self, channel::Sender},
            shell::wlr_layer,
        },
    },
};
use mctk_smithay::WindowMessage;
use mctk_smithay::{layer_surface::LayerOptions, layer_window::LayerWindowParams, WindowOptions};

use settings::HomescreenSettings;
use theme::HomescreenTheme;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::gui::Message;

#[derive(Debug)]
enum AppMessage {}

fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(e) => {
            println!("error while reading settings {:?}", e);

            HomescreenSettings::default()
        }
    };

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => HomescreenTheme::default(),
    };

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let mut assets: HashMap<String, String> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    for app in modules.apps.iter() {
        if let Some(app_icon) = app.icon.clone() {
            if app_icon.ends_with(".png") {
                assets.insert(app.app_id.to_owned(), app.icon.clone().unwrap().to_owned());
            } else if app_icon.ends_with(".svg") {
                svgs.insert(app.app_id.to_owned(), app.icon.clone().unwrap().to_owned());
            }
        }
    }

    let namespace = "mctk.homescreen".to_string();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
        layer: wlr_layer::Layer::Bottom,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };

    let (mut app, mut event_loop, window_tx) =
        mctk_smithay::layer_window::LayerWindow::open_blocking::<Homescreen, AppMessage>(
            LayerWindowParams {
                title: "Homescreen".to_string(),
                namespace,
                window_opts,
                fonts,
                assets,
                layer_shell_opts,
                svgs,
            },
            None,
        );

    let handle = event_loop.handle();

    //subscribe to events channel
    let (channel_tx, channel_rx) = calloop::channel::channel();
    let window_tx_2 = window_tx.clone();
    let _ = handle.insert_source(channel_rx, move |event, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => {
                let _ = window_tx_2
                    .clone()
                    .send(WindowMessage::Send { message: msg!(msg) });
            }
            calloop::channel::Event::Closed => {}
        };
    });

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    //End

    Ok(())
}
