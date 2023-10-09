use qrcode::QrCode;
use image::{Luma, png::PngEncoder};
use urlencoding::decode;

use warp::{Filter, reply::Response};

#[tokio::main]
async fn main() {
    
    femme::with_level(log::LevelFilter::Info);

    let port = std::env::var("PORT").unwrap_or("80".to_string()).parse::<u16>();
    if port.is_err() {
        log::warn!("Could not parse port from environment, falling back to port 80");
        log::warn!("Port parse error message: {}", port.clone().unwrap_err());
    }
    let port = port.unwrap_or(80);

    // Match any request and return hello world!
    let routes =
        warp::path::end()
            .map(|| "Hello, World!")
            .or(
                warp::path!(String)
                    .and(
                        warp::get()
                    )
                    .map(|x: String| {
                            let text = decode(x.as_str()).unwrap().to_string();
                            let code = QrCode::with_error_correction_level(text, qrcode::EcLevel::M).unwrap();
                            let img = code
                                .render::<Luma<u8>>()
                                .quiet_zone(false)
                                .module_dimensions(1, 1)
                                .build();

                            let mut out = Vec::<u8>::new();
                            let enc = PngEncoder::new(&mut out);

                            let (width, height) = (img.width(), img.height());

                            enc.encode(&img.into_raw(), width, height, image::ColorType::L8).unwrap();
                            
                            let mut res = Response::new(out.into());
                            res.headers_mut().insert("Content-Type", warp::http::HeaderValue::from_static("image/png"));
                            res.headers_mut().insert("Date", warp::http::HeaderValue::from_static("Sun, 01 Jan 1984 00:00:00 GMT"));
                            res
                    })
            );

    let (addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(
            ([0, 0, 0, 0], port),
            async move {
                tokio::signal::ctrl_c().await.map_err(|err| format!("Failed to listen to Ctrl-C: {}", err.to_string())).unwrap()
            }
        );
    log::info!("Running on http://{}", addr);
    server.await;

    println!(); // print a new line before exiting
}
