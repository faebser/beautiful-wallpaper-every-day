extern crate reqwest;
use reqwest::Url;

#[macro_use]
extern crate hyper;
use hyper::header::{Headers, Authorization};

extern crate serde;
extern crate serde_json;

extern crate rand;
use rand::Rng;

#[macro_use]
extern crate serde_derive;

extern crate glib;
extern crate gio;
use gio::SettingsExt;
//use glib::prelude::*;
use glib::Variant;

extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{Write, copy};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct Urls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    title: String,
    photo_count: u32,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
struct Exif {
    make: Option<String>,
    model: Option<String>,
    exposure_time: Option<String>,
    aperture: Option<String>,
    focal_length: Option<String>,
    iso: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    _self: String,
    html: String,
    photos: Option<String>,
    likes: Option<String>,
    portfolio: Option<String>,
    download: Option<String>,
    download_location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileImage {
    small: String,
    medium: String,
    large: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    updated_at: Option<String>,
    twitter_username: Option<String>,
    portfolio_url: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    total_likes: u32,
    total_photos: u32,
    total_collections: u32,
    profile_image: ProfileImage,
    links: Links,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Position {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    city: String,
    country: String,
    position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
struct Collection {
    id: u32,
    title: String,
    published_at: String,
    updated_at: String,
    curated: bool,
    cover_photo: CoverPhoto,
    user: User,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoverPhoto {
    id: String,
    width: u16,
    height: u16,
    color: String,
    likes: u16,
    liked_by_user: bool,
    description: Option<String>,
    user: User,
    urls: Urls,
    categories: Vec<Category>,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnsplashFoto {
    id: String,
    created_at: String,
    updated_at: String,
    width: u16,
    height: u16,
    color: String,
    downloads: u16,
    likes: u16,
    liked_by_user: bool,
    description: Option<String>,
    exif: Exif,
    location: Option<Location>,
    current_user_collections: Vec<Collection>,
    urls: Urls,
    categories: Vec<Category>,
    links: Links,
    user: User,
    slug: Option<String>,
}

header! { (AcceptVersion, "Accept-Version") => [String] }

fn main() {
    let mut headers = Headers::new();
    let client = reqwest::Client::new();

    headers.set(Authorization(
        "Client-ID ee88235a89c58088c3ebf8025e90214c4574909913e0b7442165f4f87452384e"
            .to_owned(),
    ));
    headers.set(AcceptVersion("v1".to_owned()));

    let matches = App::new("unsplash")
        .version("0.1.0")
        .about("Small utility that queries the Unsplash API for a random picture,
which it then sets as your background wallpaper.")
        .arg(Arg::with_name("subject")
            .help("Subject of the picture.")
            .short("s")
            .long("subject")
            .takes_value(true))
        .get_matches();

    let mut params = vec![("orientation", "landscape"), ("w", "3840")];
    if let Some(s) = matches.value_of("subject") {
        params.push(("query", s));
    } else {
        params.push(("query", "space stars"));
    }

    // println!("params are {:?}", params);
    
    let url = Url::parse_with_params(
        "https://api.unsplash.com/photos/random",
        &params
    ).unwrap();

    // use xdg pictures directory to save images

    let output = Command::new("xdg-user-dir")
        .arg("PICTURES")
        .output()
        .expect("failed to read PICTURES from xdg-user-dir");


    let input_path = String::from_utf8_lossy(&output.stdout);
    let root_path = input_path.trim_right_matches("\n");

    println!("XDG_PICTURES_DIR is {:?}", root_path);


    let resp = client.get(url).headers(headers).send();
    let json: UnsplashFoto = match resp {
        Ok(mut response) => {
            match response.json() {
                Ok(result) => result,
                Err(e) => {
                    // let's keep the jsons responses that error
                    // to build tests later
                    let name_rnd: String = rand::thread_rng().gen_ascii_chars().take(10).collect();
                    let full_name = &["jsons", &name_rnd].join("/");
                    let path = Path::new(full_name);
                    let mut buf: Vec<u8> = vec![];
                    response.copy_to(&mut buf).unwrap();

                    let mut file = match File::create(&path) {
                        Err(why) => {
                            panic!(
                                "could not create {:?}: {:?}",
                                path.display(),
                                why.description()
                            )
                        }
                        Ok(file) => file,
                    };

                    match file.write_all(buf.as_slice()) {
                        Err(why) => {
                            panic!(
                                "could not write to {:?}: {:?}",
                                path.display(),
                                why.description()
                            )
                        }
                        Ok(_) => println!("wrote faulty json to {:?}", path.display()),
                    };

                    panic!(
                        "JSON PARSE ERROR: {:?} \n {:?}",
                        e,
                        response.text().unwrap()
                    )
                }
            }
        }
        Err(e) => panic!("NETWORK ERROR: {:?}", e),
    };

    match client.get(&json.urls.full).send() {
        Ok(mut response) => {
            // get respones bytes
            // and write them to file
            let name_rnd: String = rand::thread_rng().gen_ascii_chars().take(10).collect();
            let full_name = &[root_path, "backgrounds", &name_rnd].join("/");
            let file_url =
                Variant::from(&["file:/", root_path, "backgrounds", &name_rnd].join("/"));
            let path = Path::new(full_name);

            let mut file = match File::create(&path) {
                Err(why) => {
                    panic!(
                        "could not create {:?}: {:?}",
                        path.display(),
                        why.description()
                    )
                }
                Ok(file) => file,
            };

            match copy(&mut response, &mut file) {
                Err(why) => {
                    panic!(
                        "could not write to {:?}: {:?}",
                        path.display(),
                        why.description()
                    )
                }
                Ok(_) => {
                    println!("wrote image to {:?}", path.display());
                    let settings = gio::Settings::new("org.gnome.desktop.background");
                    settings.set_value("picture-uri", &file_url);
                    gio::Settings::sync();

                    match json.links.download_location {
                        Some(download_location) => {
                            // ping download location per api guidelines
                            let mut headers = Headers::new();
                            headers.set(Authorization(
                                "Client-ID ee88235a89c58088c3ebf8025e90214c4574909913e0b7442165f4f87452384e"
                                    .to_owned(),
                            ));
                            headers.set(AcceptVersion("v1".to_owned()));

                            match client
                                .get(&download_location)
                                .headers(headers)
                                .send() {
                                Ok(_) => println!("Pinged API for download"),
                                Err(_) => println!("Network error while pinging API for download"),
                            }
                        }
                        None => println!("Response did not contain download link"),
                    }
                }
            };
        }
        Err(why) => panic!("NETWORK ERROR: {:?}", why.description()),
    };
}
