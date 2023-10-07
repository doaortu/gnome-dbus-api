use handlers::easy_gnome;

mod handlers;

#[tokio::main]
async fn main() {
    // println!("{}", easy_gnome::nightlight::nightlight_active().await);
    // println!("{}", easy_gnome::screen::brightness().await);
    // easy_gnome::screen::set_brightness(40).await;
    // easy_gnome::screen::step_up().await;
    // easy_gnome::screen::step_down().await;
    // let apps = easy_gnome::apps::Apps::new();
    // let a = apps.get_apps();
    // for i in a {
    //     println!("{}", i.name);
    // }
    // println!("{}", apps.apps[0].get_base64_icon().unwrap());
}
