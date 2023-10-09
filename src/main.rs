use qrcode::QrCode;
use image::{Luma, png::PngEncoder};
use urlencoding::decode;

use warp::{Filter, reply::Response};

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
                            res
                    })
            );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}