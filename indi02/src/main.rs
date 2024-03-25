use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name: String| {
            let reversed = name.chars().rev().collect::<String>();
            format!("Hello, {}! Your name in reverse is {}.", name, reversed)
        });

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}