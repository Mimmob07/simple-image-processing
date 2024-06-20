# Simple Image Processing in Rust

## Features
 - Grayscale
 - Reflect [^1]
 - Blur
 - Edge Detection [^2]

[^1]: Reflects the image horizontally
[^2]: Uses a sobel operator on all 3 channels for grayscale and color edge detection

## Edge Detection Showcase
| Before | After |
| ----------- | ----------- |
| ![bike before](images/400px-Bikesgray.jpg) | ![bike after](images/400px-Bikesgray_after.jpg) |
| ![bike before](images/wallhaven-water.png) | ![bike after](images/wallhaven-water.png) |
