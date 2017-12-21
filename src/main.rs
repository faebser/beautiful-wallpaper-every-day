extern crate reqwest;
#[macro_use]
extern crate hyper;
use hyper::header::{Headers, Authorization};

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Urls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Category {
    id: String,
    title: String,
    photo_count: u32,
    links: Links,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Exif {
    make: Option<String>,
    model: Option<String>,
    exposure_time: Option<String>,
    aperture: Option<String>,
    focal_length: Option<String>,
    iso: Option<u16>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
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

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct ProfileImage {
    small: String,
    medium: String,
    large: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct User {
    id: String,
    username: String,
    name: String,
    first_name: String,
    last_name: Option<String>,
    updated_at: Option<String>,
    twitter_username: Option<String>,
    portfolio_url: String,
    bio: String,
    location: String,
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

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Location {
    city: String,
    country: String,
    position: Position,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
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

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
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

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
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

    let resp = client
        .get("https://api.unsplash.com/photos/random")
        .headers(headers)
        .send();


    let json: UnsplashFoto = match resp {
        Ok(mut response) => {
            match response.json() {
                Ok(result) => result,
                Err(e) => {
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

    println!("{:?}", json);

    //let test: UnsplashFoto = serde_json::from_str(data).unwrap();

    //assert!(resp.status().is_success());

    //let mut content = String::new();
    //resp.read_to_string(&mut content);
}