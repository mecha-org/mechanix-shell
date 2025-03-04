use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use mechanix_system_dbus_client::display::{Display, NotificationStream};
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref BATTERY_MODEL: BrightnessModel = BrightnessModel {
        brightness_percentage: Context::new(5. as u8)
    };
}

#[derive(Model)]
pub struct BrightnessModel {
    pub brightness_percentage: Context<u8>,
}

impl BrightnessModel {
    pub fn get() -> &'static Self {
        &BATTERY_MODEL
    }

    pub fn set_brightness(value: u8) {
        RUNTIME.spawn(async move {
            let result =
                Display::set_brightness_percentage((value as f32 / 100. * 254.).max(5.) as u8)
                    .await;

            match result {
                Ok(v) => {
                    BrightnessModel::update();
                    v
                }
                Err(e) => {
                    eprintln!("BrightnessModel::error while setting brightness {}", e)
                }
            };
        });
    }

    pub fn update() {
        RUNTIME.spawn(async {
            match Display::get_brightness_percentage().await {
                Ok(value) => {
                    BrightnessModel::get()
                        .brightness_percentage
                        .set(((value as f32 / 254. * 100.) as u8).into());
                }
                Err(e) => {
                    eprintln!(
                        "BrightnessModel::error while getting brightness value {}",
                        e
                    );
                }
            };

            // let mut stream_res = BrightnessService::get_notification_stream().await;
            // if let Err(e) = stream_res.as_ref() {
            //     error!(task, "error while getting brightness stream {}", e);
            //     BrightnessModel::get().brightness_percentage.set(0.0);
            //     return;
            // }
            // loop {
            //     select! {
            //         signal = stream_res.as_mut().unwrap().next() => {
            //             if signal.is_none() {
            //                 continue;
            //             }
            //
            //             if let Ok(args) = signal.unwrap().args() {
            //                 let event = args.event;
            //                 BrightnessModel::get().brightness_percentage.set(event.brightness_percentage);
            //             }
            //
            //         }
            //     }
            // }
        });
    }
}
