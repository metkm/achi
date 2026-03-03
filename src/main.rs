mod interfaces;
mod steam;

fn main() {
    let steam = steam::Steam::new();

    let client = steam.create_interface::<interfaces::client::SteamClient>();

    let pipe = client.create_stream_pipe();

    if pipe == 0 {
        println!("failed to create pipe");
    }

    let user = client.connect_to_global_user(pipe);
    let steam_user = client.get_steam_user(user, pipe);

    println!(
        "steam user and it's id {:?} - {:?}",
        steam_user,
        steam_user.get_steam_id()
    );
}
