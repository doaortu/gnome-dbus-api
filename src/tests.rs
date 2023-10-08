use crate::handlers::easy_gnome;
use crate::handlers::easy_gnome::nightlight;
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
fn get_base64_icon() {
    let apps = easy_gnome::apps::Apps::new();
    let app = apps.get_apps().get(0).unwrap();
    println!("{:?}", app.get_base64_icon());
    assert!(app.get_base64_icon().is_some());
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
