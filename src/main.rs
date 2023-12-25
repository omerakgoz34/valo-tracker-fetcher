const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!(" ");
    #[cfg(debug_assertions)]
    println!(">>> DEBUG BUILD!");
    println!(">>> {} v{}", APP_NAME, APP_VERSION);

    let url_tracker = "https://tracker.gg/valorant/profile/riot/";

    // Get the player username from system environment variables
    let username = match std::env::var("valo-tracker-fetcher_username") {
        Ok(user) => user,
        Err(_) => {
            println!(" ");
            println!(">>> ERROR: The player username is not exist. Please make sure you added your valorant tracker username to system environment variables.");
            return;
        },
    };

    // Fetch the player
    println!(" ");
    println!(">>> Fetching the player...");
    
    let url = &(url_tracker.to_owned() + &username)[..];
    #[allow(unused_variables)]
    let response = match  minreq::get(url).send() {
        Ok(res) => res,
        Err(_) => {
            println!(" ");
            println!(">>> ERROR: Couldn't connect to the server. Please check your internet connection.");
            return;
        },
    };
    println!(" ");
    println!(">>> Successfully fetched the player {}", username);
    println!(">>> The player profile should be updated soon if the player is public on Tracker.");
    println!("Press Enter to exit.");
    #[allow(unused_variables)]
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();
}
