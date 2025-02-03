mod proxies;

pub mod bluetooth {
    use crate::proxies;
    pub use proxies::bluetooth_proxy::{BluetoothService, NotificationStream};
}

pub mod display {
    use crate::proxies;
    pub use proxies::display_proxy::{Display, NotificationStream};
}

pub mod hardware_buttons {
    use crate::proxies;
    pub use mechanix_system_dbus_server::{Key, KeyEvent};
    pub use proxies::hardware_button::{HwButton, NotificationStream};
}

pub mod security {
    use crate::proxies;
    pub use proxies::security_proxy::Security;
}
