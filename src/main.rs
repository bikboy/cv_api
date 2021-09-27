use warp::Filter;

#[tokio::main]
async fn main() {
    // struct contact {
    //     mail: String,
    //     skype: String,
    //     phone: String,
    //     telegram: String
    // }
    // let c = contact {
    //     mail: String::from("t.bikbaev@gmail.com"),
    //     skype: String::from("tim.bikbaev"),
    //     phone: String::from("+380671014882"),
    //     telegram: String::from("@t_bikbaev")
    // };
    let contacts = warp::path!( "api" / String)
        .map(|source: String| 
            match source.as_str() {
                "contacts" => "{\"mail\": \"t.bikbaev@gmail.com\",
\"skype\": \"tim.bikbaev\",
\"phone\": \"+380671014882\",
\"telegram\": \"@t_bikbaev\"}",
                _ => "Not found"
            });
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
        .allow_methods(vec!["GET", "POST", "DELETE"]);
        
    let route = warp::any()
        .map(warp::reply)
        .with(cors);
    warp::serve(contacts)
        .run(([0, 0, 0, 0], 5000))
        .await;
}