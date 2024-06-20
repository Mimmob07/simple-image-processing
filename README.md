# Simple Image Processing in Rust

## Features
 - Grayscale
 - Reflect<sup>1</sup>
 - Blur
 - Edge Detection<sup>2</sup>

<sup>1</sup> Reflects the image horizontally

<sup>2</sup> Uses a sobel operator on all 3 channels for grayscale and color edge detection

## Edge Detection Showcase
| Before | After |
| ----------- | ----------- |
| ![bike before](images/400px-Bikesgray.jpg) | ![bike after](images/400px-Bikesgray_after.jpg) |
| ![bike before](images/wallhaven-water.png) | ![bike after](images/wallhaven-water_after.png) |
