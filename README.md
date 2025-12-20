# Image-Rust
A CLI application for applying various image filter algorithms.
## Implemented Filters
- Reverse Colors
- Pixelation
- Floyd-Steinberg Dithering
- Palette Application
## Usage
```
cargo run --release -- -f <filter_tag_chain> <input_image_path> <output_image_path>
```
## Examples
Original input image:
<p align="center">
  <img src="resources/input.jpg"/>
</p>

Reverse colors:

```
cargo run --release -- -f rev input.jpg output.jpg
```

<p align="center">
  <img src="resources/output_rev.png"/>
</p>

Apply Palette:
```
cargo run --release -- -f pal input.jpg output.jpg
```

<p align="center">
  <img src="resources/output_pal.png"/>
</p>

Dithering:
```
cargo run --release -- -f floyd input.jpg output.jpg
```

<p align="center">
  <img src="resources/output_floyd.png"/>
</p>

Pixelate:
```
cargo run --release -- -f pix=16 input.jpg output.jpg
```

<p align="center">
  <img src="resources/output_pix_16.png"/>
</p>

Chain Filters:
```
cargo run --release -- -f pix=16,pal,rev,floyd input.jpg output.jpg
```

<p align="center">
  <img src="resources/output_chain.png"/>
</p>
