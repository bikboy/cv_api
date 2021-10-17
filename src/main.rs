use warp::Filter;
use std::fs::File;
use std::io::Read;


#[tokio::main]
async fn main() {
    let mut contacts_file = File::open("contacts.json").unwrap();
    let mut skills_file = File::open("skillsv2.json").unwrap();
    let mut summary_file = File::open("summary.json").unwrap();
    let mut experience_file = File::open("experience.json").unwrap();
    let mut education_file = File::open("education.json").unwrap();
    let mut contacts_data = String::new();
    let mut skills_data = String::new();
    let mut summary_data = String::new();
    let mut experience_data = String::new();
    let mut education_data = String::new();
    contacts_file.read_to_string(&mut contacts_data).unwrap();
    skills_file.read_to_string(&mut skills_data).unwrap();
    summary_file.read_to_string(&mut summary_data).unwrap();
    experience_file.read_to_string(&mut experience_data).unwrap();
    education_file.read_to_string(&mut education_data).unwrap();
    let contacts_json: serde_json::Value =
        serde_json::from_str(&contacts_data).expect("JSON was not well-formatted");
    let skills_json: serde_json::Value =
        serde_json::from_str(&skills_data).expect("JSON was not well-formatted");
    let summary_json: serde_json::Value =
        serde_json::from_str(&summary_data).expect("JSON was not well-formatted");
    let experience_json: serde_json::Value =
        serde_json::from_str(&experience_data).expect("JSON was not well-formatted");
    let education_json: serde_json::Value =
        serde_json::from_str(&education_data).expect("JSON was not well-formatted");
    let err: serde_json::Value =
        serde_json::json!({"error": "API path not found"});
    let routes = warp::path!( "api" / String)
        .map(move |source: String| 
            match source.as_str() {
                "contacts" => Ok(warp::reply::json(&contacts_json)),
                "skills" => Ok(warp::reply::json(&skills_json)),
                "summary" => Ok(warp::reply::json(&summary_json)),
                "experience" => Ok(warp::reply::json(&experience_json)),
                "education" => Ok(warp::reply::json(&education_json)),
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