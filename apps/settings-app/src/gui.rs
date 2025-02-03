use crate::{
    screens::{
        about::about_device::AboutDevice,
        battery::battery_screen::{BatteryScreen, BatteryScreenRoute},
        bluetooth::{
            bluetooth_pairing_enter_code::BluetoothPairingEnterCode,
            bluetooth_screen::BluetoothScreen,
        },
        display::display_screen::{DisplayScreen, DisplayScreenRoute},
        network::{
            add_network::AddNetwork, network_details::NetworkDetails,
            network_settings::NetworkSettings, networking::NetworkingScreen,
            saved_network_details::SavedNetworkDetails,
            unknown_network_details::UnknownNetworkDetails,
        },
        settings_menu::settings_screen::SettingsScreen,
        sound::sound_screen::{SoundScreen, SoundScreenRoute},
    },
    settings::{self, MainSettings},
    AppMessage, AppParams,
};
use mctk_core::{
    component::{self, Component, RootComponent},
    lay,
    layout::{Alignment, Direction},
    node, rect,
    reexports::smithay_client_toolkit::reexports::calloop,
    size_pct,
    widgets::{Div, HDivider},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

#[derive(Default, Debug, Clone)]
pub enum NetworkScreenRoutes {
    #[default]
    Networking,
    UnknownNetworkDetails {
        mac: String,
    },
    AddNetwork {
        ssid: String,
    },
    NetworkSettings,
    SavedNetworkDetails {
        mac: String,
    },
    NetworkDetails,
}

#[derive(Default, Debug, Clone)]
pub enum Routes {
    #[default]
    SettingsList,
    Network {
        screen: NetworkScreenRoutes,
    },
    BluetoothScreen,
    BluetoothPairingVerifyCode,
    BluetoothPairingEnterCode,
    BluetoothDeviceInfo,
    DisplayScreen,
    AppearanceScreen,
    BatteryScreen,
    SoundScreen,
    AboutScreen,
    LockScreen,
    LanguageScreen,
    LanguageSelect,
}

#[derive(Debug)]
pub struct SettingsAppState {
    settings: Arc<RwLock<MainSettings>>,
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
    current_route: Routes,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeRoute { route: Routes },
    ChangeSoundScreenRoute { route: SoundScreenRoute },
    ChangeDisplayScreenRoute { route: DisplayScreenRoute },
    ChangeBatteryScreenRoute { route: BatteryScreenRoute },
}

/// # SettingsApp State
///
/// This struct is the state definition of the entire application
#[component(State = "SettingsAppState")]
#[derive(Debug, Default)]
pub struct SettingsApp {}

#[state_component_impl(SettingsAppState)]
impl Component for SettingsApp {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => MainSettings::default(),
        };

        self.state = Some(SettingsAppState {
            settings: Arc::new(RwLock::new(MainSettings::default())),
            app_channel: None,
            current_route: Routes::default(),
        });
    }

    fn view(&self) -> Option<Node> {
        let mut app_node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );
        // app_node = app_node.push(node!(
        //     HDivider {
        //         size: 1.,
        //         color: Color::rgba(83., 83., 83., 1.)
        //     },
        //     lay![
        //         padding: [2.0, 20.0, 10.0, 20.0],
        //     ],
        // ));
        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                padding: [0., 20., 0., 20.],
            ]
        );

        match &self.state_ref().current_route {
            Routes::SettingsList => base = base.push(node!(SettingsScreen::new())),
            Routes::SoundScreen => base = base.push(node!(SoundScreen::new())),
            Routes::Network { screen } => match screen {
                NetworkScreenRoutes::Networking => base = base.push(node!(NetworkingScreen::new())),
                NetworkScreenRoutes::AddNetwork { ssid } => {
                    base = base.push(node!(AddNetwork::new(ssid.to_string())))
                }
                NetworkScreenRoutes::NetworkSettings => {
                    base = base.push(node!(NetworkSettings::new()))
                }
                NetworkScreenRoutes::NetworkDetails => {
                    base = base.push(node!(NetworkDetails::new()))
                }
                NetworkScreenRoutes::UnknownNetworkDetails { mac } => {
                    base = base.push(node!(UnknownNetworkDetails::new(mac.to_string())))
                }
                NetworkScreenRoutes::SavedNetworkDetails { mac } => {
                    base = base.push(node!(SavedNetworkDetails::new(mac.to_string())))
                }
            },
            Routes::DisplayScreen => base = base.push(node!(DisplayScreen::new())),
            Routes::BatteryScreen => base = base.push(node!(BatteryScreen::new())),
            Routes::AboutScreen => base = base.push(node!(AboutDevice {})),
            Routes::BluetoothScreen => base = base.push(node!(BluetoothScreen {})),
            Routes::BluetoothPairingEnterCode => {
                base = base.push(node!(BluetoothPairingEnterCode {}))
            }
            _ => (),
        }

        app_node = app_node.push(base);
        Some(app_node)
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::ChangeRoute { route } => match route {
                    Routes::SettingsList => {
                        self.state_mut().current_route = route.clone();
                    }
                    Routes::Network { .. } => {
                        self.state_mut().current_route = route.clone();
                    }
                    _ => {
                        self.state_mut().current_route = route.clone();
                    }
                },
                _ => (),
            }
        }

        vec![]
    }
}
impl RootComponent<AppParams> for SettingsApp {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
        self.state_mut().settings = app_params.settings.clone();
    }
}
