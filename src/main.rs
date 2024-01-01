const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const USERNAME_KEY_NAME: &str = "VALO-TRACKER-FETCHER_USERNAME";

fn main() {
    println!(" ");

    // DEBUG
    #[cfg(debug_assertions)]
    println!("# DEBUG BUILD!");

    println!("# {} v{} @ github.com/omerakgoz34/{}", APP_NAME, APP_VERSION, APP_NAME);

    let url_tracker = "https://tracker.gg/valorant/profile/riot/";

    // Get player username from system environment variables
    let username = match std::env::var(USERNAME_KEY_NAME) {
        Ok(user) => user,
        Err(_) => {
            println!(" ");
            println!("ERROR > There is no set player username. Please add your player username to system environment variables with the variable name as {}.", USERNAME_KEY_NAME);
            exit_msg();
            return;
        },
    };

    // Fetch the player
    println!(" ");
    println!("> Fetching the player...");
    let url = &(url_tracker.to_owned() + &username.replace("#", "%23"))[..];
    #[allow(unused_variables)]
    let response = match minreq::get(url).with_max_redirects(100).with_timeout(10).send() {
        Ok(res) => res,
        Err(_) => {
            println!(" ");
            println!("ERROR > Couldn't connect to the server. Please check your internet connection.");
            exit_msg();
            return;
        },
    };

    // Log the response
    #[cfg(debug_assertions)]
    {
        println!(" ");
        println!("> HTTP Response: {}", response.as_str().unwrap());
    }

    // Fake wait time :p
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Print the result
    println!(" ");
    println!("> Successfully fetched the player {}", username);
    println!("> The player profile should be updated soon if the player is public on Tracker.");
    exit_msg()
}

// Pause the console before exiting
fn exit_msg() {
    println!(" ");
    println!("Press Enter or close this window to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}