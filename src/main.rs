use std::error;
use std::result;

use structopt::StructOpt;

mod config;
mod steam;
mod tour;

// Generic result
type Result<T> = result::Result<T, Box<dyn error::Error>>;

/// A tool for querying Valve MvM servers
#[derive(StructOpt, Debug)]
enum Opt {
    /// Finds MvM matches
    Find {
        /// Tour to search
        #[structopt(short, long)]
        tour: Option<String>,

        /// Mission to search
        #[structopt(short, long)]
        mission: Option<String>,

        /// Skip player queries
        #[structopt(long)]
        skip_players: bool,

        /*
        /// Number of worker threads for performing player queries
        #[structopt(long, default_value = "4")]
        a2s_threads: u32,
        */
    },

    /// Lists known missions
    ListMissions,

    /// Sets the Steam web API key
    SetKey {
        /// Steam web API key
        key: String,
    },
}

// Unwrap the config or return -1 from the current function if failed
macro_rules! config_or_fail {
    ($conf:expr) => {
        match &$conf {
            Some(c) => c,
            None    => {
                eprintln!("Couldn't find Steam web API key");
                eprintln!("Run `mvmtool set-key <key>`");
                return Ok(-1);
            }
        }
    };
}

struct MvMTool {
    config: Option<config::Config>,
    tours:  tour::Database,
}

impl MvMTool {
    fn new() -> Self {
        Self {
            config: config::Config::load(),
            tours:  tour::Database::new(),
        }
    }

    // Create a master server query for Valve MvM servers
    fn msquery(&self, key: &String) -> steam::ServerListQuery {
        let mut q = steam::ServerListQuery::new(key.to_string());
        q.set_appid(440);
        q.set_empty(false);
        q.set_tags(String::from("valve,mvm,hidden"));

        q
    }

    // Print a server
    fn print_server(&self, server: &steam::Server, key: &String, skip_players: bool) -> Result<()> {
        // Server name
        println!("{}", server.name);

        // IP
        println!("    IP: {}", server.addr);

        // Mission
        if let Some(mission) = self.tours.find_mission(&server.map) {
            println!("    Mission: {} ({})", mission.name, mission.popfile);
        }

        // Players
        println!("    Players: {}/6", server.players);

        if skip_players {
            return Ok(());
        }


        // A2S_PLAYER request
        let players = server.request_players(key)?;
        
        for player in players.iter() {
            println!("        {}", player.name);
            println!("            Kills: {}", player.score);
            let seconds = player.time_played;
            let minutes = seconds / 60;
            let seconds = seconds - minutes * 60;
            println!("            Time: {}m{}s", minutes, seconds);
        }

        Ok(())
    }

    // mvmtool find
    fn find(&self, t: Option<String>, m: Option<String>, skip_players: bool) -> Result<i32> {
        // Find tour if set
        let tour: Option<&tour::Tour> = match t {
            Some(ref t) => self.tours.find_tour(&t),
            None        => None
        };
        // Find mission if set
        let mission: Option<&tour::Mission> = match m {
            Some(ref m) => self.tours.find_mission(&m),
            None        => None
        };
        
        // Failed if the user asked for a tour or mission we don't know
        if t.is_some() && tour.is_none() {
            eprintln!("Unknown tour: {}", t.unwrap());
            eprintln!("Run `mvmtool list-missions` to list known tours");
            return Ok(-1);
        }
        if m.is_some() && mission.is_none() {
            eprintln!("Unknown mission: {}", m.unwrap());
            eprintln!("Run `mvmtool list-missions` to list known missions");
            return Ok(-1);
        }

        // Load Web API key
        let config = config_or_fail!(self.config);

        // Create query
        let mut query = self.msquery(&config.key);

        // If we know exactly what mission to look for, we can add that as a
        // msq filter
        if let Some(mission) = mission {
            query.set_map(mission.popfile.clone());
        }

        // For searching entire tours, request all missions and filter
        // afterwards
        let mission_filter: Option<Vec<String>> = match tour {
            Some(ref tour)  => {
                Some(tour.missions.iter().map(|m| m.popfile.clone()).collect())
            },
            None            => None
        };

        // Request server list from Valve
        let servers = query.send()?;

        // Remove bad results
        let servers: Vec<&steam::Server> = servers.iter()
            .filter(|s| {
                // Some community boot camp servers are tagged as "valve"
                if !s.name.starts_with("Valve Matchmaking Server") {
                    return false;
                }

                // If a tour was supplied, filter to missions in that tour
                if let Some(mission_filter) = &mission_filter {
                    if !mission_filter.contains(&s.map) {
                        return false;
                    }
                }

                true
            }).collect();

        // Get longest server name for pretty printing
        let max_server_name = servers
            .iter()
            .map(|s| s.name.len())
            .max()
            .unwrap_or(50);

        // Ok now print the info
        for server in servers.iter() {
            self.print_server(server, &config.key, skip_players)?;
        }

        // Print totals
        let total_players = servers
            .iter()
            .fold(0u32, |sum, server| sum + server.players);
        let total_max_players = servers.len() as u32 * 6;
        println!("{:=<1$}", "", max_server_name);
        println!("Total servers: {}", servers.len());
        println!("Total players: {}", total_players);
        println!("Total open slots: {}", total_max_players - total_players);
        println!("{:=<1$}", "", max_server_name);

        Ok(0)
    }

    // mvmtool list-missions
    fn list_missions(&self) -> Result<i32> {
        for tour in self.tours.tours.iter() {
            match &tour.nicknames {
                Some(nicknames) => {
                    println!("{} ({}):", tour.name, nicknames.join(", "));
                },
                None => {
                    println!("{}:", tour.name);
                }
            }
            for mission in tour.missions.iter() {
                match &mission.nicknames {
                    Some(nicknames) => {
                        println!("    {} ({})",
                            mission.name, nicknames.join(", "));
                    },
                    None => {
                        println!("    {}", mission.name);
                    }
                }
            }
        }
        Ok(0)
    }

    // mvmtool set-key <key>
    fn set_key(&self, key: String) -> Result<i32> {
        config::Config::create(key);
        Ok(0)
    }
}

fn main() {
    // Init app
    let mvmtool = MvMTool::new();

    // Parse CLI
    let opt = Opt::from_args();

    // Run command
    let result = match opt {
        Opt::SetKey { key } => {
            mvmtool.set_key(key)
        },
        Opt::ListMissions => {
            mvmtool.list_missions()
        },
        Opt::Find { tour, mission, skip_players } => {
            mvmtool.find(tour, mission, skip_players)
        },
    };

    // Exit with code
    std::process::exit(match result {
        Ok(code)    => { code },
        Err(err)    => { eprintln!("Internal error: {}", err); -1 },
    });
}
