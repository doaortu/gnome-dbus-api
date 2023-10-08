#![feature(ascii_char)]

pub mod easy_gnome {
    use std::collections::HashMap;
    use zbus::{dbus_proxy, Result};

    #[dbus_proxy(
        interface = "org.freedesktop.login1.Manager",
        default_service = "org.freedesktop.login1",
        default_path = "/org/freedesktop/login1"
    )]
    trait PowerManagement {
        async fn Suspend(&self, arg: bool) -> Result<()>;
        async fn PowerOff(&self, arg: bool) -> Result<()>;
        async fn Reboot(&self, arg: bool) -> Result<()>;
    }
    // Locales
    // #[dbus_proxy(
    //     interface = "org.freedesktop.locale1",
    //     default_service = "org.freedesktop.locale1",
    //     default_path = "/org/freedesktop/locale1"
    // )]
    // trait Locales {
    //     #[dbus_proxy(property)]
    //     fn x11_layout(&self) -> Result<String>;
    // }

    // Shell extensions
    #[dbus_proxy(
        interface = "org.gnome.Shell.Extensions",
        default_service = "org.gnome.Shell.Extensions",
        default_path = "/org/gnome/Shell/Extensions"
    )]
    trait Extensions {
        async fn ListExtensions(
            &self,
        ) -> Result<HashMap<String, HashMap<String, zvariant::OwnedValue>>>;
        async fn LaunchExtensionPrefs(&self, uuid: String) -> Result<()>;
        async fn EnableExtension(&self, uuid: String) -> Result<bool>;
        async fn DisableExtension(&self, uuid: String) -> Result<bool>;
        async fn UninstallExtension(&self, uuid: String) -> Result<bool>;
    }

    /// # Extension states
    /// ```xml
    /// <member>1: ENABLED</member>
    /// <member>2: DISABLED</member>
    /// <member>3: ERROR</member>
    /// <member>4: OUT_OF_DATE</member>
    /// <member>5: DOWNLOADING</member>
    /// <member>6: INITIALIZED</member>
    /// <member>99: UNINSTALLED</member>
    /// ```
    /// https://gitlab.gnome.org/GNOME/gnome-shell/-/blob/92d3c6e051958b31151bf9538205a71cab6f70d7/data/dbus-interfaces/org.gnome.Shell.Extensions.xml#L73
    #[derive(Debug)]
    enum ListExtensionState {
        ENABLED = 1,
        DISABLED = 2,
        ERROR = 3,
        OUT_OF_DATE = 4,
        DOWNLOADING = 5,
        INITIALIZED = 6,
        UNINSTALLED = 99,
    }
    #[derive(Debug)]
    struct ListExtension {
        uuid: String,
        name: String,
        description: String,
        state: ListExtensionState,
        version: String,
        url: String,
    }
    impl ExtensionsProxy<'static> {
        async fn launch_extension_prefs(&self, uuid: &str) -> Result<()> {
            let _reply = self
                .LaunchExtensionPrefs(uuid.to_string())
                .await
                .unwrap_or_else(|_| ());
            Ok(())
        }
        async fn list_extensions(&self) -> Vec<ListExtension> {
            let list = self.ListExtensions().await.unwrap();
            let mut list_extension: Vec<ListExtension> = Vec::new();
            for extension in list {
                let uuid = extension.0;
                let name = extension
                    .1
                    .get("name")
                    .unwrap()
                    .to_owned()
                    .try_into()
                    .unwrap();
                let description = extension
                    .1
                    .get("description")
                    .unwrap()
                    .to_owned()
                    .try_into()
                    .unwrap();
                let try_version = extension.1.get("version");
                let version = match try_version {
                    Some(version) => version
                        .to_owned()
                        .try_into()
                        .unwrap_or_else(|_| "".to_string()),
                    None => "".to_string(),
                };
                let state_number: f64 = extension
                    .1
                    .get("state")
                    .unwrap()
                    .to_owned()
                    .try_into()
                    .unwrap();
                let state: ListExtensionState = match state_number {
                    1.0 => ListExtensionState::ENABLED,
                    2.0 => ListExtensionState::DISABLED,
                    3.0 => ListExtensionState::ERROR,
                    4.0 => ListExtensionState::OUT_OF_DATE,
                    5.0 => ListExtensionState::DOWNLOADING,
                    6.0 => ListExtensionState::INITIALIZED,
                    99.0 => ListExtensionState::UNINSTALLED,
                    _ => ListExtensionState::UNINSTALLED,
                };
                let url = extension
                    .1
                    .get("url")
                    .unwrap()
                    .to_owned()
                    .try_into()
                    .unwrap();
                let item = ListExtension {
                    uuid,
                    name,
                    description,
                    version,
                    state,
                    url,
                };
                list_extension.push(item);
            }
            list_extension
        }
    }
    // Shell screenshot
    #[dbus_proxy(
        interface = "org.gnome.Shell.Screenshot",
        default_path = "/org/gnome/Shell/Screenshot"
    )]
    trait Screenshot {
        // Output: ({'color': <(0.20784313725490197, 0.51764705882352946, 0.89411764705882357)>},)
        async fn PickColor(&self) -> Result<HashMap<String, zvariant::OwnedValue>>;
    }
    impl ScreenshotProxy<'static> {
        async fn pick_color(&self) -> (f64, f64, f64) {
            let pick_color = self.PickColor().await.unwrap();
            let value = pick_color.get("color").unwrap();
            let (r, g, b): (f64, f64, f64) = value.to_owned().try_into().unwrap();

            (r, g, b)
        }
    }

    #[dbus_proxy(
        interface = "org.gnome.SettingsDaemon.Power.Screen",
        default_service = "org.gnome.SettingsDaemon.Power",
        default_path = "/org/gnome/SettingsDaemon/Power"
    )]
    trait Screen {
        #[dbus_proxy(property)]
        fn Brightness(&self) -> Result<i32>;
        #[dbus_proxy(property)]
        fn set_Brightness(&self, brightness: i32) -> Result<()>;
        fn StepUp(&self) -> Result<()>;
        fn StepDown(&self) -> Result<()>;
    }

    // Settings color (night light)
    #[dbus_proxy(
        interface = "org.gnome.SettingsDaemon.Color",
        default_service = "org.gnome.SettingsDaemon.Color",
        default_path = "/org/gnome/SettingsDaemon/Color"
    )]
    trait Nightlight {
        #[dbus_proxy(property)]
        fn set_NightLightActive(&self, active: bool) -> Result<()>;
        #[dbus_proxy(property)]
        fn NightLightActive(&self) -> Result<bool>;
        #[dbus_proxy(property)]
        fn Temperature(&self) -> Result<u32>;

        async fn SetTemperature(&self, temperature: u32) -> Result<()>;
    }

    trait Settings {}

    pub mod power {
        use zbus::Connection;

        use crate::handlers::easy_gnome::PowerManagementProxy;

        pub async fn power_off() {
            let connection = Connection::system().await.unwrap();
            let proxy = PowerManagementProxy::new(&connection).await.unwrap();
            proxy.PowerOff(true).await.unwrap();
        }
        pub async fn suspend() {
            let connection = Connection::system().await.unwrap();
            let proxy = PowerManagementProxy::new(&connection).await.unwrap();
            proxy.Suspend(true).await.unwrap();
        }
        pub async fn reboot() {
            let connection = Connection::system().await.unwrap();
            let proxy = PowerManagementProxy::new(&connection).await.unwrap();
            proxy.Reboot(true).await.unwrap();
        }
    }

    pub mod screenshot {
        use zbus::Connection;

        use crate::handlers::easy_gnome::ScreenshotProxy;

        pub async fn pick_color() -> (f64, f64, f64) {
            let connection = Connection::session().await.unwrap();
            let proxy = ScreenshotProxy::new(&connection).await.unwrap();
            proxy.pick_color().await
        }
    }

    pub mod screen {
        use zbus::Connection;

        use crate::handlers::easy_gnome::ScreenProxy;

        pub async fn brightness() -> i32 {
            let connection = Connection::session().await.unwrap();
            let proxy = ScreenProxy::new(&connection).await.unwrap();
            proxy.Brightness().await.unwrap()
        }
        pub async fn set_brightness(brightness: i32) {
            let connection = Connection::session().await.unwrap();
            let proxy = ScreenProxy::new(&connection).await.unwrap();
            proxy.set_Brightness(brightness).await.unwrap();
        }
        pub async fn step_up() {
            let connection = Connection::session().await.unwrap();
            let proxy = ScreenProxy::new(&connection).await.unwrap();
            proxy.StepUp().await.unwrap();
        }
        pub async fn step_down() {
            let connection = Connection::session().await.unwrap();
            let proxy = ScreenProxy::new(&connection).await.unwrap();
            proxy.StepDown().await.unwrap();
        }
    }

    pub mod nightlight {
        use std::process::Command;

        use zbus::Connection;

        use crate::handlers::easy_gnome::NightlightProxy;

        pub async fn nightlight_active() -> bool {
            let connection = Connection::session().await.unwrap();
            let proxy = NightlightProxy::new(&connection).await.unwrap();
            proxy.NightLightActive().await.unwrap()
        }
        pub async fn set_nightlight_active(active: bool) {
            Command::new("gsettings")
                .arg("set")
                .arg("org.gnome.settings-daemon.plugins.color")
                .arg("night-light-enabled")
                .arg(active.to_string())
                .spawn()
                .expect("failed to execute process");
        }
        pub async fn temperature() -> u32 {
            let connection = Connection::session().await.unwrap();
            let proxy = NightlightProxy::new(&connection).await.unwrap();
            proxy.Temperature().await.unwrap()
        }
        pub async fn set_temperature(temperature: u32) {
            let connection = Connection::session().await.unwrap();
            let proxy = NightlightProxy::new(&connection).await.unwrap();
            proxy.SetTemperature(temperature).await.unwrap();
        }
    }

    pub mod apps {

        use std::io::Cursor;

        use gio::glib::{home_dir, GString};
        use gio::prelude::*;
        use gio::AppInfo;
        use gtk::IconTheme;
        use gtk::{prelude::*, IconLookupFlags};
        use image::ImageOutputFormat;

        // #[derive(Debug)]
        pub struct App {
            pub name: GString,
            pub description: Option<GString>,
            pub icon: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
            // pub launch: &'static dyn Fn() -> Result<(), Box<dyn std::error::Error>>,
        }
        impl App {
            pub fn get_name(&self) -> &GString {
                &self.name
            }
            pub fn get_description(&self) -> &Option<GString> {
                &self.description
            }
            pub fn get_icon(&self) -> &Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
                &self.icon
            }
            pub fn get_base64_icon(&self) -> Option<String> {
                match &self.icon {
                    Some(icon) => {
                        let mut image_data: Vec<u8> = Vec::new();
                        icon.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
                            .unwrap();
                        let res_base64 = base64::encode(image_data);
                        Some(format!("data:image/png;base64,{}", res_base64))
                    }
                    None => None,
                }
            }
            pub fn launch(&self) -> Result<(), gio::glib::Error> {
                // Find app by name
                let __apps = AppInfo::all();
                __apps
                    .iter()
                    .find(|app| app.name().eq_ignore_ascii_case(&self.name))
                    .unwrap()
                    .launch(&[], None::<&gio::AppLaunchContext>)
            }
        }

        // #[derive(Debug)]
        pub struct Apps {
            pub apps: Vec<App>,
        }

        impl Apps {
            // pub fn set_launch_callback(&self, launch: Box<dyn Fn() -> Result<(), Box<dyn Error>>>) {
            //     self.launch = launch;
            // }
            pub fn get_apps(&self) -> &Vec<App> {
                &self.apps
            }

            pub fn new() -> Apps {
                gtk::init().unwrap();
                const ICON_SIZE: i32 = 128;

                let __apps = AppInfo::all();
                let icon_theme: IconTheme = IconTheme::default().unwrap();
                icon_theme.add_resource_path(
                    format!(
                        "{}/.local/share/icons/hicolor",
                        home_dir().to_str().unwrap()
                    )
                    .as_str(),
                );
                let mut apps: Vec<App> = Vec::new();

                for app in &__apps {
                    let name = app.name();
                    let description = app.description();
                    let icon = app.icon();
                    // let launch: &'static (dyn Fn() -> Result<(), ()> + 'static) = || {
                    //     let launch_context = gio::AppLaunchContext::NONE;
                    //     app.launch_uris(&[], launch_context)?;
                    //     Ok(())
                    // };

                    if icon.is_none() {
                        apps.push(App {
                            name,
                            description,
                            icon: None,
                            // launch: &launch,
                        });
                        continue;
                    }
                    let icon_name = gio::prelude::IconExt::to_string(&icon.unwrap()).unwrap();
                    // // Transform icon name to pixbuf
                    let pixbuf = icon_theme
                        .load_icon(&icon_name, ICON_SIZE, IconLookupFlags::GENERIC_FALLBACK)
                        .unwrap_or(
                            icon_theme
                                .load_icon("info", ICON_SIZE, IconLookupFlags::GENERIC_FALLBACK)
                                .unwrap(),
                        );

                    // Pix buf are cuadruplets of u8 (rgba)
                    let bytes: Vec<u8> = pixbuf.unwrap().read_pixel_bytes().unwrap().to_vec();

                    // Using image library build a png based on cuadruplets (rgba)
                    let png: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                        match image::RgbaImage::from_vec(ICON_SIZE as u32, ICON_SIZE as u32, bytes)
                        {
                            Some(png) => png,
                            None => continue,
                        };

                    apps.push(App {
                        name,
                        description,
                        icon: Some(png),
                        // launch: &launch,
                    });
                }
                Apps { apps }
            }
        }
    }
}
