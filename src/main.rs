use std::{collections::HashMap, hash::Hash, ops::Deref};
use zbus::{dbus_proxy, Connection, Result};

#[dbus_proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait PowerManagement {
    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    async fn Suspend(&self, arg: bool) -> Result<()>;
    async fn PowerOff(&self, arg: bool) -> Result<()>;
    async fn Reboot(&self, arg: bool) -> Result<()>;
}
// Locales
#[dbus_proxy(
    interface = "org.freedesktop.locale1",
    default_service = "org.freedesktop.locale1",
    default_path = "/org/freedesktop/locale1"
)]
trait Locales {
    #[dbus_proxy(property)]
    fn x11_layout(&self) -> Result<String>;
}

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

// screen brightness & keyboard brightness
// #[dbus_proxy(
//     interface = "org.gnome.SettingsDaemon.Power.Screen",
//     default_service = "org.gnome.SettingsDaemon.Power.Screen",
//     default_path = "/org/gnome/SettingsDaemon/Power"
// )]
// trait Screen {
//     async fn GetBrightness(&self) -> Result<f64>;
//     async fn SetBrightness(&self, brightness: f64) -> Result<()>;
// }

// Settings color (night light)
#[dbus_proxy(
    interface = "org.freedesktop.DBus.Properties",
    default_service = "org.gnome.SettingsDaemon.Color",
    default_path = "/org/gnome/SettingsDaemon/Color"
)]
trait Nightlight {
    #[dbus_proxy(property)]
    fn NightLightActive(&self) -> Result<bool>;
    #[dbus_proxy(property)]
    fn Temperature(&self) -> Result<u32>;

    async fn SetNightLightActive(&self, active: bool) -> Result<()>;
    async fn SetTemperature(&self, temperature: u32) -> Result<()>;
}

trait Settings {}

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = ShellProxy::new(&connection).await?;
    let (reply) = proxy.ShowApplications().await;
    // let reply=proxy.Suspend(true).await?;
    println!("{:?}", reply);
    //================================
    // let connection = Connection::system().await?;

    // let reply = connection
    //     .call_method(
    //         Some("org.freedesktop.login1"),
    //         "/org/freedesktop/login1",
    //         Some("org.freedesktop.login1.Manager"),
    //         "Suspend",
    //         &(true),
    //     )
    //     .await?;

    // let id: &str = reply.body()?;
    // println!("Unique ID of the bus: {}", id);
    Ok(())
}
