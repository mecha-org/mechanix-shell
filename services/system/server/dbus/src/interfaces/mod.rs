mod bluetooth_interface;
pub use bluetooth_interface::{
    bluetooth_event_notification_stream, BluetoothBusInterface, BluetoothNotificationEvent,
};

mod display_interface;
pub use display_interface::DisplayBusInterface;

mod hardware_buttons;
pub use hardware_buttons::{hw_buttons_notification_stream, HwButtonInterface};

mod security_interface;
pub use security_interface::SecurityBusInterface;
