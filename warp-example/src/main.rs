use warp::Filter;
mod routes;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String) 
        .map(|name| format!("Hello, {}!", name)); 

    let stream = warp::path!(String / "stream" / String).map(|application_id, stream_id| format!("{} and {}", application_id, stream_id));
   
    let all = hello.or(stream); 


    warp::serve(all)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
