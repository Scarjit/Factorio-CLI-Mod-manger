
use lazy_static::lazy_static;
use semver::{Version};


use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;



lazy_static! {
    pub static ref RQClient: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

pub fn parse_version_number(server_files_path: &Path) -> Version {
    let pdata_path = server_files_path.join(Path::new("player-data.json"));
    match File::open(pdata_path) {
        Ok(pfile) => {
            match serde_json::from_reader::<BufReader<File>, crate::json::player_data::PlayerData>(
                BufReader::new(pfile),
            ) {
                Ok(v) => unsafe {
                    match Version::parse(&v.last_played_version.unwrap().game_version.unwrap()) {
                        Ok(v) => {
                            return v;
                        }
                        Err(e) => {
                            panic!("Could not parse game version: {}", e);
                        }
                    }
                },
                Err(e) => {
                    panic!("Could not decode player-data.json: {:?}", e);
                }
            }
        }
        Err(e) => {
            panic!("Could not load player-data.json: {:?}.\nStart factorio at least once before using this tool !", e);
        }
    }
}

pub fn login(username: &str, password: &str) -> String {
    let res = RQClient
        .post("https://auth.factorio.com/api-login")
        .query(&[("username", &username), ("password", &password)])
        .send()
        .unwrap()
        .text()
        .unwrap();
    let token = String::from(res).replace("[","").replace("]","").replace("\"", "");
    return format!("?username={}&token={}", username, token);
}

pub static mut FULL_LIST: Option<crate::json::mod_list::FullList> = None;
pub fn get_all_mods() -> bool {
    println!("Downloading mod list ...");
    unsafe {
        if FULL_LIST.is_none() {
            let resp_x = RQClient
                .get("https://mods.factorio.com/api/mods?page_size=max")
                .send();

            match resp_x {
                Ok(v) => {
                    if v.status().is_success() {
                        let resp = v.text().unwrap();
                        match serde_json::from_str::<crate::json::mod_list::FullList>(&resp) {
                            Ok(v) => {
                                FULL_LIST = Some(v);
                                return true;
                            }
                            Err(e) => {
                                println!("Failed to parse modlist: {}", e);
                            }
                        }
                    } else {
                        println!("Downloading mod list failed: {}", v.status());
                    }
                }
                Err(e) => {
                    println!("Downloading mod list failed !: {}", e);
                }
            }
        }
    }
    return false;
}

pub fn mod_exists(mod_name: &str) -> Option<String> {
    let list = &unsafe { crate::helper::FULL_LIST.as_ref() }
        .unwrap()
        .results;
    for r in list {
        if r.name == mod_name || r.title == mod_name {
            return Some(String::from(&r.name));
        }
    }
    return None;
}

pub struct ModMatch {
    pub downloads: i64,
    pub levenshtein_distance: usize,
    pub mod_name: String,
    pub mod_title: String,
}

pub fn find_near_match(mod_name: &str) -> Vec<ModMatch> {
    let list = &unsafe { crate::helper::FULL_LIST.as_ref() }
        .unwrap()
        .results;
    let mut near_matches: Vec<ModMatch> = Vec::new();
    for r in list {
        let lv_name = levenshtein::levenshtein(&r.name, mod_name);
        let lv_title = levenshtein::levenshtein(&r.title, mod_name);

        let lv = if lv_name < lv_title {
            lv_name
        } else {
            lv_title
        };

        if lv < 5 {
            near_matches.push(ModMatch {
                downloads: r.downloads_count,
                levenshtein_distance: lv,
                mod_name: String::from(&r.name),
                mod_title: String::from(&r.title),
            })
        }
    }

    near_matches.sort_by(|a, b| {
        if a.levenshtein_distance <= 2 && b.levenshtein_distance <= 2 {
            if a.levenshtein_distance != b.levenshtein_distance {
                return if a.levenshtein_distance < b.levenshtein_distance {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
        } else if a.levenshtein_distance <= 2 {
            return Ordering::Less;
        } else if b.levenshtein_distance <= 2 {
            return Ordering::Greater;
        }

        //Sort by downloads descending
        return b.downloads.cmp(&a.downloads);
    });

    return near_matches;
}

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub version: Version,
    pub eq: DepEq,
}

#[derive(Debug)]
pub enum DepEq {
    Smaller,
    SmallerEqual,
    Equal,
    GreaterEqual,
    Greater,
}

pub fn parse_deps(deps: &Vec<String>) -> Vec<Dependency> {
    let mut depvec: Vec<Dependency> = vec![];
    for dep in deps {
        let d_one = dep.chars().next().unwrap();
        let mut d = Dependency {
            name: "".to_string(),
            version: Version::new(0, 0, 0),
            eq: DepEq::Smaller,
        };

        if d_one == '?' || d_one == '(' || d_one == '!' {
            continue;
        }

        let splits: Vec<&str> = dep.split(' ').collect();
        d.name = String::from(splits[0]);
        let mut v = String::from(splits[2]);
        if v.len() == 4 {
            v += ".0";
        }
        d.version = Version::parse(&v).unwrap();

        let eq_op = splits[1];

        d.eq = match eq_op {
            "<" => DepEq::Smaller,
            "<=" => DepEq::SmallerEqual,
            "=" => DepEq::Equal,
            ">" => DepEq::Greater,
            ">=" => DepEq::GreaterEqual,
            _ => panic!("Dependency parsing failed ! {}", dep),
        };

        depvec.push(d);
    }

    return depvec;
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct ModDown {
    pub url: String,
    pub file_name: String,
}
