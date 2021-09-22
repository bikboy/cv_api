use warp::Filter;

#[tokio::main]
async fn main() {
    let contacts = warp::path!( "api" / "contacts" / String)
        .map(|source: String| 
            match source.as_str() {
                "mail" => "t.bikbaev@gmail.com",
                "skype" => "tim.bikbaev",
                "phone" => "+380671014882",
                "telegram" => "@t_bikbaev",
                _ => "Not found"
            });

    warp::serve(contacts)
        .run(([0, 0, 0, 0], 5000))
        .await;
}