use crate::handlers::easy_gnome;
use crate::handlers::easy_gnome::battery;
use crate::handlers::easy_gnome::extensions;
use crate::handlers::easy_gnome::interface;
use crate::handlers::easy_gnome::nightlight;
use crate::handlers::easy_gnome::peripherals;
use crate::handlers::easy_gnome::power;
use crate::handlers::easy_gnome::screen;
use crate::handlers::easy_gnome::screenshot;
#[test]
fn get_all_apps() {
    gtk::init().unwrap();
    let apps = easy_gnome::apps::Apps::new();
    println!("{:?}", apps.get_apps().len());
    assert!(apps.get_apps().len() > 0);
}
async fn pick_color() {
    let (r, g, b) = screenshot::pick_color().await;
}
#[tokio::test]
async fn set_power_profile() {
    let power_profile = easy_gnome::PowerProfile::PowerSaver;
    power::set_power_profile(power_profile).await;
    assert_eq!(power::get_power_profile().await, power_profile);
}

#[tokio::test]
async fn get_extensions() {
    let extensions = extensions::get_extensions().await;
    assert!(extensions.len() > 0);
    println!("{:?}", extensions);
}
#[tokio::test]
async fn launch_extension_preferences() {
    let _extensions_list = extensions::get_extensions().await;
    // You can get the extension uuid from the extensions::get_extensions() function
    let extension_uuid = "ubuntu-appindicators@ubuntu.com";
    extensions::open_extension_preferences(extension_uuid).await
}
#[tokio::test]
async fn disable_extension() {
    let _extensions_list = extensions::get_extensions().await;
    // You can get the extension uuid from the extensions::get_extensions() function
    let extension_uuid = "extension-list@tu.berry";
    extensions::disable_extension(extension_uuid).await
}
#[tokio::test]
async fn enable_extension() {
    let _extensions_list = extensions::get_extensions().await;
    // You can get the extension uuid from the extensions::get_extensions() function
    let extension_uuid = "extension-list@tu.berry";
    extensions::enable_extension(extension_uuid).await
}
#[tokio::test]
async fn uninstall_extension() {
    let _extensions_list = extensions::get_extensions().await;
    // You can get the extension uuid from the extensions::get_extensions() function
    let extension_uuid = "extension-list@tu.berry";
    extensions::uninstall_extension(extension_uuid).await
}

#[test]
fn set_keyboard_press_delay() {
    let delay = 100;
    peripherals::set_keyboard_press_delay(delay).unwrap();
    assert_eq!(peripherals::get_keyboard_press_delay().unwrap(), delay);
}
#[test]
fn reset_keyboard_press_delay() {
    let default_delay = 500;
    peripherals::reset_keyboard_press_delay().unwrap();
    assert_eq!(
        peripherals::get_keyboard_press_delay().unwrap(),
        default_delay
    );
}
#[test]
fn set_keyboard_repeat_interval() {
    let interval = 100;
    peripherals::set_keyboard_repeat_interval(interval).unwrap();
    assert_eq!(
        peripherals::get_keyboard_repeat_interval().unwrap(),
        interval
    );
}
#[test]
fn reset_keyboard_repeat_interval() {
    let default_interval = 30;
    peripherals::reset_keyboard_repeat_interval().unwrap();
    assert_eq!(
        peripherals::get_keyboard_repeat_interval().unwrap(),
        default_interval
    );
}
#[tokio::test]
async fn get_battery_display() {
    let battery_display = battery::get_current_device_battery().await.unwrap();
    let full_design_battery = battery_display.energy_full_design().await.unwrap();
    let full_battery = battery_display.energy_full().await.unwrap();
    let current_battery = battery_display.energy().await.unwrap();
    let percentage = battery_display.percentage().await.unwrap();
    let battery_state = battery_display.state().await.unwrap();
    let temperature = battery_display.temperature().await.unwrap();
    let is_rechargable = battery_display.is_rechargeable().await.unwrap();
    let model = battery_display.model().await.unwrap();
    let vendor = battery_display.vendor().await.unwrap();
    let power_supply = battery_display.power_supply().await.unwrap();
    let battery_type = battery_display.type_().await.unwrap();

    assert!(full_design_battery > 0.0);
    assert!(full_battery > 0.0);
    assert!(current_battery > 0.0);
    assert!(percentage > 0.0);
    assert!(is_rechargable);
    assert!(power_supply);
}
#[test]
fn set_show_battery_percentage() {
    let show_percentage = true;
    interface::set_show_battery_percentage(show_percentage).unwrap();
    assert_eq!(
        interface::get_show_battery_percentage().unwrap(),
        show_percentage
    );
}
#[test]
fn reset_show_battery_percentage() {
    let show_percentage = false;
    interface::reset_show_battery_percentage().unwrap();
    assert_eq!(
        interface::get_show_battery_percentage().unwrap(),
        show_percentage
    );
}
#[test]
fn set_locate_pointer() {
    let locate_pointer = true;
    interface::set_locate_pointer(locate_pointer).unwrap();
    assert_eq!(interface::get_locate_pointer().unwrap(), locate_pointer);
}
#[test]
fn reset_locate_pointer() {
    let locate_pointer = false;
    interface::reset_locate_pointer().unwrap();
    assert_eq!(interface::get_locate_pointer().unwrap(), locate_pointer);
}

#[test]
fn set_cursor_size() {
    let cursor_size = 50;
    interface::set_cursor_size(cursor_size).unwrap();
    assert_eq!(interface::get_cursor_size().unwrap(), cursor_size);
}
#[test]
fn reset_cursor_size() {
    let cursor_size = 24;
    interface::reset_cursor_size().unwrap();
    assert_eq!(interface::get_cursor_size().unwrap(), cursor_size);
}
#[tokio::test]
async fn get_devices_battery() {
    let battery_devices = battery::get_devices_battery().await.unwrap();
    for device in battery_devices {
        let full_design_battery = device.energy_full_design().await.unwrap();
        let full_battery = device.energy_full().await.unwrap();
        let current_battery = device.energy().await.unwrap();
        let percentage = device.percentage().await.unwrap();
        let battery_state = device.state().await.unwrap();
        let temperature = device.temperature().await.unwrap();
        let is_rechargable = device.is_rechargeable().await.unwrap();
        let model = device.model().await.unwrap();
        let vendor = device.vendor().await.unwrap();
        let power_supply = device.power_supply().await.unwrap();
        let battery_type = device.type_().await.unwrap();

        println!(
            "full_design_battery: {} full_battery: {} current_battery: {} percentage: {} battery_state: {:?} temperature: {} is_rechargable: {} model: {} vendor: {} power_supply: {} battery_type: {:?}",
            full_design_battery, full_battery, current_battery, percentage, battery_state, temperature, is_rechargable, model, vendor, power_supply,battery_type
        );
    }
}
#[test]
fn set_mouse_natural_scroll() {
    peripherals::set_mouse_natural_scroll(true).unwrap();
    assert_eq!(peripherals::get_mouse_natural_scroll().unwrap(), true);
}
#[test]
fn reset_mouse_natural_scroll() {
    peripherals::reset_mouse_natural_scroll().unwrap();
    assert_eq!(peripherals::get_mouse_natural_scroll().unwrap(), false);
}
#[test]
fn set_touchpad_tap_to_click() {
    peripherals::set_touchpad_tap_to_click(false).unwrap();
    assert_eq!(peripherals::get_touchpad_tap_to_click().unwrap(), false);
}
#[test]
fn reset_touchpad_tap_to_click() {
    peripherals::reset_touchpad_tap_to_click().unwrap();
    assert_eq!(peripherals::get_touchpad_tap_to_click().unwrap(), false);
}
#[test]
fn set_two_finger_scroll() {
    peripherals::set_two_finger_scroll(false).unwrap();
    assert_eq!(peripherals::get_two_finger_scroll().unwrap(), false);
}
#[test]
fn reset_two_finger_scroll() {
    peripherals::reset_two_finger_scroll().unwrap();
    assert_eq!(peripherals::get_two_finger_scroll().unwrap(), true);
}

async fn power_off() {
    power::power_off().await;
}
async fn reboot() {
    power::reboot().await;
}
async fn suspend() {
    power::suspend().await;
}

#[test]
fn get_temperature() {
    let temperature: u32 = nightlight::get_temperature();
    println!("temperature: {}", temperature);
    assert!(temperature > 0);
}
#[test]
fn set_temperature() {
    let temperature: u32 = 3000;
    nightlight::set_temperature(temperature);
    assert_eq!(nightlight::get_temperature(), temperature);
}
#[test]

fn reset_temperature() {
    let temperature: u32 = 2700;
    nightlight::reset_temperature();
    assert_eq!(nightlight::get_temperature(), temperature);
}
#[test]
fn set_nightlight_active() {
    let active = true;
    nightlight::set_nightlight_active(active);
    assert_eq!(nightlight::get_nightlight_active(), active);
}
#[test]
fn get_nightlight_active() {
    let active = false;
    nightlight::set_nightlight_active(active);
    assert_eq!(nightlight::get_nightlight_active(), active);
}

async fn brightness_up() -> Result<(), String> {
    screen::step_up().await;
    Ok(())
}
async fn brightness_down() -> Result<(), String> {
    screen::step_down().await;
    Ok(())
}
async fn get_brightness() -> Result<i32, String> {
    let brightness = screen::brightness().await;
    Ok(brightness)
}
async fn set_brightness(value: i32) -> Result<(), String> {
    screen::set_brightness(value).await;
    Ok(())
}
