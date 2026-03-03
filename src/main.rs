use crate::games::get_game_list;

mod error;
mod games;
mod interfaces;
mod steam;

fn main() -> anyhow::Result<()> {
    let steam = steam::Steam::new()?;
    let client = steam.create_interface::<interfaces::client::SteamClient>()?;

    let pipe = client.create_stream_pipe()?;
    let user = client.connect_to_global_user(pipe);

    let steam_user = client.get_steam_user(user, pipe);

    let steam_id = steam_user.get_steam_id();
    let is_logged_on = steam_user.get_is_logged_on();

    println!("steam id {:?} - logged on - {:?}", steam_id, is_logged_on);

    let steam_apps001 = client.get_steam_apps001(user, pipe);

    // let data = steam_apps001.get_appdata(480, "name");
    // println!("App name {:?}", data.unwrap());

    let steam_apps008 = client.get_steam_apps008(user, pipe);

    for id in get_game_list().unwrap() {
        if !steam_apps008.is_subscribed_app(id) {
            continue;
        }

        let name = steam_apps001.get_appdata(id, "name").unwrap();
        println!("id: {:?} -  name: {:?}", id, name);
    }

    Ok(())
}
