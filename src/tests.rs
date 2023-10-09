use crate::handlers::easy_gnome;
use crate::handlers::easy_gnome::nightlight;
use crate::handlers::easy_gnome::peripherals;
use crate::handlers::easy_gnome::power;
use crate::handlers::easy_gnome::screen;
use crate::handlers::easy_gnome::screenshot;
#[test]
fn get_all_apps() {
    let apps = easy_gnome::apps::Apps::new();
    assert!(apps.get_apps().len() > 0);
}
async fn pick_color() {
    let (r, g, b) = screenshot::pick_color().await;
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

async fn get_nightlight_active() -> Result<bool, String> {
    let is_active: bool = nightlight::nightlight_active().await;
    Ok(is_active)
}
async fn get_temperature() -> Result<u32, String> {
    let temperature: u32 = nightlight::temperature().await;
    Ok(temperature)
}
async fn set_nightlight(status: bool) -> Result<(), String> {
    nightlight::set_nightlight_active(status).await;
    Ok(())
}
async fn set_temperature(temperature: u32) -> Result<(), String> {
    nightlight::set_temperature(temperature).await;
    Ok(())
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
