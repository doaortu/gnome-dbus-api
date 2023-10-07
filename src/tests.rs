use crate::handlers::easy_gnome;

#[test]
fn get_all_apps() {
    let apps = easy_gnome::apps::Apps::new();
    assert!(apps.get_apps().len() > 0);
}
#[test]
fn get_base64_icon() {
    let apps = easy_gnome::apps::Apps::new();
    let app = apps.get_apps().get(0).unwrap();
    println!("{:?}", app.get_base64_icon());
    assert!(app.get_base64_icon().is_some());
}
