use warp::Filter;
use std::fs::File;
use std::io::Read;


#[tokio::main]
async fn main() {
    let mut contacts_file = File::open("contacts.json").unwrap();
    let mut skills_file = File::open("skills.json").unwrap();
    let mut contacts_data = String::new();
    let mut skills_data = String::new();
    contacts_file.read_to_string(&mut contacts_data).unwrap();
    skills_file.read_to_string(&mut skills_data).unwrap();
    let contacts_json: serde_json::Value =
        serde_json::from_str(&contacts_data).expect("JSON was not well-formatted");
    let skills_json: serde_json::Value =
        serde_json::from_str(&skills_data).expect("JSON was not well-formatted");
    let err: serde_json::Value =
        serde_json::json!({"error": "API path not found"});
    let routes = warp::path!( "api" / String)
        .map(move |source: String| 
            match source.as_str() {
                "contacts" => Ok(warp::reply::json(&contacts_json)),
                "skills" => Ok(warp::reply::json(&skills_json)),
                _ => Ok(warp::reply::json(&err))
            });
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
        .allow_methods(vec!["GET", "POST", "DELETE"]);
        
    let route = warp::any()
        .map(warp::reply)
        .with(cors);
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
        .await;
}