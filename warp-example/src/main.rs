use warp::Filter;
mod routes;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String) 
        .map(|name| format!("Hello, {}!", name)); 

    let stream = warp::path!(String / "stream" / String).map(|application_id, stream_id| format!("{}, {}", application_id, stream_id));
    


    warp::serve(hello)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
