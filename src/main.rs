mod steam;
mod wrappers;

fn main() {
    let steam = steam::Steam::new();

    let client = steam.create_interface::<wrappers::client::SteamClient>();

    let pipe = client.create_stream_pipe();

    if pipe == 0 {
        println!("failed to create pipe");
    }

    println!("{:?}", pipe);
}
