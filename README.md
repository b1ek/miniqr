# miniQR

miniQR is a micro HTTP service that generates QR codes on the fly.

This server is very small (~1MB docker image) and portable since it
runs anywhere Rust can compile for.

## Building a docker image
Just use the dockerfile provided in the root of the project.

## API Docs
Let's say you want a QR with this text in it: `fkpt`.  

To get this QR, you will need to send a GET request to `http://miniqr_instance/fkpt`. That's it.

## Upscaling the QR image
Since the QR image that is provided is ridiculously small, you will need to upscale it
using your platform's GUI toolkit.

And I am guessing that your GUI toolkit is CSS, so here is a snippet for that:

```css
img.qrcode-image {
    width: 400px;
    image-rendering: pixelated; /* This line "unblurs" the qr code */

    padding: 60px;
    background: white;
}
```

```html
<img src='http://miniqr/aaa' class='qrcode-image'>
```