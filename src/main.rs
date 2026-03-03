mod games;
mod interfaces;
mod steam;

fn main() {
    let steam = steam::Steam::new();

    let client = steam.create_interface::<interfaces::client::SteamClient>();

    let pipe = client.create_stream_pipe();

    if pipe == 0 {
        println!("failed to create pipe");
        return;
    }

    let user = client.connect_to_global_user(pipe);
    let steam_user = client.get_steam_user(user, pipe);

    let steam_id = steam_user.get_steam_id();
    let is_logged_on = steam_user.get_is_logged_on();

    println!("steam id {:?} - logged on - {:?}", steam_id, is_logged_on);

    let steam_apps = client.get_steam_apps(user, pipe);

    let data = steam_apps.get_appdata(480, "name");
    println!("App name {:?}", data.unwrap())
}
