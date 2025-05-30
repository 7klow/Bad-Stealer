/*

▀█████████▄     ▄████████ ████████▄     ▄████████     ███        ▄████████    ▄████████  ▄█          ▄████████    ▄████████ 
  ███    ███   ███    ███ ███   ▀███   ███    ███ ▀█████████▄   ███    ███   ███    ███ ███         ███    ███   ███    ███ 
  ███    ███   ███    ███ ███    ███   ███    █▀     ▀███▀▀██   ███    █▀    ███    ███ ███         ███    █▀    ███    ███ 
 ▄███▄▄▄██▀    ███    ███ ███    ███   ███            ███   ▀  ▄███▄▄▄       ███    ███ ███        ▄███▄▄▄      ▄███▄▄▄▄██▀ 
▀▀███▀▀▀██▄  ▀███████████ ███    ███ ▀███████████     ███     ▀▀███▀▀▀     ▀███████████ ███       ▀▀███▀▀▀     ▀▀███▀▀▀▀▀   
  ███    ██▄   ███    ███ ███    ███          ███     ███       ███    █▄    ███    ███ ███         ███    █▄  ▀███████████ 
  ███    ███   ███    ███ ███   ▄███    ▄█    ███     ███       ███    ███   ███    ███ ███▌    ▄   ███    ███   ███    ███ 
▄█████████▀    ███    █▀  ████████▀   ▄████████▀     ▄████▀     ██████████   ███    █▀  █████▄▄██   ██████████   ███    ███ 
                                                                                        ▀                        ███    ███ 
    >> Github : https://github.com/7klow/Bad-Stealer
    >> License: GNU General Public License v3.0
*/

mod config;
mod utils;
mod watcher;
mod network;

use tokio::signal;
use colored::*;

use config::Config;
use config::KeywordConfig;
use utils::platform::get_platform;

#[tokio::main]
async fn main() {
    let ascii_banner: &str = r#"
        ▀█████████▄     ▄████████ ████████▄     ▄████████     ███        ▄████████    ▄████████  ▄█          ▄████████    ▄████████ 
          ███    ███   ███    ███ ███   ▀███   ███    ███ ▀█████████▄   ███    ███   ███    ███ ███         ███    ███   ███    ███ 
          ███    ███   ███    ███ ███    ███   ███    █▀     ▀███▀▀██   ███    █▀    ███    ███ ███         ███    █▀    ███    ███ 
         ▄███▄▄▄██▀    ███    ███ ███    ███   ███            ███   ▀  ▄███▄▄▄       ███    ███ ███        ▄███▄▄▄      ▄███▄▄▄▄██▀ 
        ▀▀███▀▀▀██▄  ▀███████████ ███    ███ ▀███████████     ███     ▀▀███▀▀▀     ▀███████████ ███       ▀▀███▀▀▀     ▀▀███▀▀▀▀▀   
          ███    ██▄   ███    ███ ███    ███          ███     ███       ███    █▄    ███    ███ ███         ███    █▄  ▀███████████ 
          ███    ███   ███    ███ ███   ▄███    ▄█    ███     ███       ███    ███   ███    ███ ███▌    ▄   ███    ███   ███    ███ 
        ▄█████████▀    ███    █▀  ████████▀   ▄████████▀     ▄████▀     ██████████   ███    █▀  █████▄▄██   ██████████   ███    ███ 
                                                                                        ▀                        ███    ███
                                                                                        "#;

    println!("{}\n💻 Platform: {}\n📁 Config Dir: {}",
            ascii_banner.cyan().bold(),
            get_platform().blue(),
            config::get_config_dir_str().green()
    );

    let path_config = Config::new();
    let keyword_config = KeywordConfig::new();
    
    println!("✅ Path config loaded: {:?}\n✅ Keyword config loaded: {:?}",
                path_config, keyword_config);

    #[cfg(feature = "linux")]
    {
        if unsafe { libc::getuid() } != 0 {
            println!("⚠️ Warning: Running without root privileges.");
            println!("⚠️ Linux blocking mode requires root access.");
            println!("⚠️ Run with: sudo ./bad-stealer");
        }
    }

    #[cfg(feature = "windows")]
    {
        println!("⚠️ Make sure you're running as Administrator for full functionality.");
    }

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("\n❌ User shutdown.");
        }
    }

    println!("🔒 Surveillance stopped properly.");
}