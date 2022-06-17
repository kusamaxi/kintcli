/* 
    KintCLI is command line application written in Rust to make it easy to
    install and run combination of bitcoin node and Interlay/Kintsugi vault.
    You can call it in terminal with `kintcli` command. It has subcommands
    for install the bitcoin node, install the vault, update the vault,
    start the vault, stop the vault, 

    kintcli install has 5 subcommands:
        - subkey: generate a new keypair using subkey and save it to the vault
        - bitcoin: install the bitcoin full node
        - vault: install the vault client
    all 5 subcommands have flags for configuration settings and arguments,
    if required flags are not given then cli will ask for them.
    when you choose `bitcoin` subcommand, it will ask you to choose the
    configuration variables for the bitcoin node.
    when you choose `vault` subcommand, it will ask you to choose the
    configuration variables for the vault like `vault_name`, `vault_port`,
    `vault_network`, `vault_password`.

    kintcli update has 2 subcommands:
        - vault: update the vault client
        - bitcoin: update the bitcoin full node
    
    kintcli config has 2 subcommands:
        - vault: configure the vault client
        - bitcoin: configure the bitcoin full node

    kintcli start has 2 subcommands:
        - vault: start the vault client
        - bitcoin: start the bitcoin full node
    if subcommand is not given, it will start all the services.

    kintcli stop has 2 subcommands:
        - vault: stop the vault client
        - bitcoin: stop the bitcoin full node
    if subcommand is not given, it will stop only vault service.

    kintcli status has 2 subcommands:
        - vault: show the status of the vault client
        - bitcoin: show the status of the bitcoin full node
    if subcommand is not given, it will show the status of vault service.

    kintcli restart has 2 subcommands:
        - vault: restart the vault client
        - bitcoin: restart the bitcoin full node
    if subcommand is not given, it will restart only vault service.

    kintcli logs has 2 subcommands:
        - 1000: show the logs of the all the services, tail -n 1000
        - now: show the logs of all the services, tail -f
    if subcommand is not given, it will show the logs of all the services and follow.


    
*/
use os_info;
use sp_variant;


// Create structs for distros
#[derive(Debug)]
pub struct Distro {
    pub name: String,
    pub pkg: String,
}



// function to detect what distribution user has
// using sp-variant package's struct detect
// if Error message is UnknownVariant, use os_info after that to detect
// the OS.
// return distro
fn detect_distro() -> String {
    if detect_linux() {
        let distro_info = sp_variant::detect();
        println!("{:?}", distro_info);
        match distro_info {
            Ok(distro_info) => {
                let distro = distro_info.family;
                println!("{}", distro);
                return distro.to_string();
            }
            Err(e) => {
                println!("{}", e);
                if e.to_string() == "Could not detect the current host's build variant" {
                    // use os_info library to detect the OS type
                    let info = os_info::get();
                    let distro = info.os_type().to_string();
                    println!("{}", distro);
                    return distro;
                } else {
                    println!("distro still not detected"); 
                    let distro = "Unknown";
                    return distro.to_string();
                }
            }
        }
    }
    else {
        println!("Not linux distribution."); 
        let distro = "Unknown";
        return distro.to_string();
    }
}

// create boolean function to detect if user has linux or not
fn detect_linux() -> bool {
    if cfg!(target_os = "linux") {
        println!("linux detected");
        return true;
    } else {
        println!("Your OS(not linux) is not supported");
        return false;
    }
}

// detect_pkg function is used to detect the package manager
// if user has apt-get, then return apt-get
// if user has pacman, then return pacman
// if user has yum, then return yum
fn detect_pkg(distro: String) -> String {
    let pkg_man;
    if distro == "debian" {
        println!("apt-get detected");
        pkg_man = "apt-get";
    } else if distro == "arch" {
        println!("pacman");
        pkg_man = "pacman";
    } else if distro == "redhat" {
        println!("yum");
        pkg_man = "yum";
    } else {
        println!("pkg not detected: {}", distro);
        pkg_man = &distro;
    }
    return pkg_man.to_string();
}

// function to install dependecies required to run bitcoin node
fn install_bitcoin_dep() {
    println!("installing bitcoin dependencies");
    // check what pkg manager user has
    let pkg_man = detect_pkg(detect_distro());
    println!("{}", pkg_man);
}



fn main() {
    install_bitcoin_dep();
}
