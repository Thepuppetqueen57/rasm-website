# I'm lazy
cargo build --target wasm32-unknown-unknown --release
rm -rf release
mkdir release
mv ./target/wasm32-unknown-unknown/release/rasm-website.wasm ./release

html="<html lang="en">
<head>
    <meta charset="utf-8">
    <title>RASM</title>
    <style>
        html,
        body,
        canvas {
            margin: 0px;
            padding: 0px;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
</head>

<body>
    <canvas id=\"glcanvas\" tabindex='1'></canvas>
    <!-- Minified and statically hosted version of https://github.com/not-fl3/macroquad/blob/master/js/mq_js_bundle.js -->
    <script src=\"https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js\"></script>
    <script>load(\"rasm-website.wasm\");</script> <!-- Your compiled wasm file -->
</body>
</html>"

echo "$html" > ./release/index.html