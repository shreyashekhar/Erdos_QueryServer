use warp::Filter;
use std::fs;

mod routes;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String) 
        .map(|name| {
	   format!("Hello, {}!", name)
	});
    
    let stream = warp::path!(String / "stream" / String)
	.map(|application_id, stream_id| {
	   format!("{} and {}", application_id, stream_id)
	});
    
    let graph = warp::path!("graph")
	.map(|| {fs::read_to_string("test.txt").expect("null")});
   
    let cors = warp::cors()
    	.allow_origin("http://localhost:3000")
    	.allow_methods(vec!["GET", "POST", "DELETE"]);

    let all = hello.or(stream).or(graph).with(cors);

    warp::serve(all)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
