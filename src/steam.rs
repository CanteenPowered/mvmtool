use std::error;
use std::result;

use serde::Deserialize;

static MSQ_ENDPOINT: &str = 
    "https://api.steampowered.com/IGameServersService/GetServerList/v1/";

static PLAYERS_ENDPOINT: &str =
    "https://api.steampowered.com/IGameServersService/QueryByFakeIP/v1/";

// Generic result
type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub addr:       String,
    pub gameport:   u32,
    pub name:       String,
    pub players:    u32,
    pub map:        String,
    pub gametype:   Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlayersResponse {
    response: PlayersResponseInner,
}

#[derive(Debug, Deserialize)]
pub struct PlayersResponseInner {
    players_data: PlayersResponseList,
}

#[derive(Debug, Deserialize)]
pub struct PlayersResponseList {
    players: Vec<Player>
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub name:           String,
    pub score:          i32,
    pub time_played:    i32,
}

impl Server {
    pub fn request_players(&self, key: &String) -> Result<Vec<Player>> {
        // let url = format!("{}?key={}&filter={}&limit=20000",
        //                   MSQ_ENDPOINT, self.key, self.filter);
        // let url = format!("{}?key={}&fake_ip=")
        // Numeric IP address
        let ip_part = self.addr.chars().take_while(|&c| c != ':').collect::<String>();
        let ip_bytes: Vec<u32> = ip_part.split(".").map(|x| x.parse::<u8>().unwrap_or(0u8) as u32).collect();
        let ip = ip_bytes[3] | ip_bytes[2] << 8 | ip_bytes[1] << 16 | ip_bytes[0] << 24;
        // Format request
        let url = format!("{}?key={}&fake_ip={}&fake_port={}&app_id=440&query_type=2", PLAYERS_ENDPOINT, key, ip, self.gameport);
        // GET
        let res: PlayersResponse = reqwest::blocking::get(url)?.json()?;
        Ok(res.response.players_data.players)
    }
}

#[derive(Debug, Deserialize)]
struct ResponseInternal {
    servers: Option<Vec<Server>>,
}

#[derive(Debug, Deserialize)]
struct Response {
    response: ResponseInternal
}

#[derive(Debug)]
pub struct ServerListQuery {
    key: String,
    filter: String,
}

impl ServerListQuery {
    pub fn new(key: String) -> Self {
        Self {
            key: key,
            filter: String::new()
        }
    }

    pub fn set_appid(&mut self, appid: u32) {
        self.filter = format!("{}\\appid\\{}", self.filter, appid);
    }

    pub fn set_empty(&mut self, empty: bool) {
        self.filter = match empty {
            true =>     format!("{}\\noplayers\\1", self.filter),
            false =>    format!("{}\\empty\\1", self.filter)
        };
    }

    pub fn set_tags(&mut self, tags: String) {
        self.filter = format!("{}\\gametype\\{}", self.filter, tags);
    }

    /*
    pub fn set_full(&mut self, full: bool) {
        // Can't search for only full servers
        if (full == false) {
            self.filter = format!("{}\\full\\1", self.filter);
        }
    }
    */

    pub fn set_map(&mut self, map: String) {
        self.filter = format!("{}\\map\\{}", self.filter, map);
    }

    pub fn send(&self) -> Result<Vec<Server>> {
        // Build filter
        let url = format!("{}?key={}&filter={}&limit=20000",
                          MSQ_ENDPOINT, self.key, self.filter);
        // GET request, parse from json
        let res: Response = reqwest::blocking::get(url)?.json()?;
        match res.response.servers {
            Some(s) => Ok(s),
            None    => Ok(vec![])
        }
    }
}
