package main

import (
	"fmt"
	"net/http"
	"os"
	"strconv"
	"strings"
)

const AppName string = "valo-tracker-fetcher"
const AppVersion string = "0.2.1"
const UsernameKeyName string = "VALO-TRACKER-FETCHER_USERNAME"
const TrackerURL string = "https://tracker.gg/valorant/profile/riot/"

func main() {
	fmt.Println("# " + AppName + " v" + AppVersion + " @ github.com/omerakgoz34/" + AppName)

	// Get player username from system environment variables
	username, ok := os.LookupEnv(UsernameKeyName)
	if !ok {
		fmt.Println("")
		fmt.Println("ERROR > There is no set player username. Please add your player username to system environment variables with the variable name " + UsernameKeyName)
		exitMsg()
		os.Exit(1)
	}

	// Fetch the player
	fmt.Println("")
	fmt.Println("> Fetching player " + username + " ...")
	resp, err := http.Get(TrackerURL + strings.ReplaceAll(username, "#", "%23"))

	// Check for errors
	if err != nil {
		fmt.Println("")
		fmt.Println("ERROR > Couldn't connect to the server! Please check your internet connection.")
		fmt.Println("ERROR > " + err.Error())
		exitMsg()
		os.Exit(1)
	} else if resp.StatusCode == 404 { // check for invalid username
		fmt.Println("")
		fmt.Println("ERROR > Couldn't find the player! Please check your username.")
		exitMsg()
		os.Exit(1)
	} else if !strings.HasPrefix(strconv.Itoa(resp.StatusCode), "2") { // check for http errors
		fmt.Println("")
		fmt.Println("ERROR > Unknown error!")
		fmt.Println("ERROR > HTTP Status: " + resp.Status)
		exitMsg()
		os.Exit(1)
	}

	// Print the result
	fmt.Println("")
	fmt.Println("> Fetched successfully!")
	fmt.Println("> It might take a while to update the profile.")
	exitMsg()
}

// Console exit message
func exitMsg() {
	fmt.Println("")
	fmt.Print("Press Enter or close this window to exit.")
	fmt.Scanln()
}
