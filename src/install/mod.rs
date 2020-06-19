use crate::helper::{get_all_mods, DepEq, Dependency, ModDown, ModMatch};


use prettytable::Table;
use semver::Version;
use std::fs::{File};
use std::path::{Path};
use std::{io, thread};

pub fn install_user_interaction(mods_path: &Path, api_token: &str) {
    let abort = false;
    let mut to_install: Vec<String> = vec![];
    while !abort {
        println!("Type mod to install. Insert 'q' to quit or 'i' to install selected.");
        let line: String = text_io::read!("{}\n");

        if line == "q" {
            return;
        }
        if line == "i" {
            install(mods_path, to_install.clone(), api_token, true);
            return;
        }

        let matches = crate::helper::find_near_match(&line);
        if &matches.get(0).unwrap().levenshtein_distance == &0 {
            let mn = String::from(&matches.get(0).unwrap().mod_name);
            let mt =&matches.get(0).unwrap().mod_title;
            if to_install.contains(&mn) {
                println!("Already downloading {}", &mt);
            } else {
                to_install.push(mn);
                println!("Installing {}", &mt);
            }
        } else {
            println!("Found multiple options:");
            let mut table = Table::new();
            table.add_row(row![
                "ID",
                "Name",
                "Title",
                "Downloads",
                "Levenshtein distance"
            ]);
            let mut i = 0;
            for m in &matches {
                table.add_row(row![
                    i,
                    m.mod_name,
                    m.mod_title,
                    m.downloads,
                    m.levenshtein_distance
                ]);
                i += 1;
            }

            table.printstd();

            println!("Select mod to install, insert 'n' to select none");
            let selected: String = text_io::read!("{}\n");
            if selected == "n" {
                continue;
            }

            match selected.parse::<usize>() {
                Ok(iv) => match &matches.get(iv) {
                    None => println!("Number outside table, aborting."),
                    Some(ivv) => {
                        to_install.push(String::from(&ivv.mod_name));
                        println!("Installing {}", &matches.get(0).unwrap().mod_title);
                    }
                },
                Err(e) => {
                    println!("{}.", e);
                }
            }
        }
    }
}

pub fn install(mods_path: &Path, params: Vec<String>, api_token: &str, gui: bool) {
    println!("Dependency parsing for: ");
    let mut table = Table::new();
    for param in &params {
        table.add_row(row![param]);
    }
    table.printstd();

    let current_version = crate::helper::parse_version_number(mods_path.parent().unwrap());

    let mut to_install: Vec<ModDown> = vec![];

    for param in params {
        let mds = parse_mod(param, api_token, &current_version, None, None);
        for md in mds {
            to_install.push(md);
        }
    }

    to_install.sort();
    to_install.dedup();

    println!("Parsed dependency graph:");
    let mut table = Table::new();
    for param in &to_install {
        table.add_row(row![param.file_name, param.url]);
    }
    table.printstd();

    if gui {
        println!("Enter 'd' to start downloading, else aborting");
        let selected: String = text_io::read!("{}\n");
        if selected != "d" {
            println!("Aborting");
            install_user_interaction(mods_path, api_token);
            return;
        }
    }

    let (s,r) = crossbeam::crossbeam_channel::unbounded();

    for m in to_install {
        s.send(m);
    }

    let mut threads = vec![];
    for _tid in 0..16 {
        let api_token_s = String::from(api_token);
        let path_s = String::from(mods_path.to_str().unwrap());
        let r1 = r.clone();
        threads.push(thread::spawn(move || {
            download_worker(r1, api_token_s, path_s);
        }));
    }

    for thread in threads {
        thread.join();
    }

    println!("Finished downloading !");
}

pub fn download_worker(r: crossbeam::crossbeam_channel::Receiver<ModDown>, api_token: String, mods_path: String){
    while r.len() > 0 {
        let md = r.recv();
        match md {
            Ok(v) => {
                let file_path = format!("{}\\{}", mods_path, v.file_name);
                if !Path::new(&file_path).exists() {
                    let down_url = format!("https://mods.factorio.com/{}{}", v.url, api_token);
                    println!("Downloading {} from {}", &v.file_name, &down_url);
                    let resp_x = crate::helper::RQClient.get(&down_url).send();
                    match resp_x {
                        Ok(mut vv) => {
                            let mut out = File::create(file_path).unwrap();
                            io::copy(&mut vv, &mut out).unwrap();
                            println!("Finished downloading {} !", &v.file_name);
                        }
                        Err(e) => {
                            println!("Failed to download mod: {} {}", e, v.file_name);
                        }
                    }
                }else{
                    println!("{} already downloaded !", v.file_name);
                }
            },
            Err(e) => {
                println!("RecvError: {}", e)
            },
        }
    }
}


pub fn parse_mod(mod_name: String, api_token: &str, current_version: &Version, version: Option<&Version>, eq: Option<&DepEq>) -> Vec<ModDown> {
    let mut to_install: Vec<ModDown> = vec![];
    if mod_name == "Base" || mod_name == "base" {
        return to_install;
    }
    let resp_x = crate::helper::RQClient
        .get(&format!(
            "https://mods.factorio.com/api/mods/{}/full{}",
            mod_name, api_token
        ))
        .send();

    match resp_x {
        Ok(v) => {
            if v.status().is_success() {
                let resp = v.text().unwrap();
                match serde_json::from_str::<crate::json::mod_full::ModFull>(&resp) {
                    Ok(v) => match v.releases {
                        None => println!("{} has no release, not installing", mod_name),
                        Some(vv) => {
                            let mut x_release: Option<usize> = None;
                            let mut x_dep: Option<Vec<Dependency>> = None;

                            let mut i = 0;
                            for vvx in &vv {
                                //println!("Checking: {:#?}", &vvx);
                                let release_factrio_version =
                                    Version::parse(&if vvx.info_json.factorio_version.len() == 4 {
                                        String::from(&vvx.info_json.factorio_version) + ".0"
                                    } else {
                                        String::from(&vvx.info_json.factorio_version)
                                    })
                                    .unwrap();

                                let release_version = Version::parse(&if vvx.version.len() == 4 {
                                    String::from(&vvx.version) + ".0"
                                } else {
                                    String::from(&vvx.version)
                                }).unwrap();

                                if release_factrio_version.major == current_version.major
                                    && release_factrio_version.minor == current_version.minor
                                {
                                    //println!("Correct major and minor ! {:?} : {:?}", release_factrio_version, current_version);
                                    let dependencies =
                                        crate::helper::parse_deps(&vvx.info_json.dependencies);
                                    let mut valid_base_dep = false;
                                    let mut has_base_dep = false;
                                    for dependency in &dependencies {
                                        if dependency.name == "Base" {
                                            has_base_dep = true;
                                            valid_base_dep = match dependency.eq {
                                                DepEq::Smaller => {
                                                    current_version < &dependency.version
                                                }
                                                DepEq::SmallerEqual => {
                                                    current_version <= &dependency.version
                                                }
                                                DepEq::Equal => {
                                                    current_version == &dependency.version
                                                }
                                                DepEq::GreaterEqual => {
                                                    current_version >= &dependency.version
                                                }
                                                DepEq::Greater => {
                                                    current_version > &dependency.version
                                                }
                                            };
                                        }
                                    }
                                    if valid_base_dep || !has_base_dep {
                                        match &version {
                                            None => {
                                                x_release = Some(i);
                                                x_dep = Some(dependencies);
                                            }
                                            Some(ve) => {
                                                if match &eq.unwrap() {
                                                    DepEq::Smaller => &&release_version < ve,
                                                    DepEq::SmallerEqual => &&release_version <= ve,
                                                    DepEq::Equal => &&release_version == ve,
                                                    DepEq::GreaterEqual => &&release_version >= ve,
                                                    DepEq::Greater => &&release_version > ve,
                                                } {
                                                    x_release = Some(i);
                                                    x_dep = Some(dependencies);
                                                }
                                            }
                                        }
                                    }
                                }
                                i += 1;
                            }

                            if x_release == None {
                                println!(
                                    "{} has no release for factorio version {}",
                                    mod_name, current_version
                                );
                            } else {
                                let r = vv.get(x_release.unwrap()).unwrap();
                                to_install.push(ModDown {
                                    url: String::from(&r.download_url),
                                    file_name: String::from(&r.file_name),
                                });

                                let depsx = parse_dep_cyclic(x_dep.unwrap(), api_token, current_version);
                                for depsx in depsx {
                                    to_install.push(depsx);
                                }
                            }
                        }
                    },
                    Err(e) => {
                        println!("Failed to parse modlist: {}", e);
                    }
                }
            } else {
                println!("Downloading mod failed: {} for {}", v.status(), mod_name);
            }
        }
        Err(e) => {
            println!("Downloading mod failed !: {} for {}", e, mod_name);
        }
    }

    return to_install;
}

pub fn parse_dep_cyclic(deps: Vec<Dependency>, api_token: &str, current_version: &Version) -> Vec<ModDown> {
    let mut moddowns: Vec<ModDown> = vec![];

    for dep in deps {
        for v in parse_mod(dep.name, api_token, current_version, Some(&dep.version), Some(&dep.eq)){
            moddowns.push(v);
        }
    }

    return moddowns;
}
