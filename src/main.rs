use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes =
        warp::path::end()
            .map(|| "Hello, World!")
            .or(
                warp::path!(String)
                    .and(
                        warp::get()
                    )
                    .map(|x| format!("u get {x}"))
            );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}