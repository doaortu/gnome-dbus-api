use handlers::easy_gnome;

mod handlers;
#[tokio::main]
async fn main() {
    // println!("{}", easy_gnome::nightlight::nightlight_active().await);
    // println!("{}", easy_gnome::screen::brightness().await);
    // easy_gnome::screen::set_brightness(40).await;
    // easy_gnome::screen::step_up().await;
    // easy_gnome::screen::step_down().await;
    easy_gnome::apps::get_all().await;
}
