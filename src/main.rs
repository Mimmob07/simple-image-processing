use clap::{Parser, Subcommand};
use image::{GenericImageView, Pixel};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    output_path: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Averages rgb values of each pixel
    GrayScale,
    /// Reflects the image horizontally
    Reflect,
    /// Sets rgb values to the average of the pixels around it
    Blur,
    /// Aplies the sobel operator
    Edges,
    /// Uses a different technique to achieve the same result as edges
    EdgesView,
}

fn main() {
    let args = Args::parse();
    let mut img = image::open(args.path).unwrap().to_rgb8();

    match args.command {
        Commands::GrayScale => {
            for pixel in img.pixels_mut() {
                let rgb: Vec<u64> = pixel
                    .channels()
                    .to_vec()
                    .iter()
                    .map(|&x| x as u64)
                    .collect();
                let average: u64 = (rgb[0] + rgb[1] + rgb[2]) / 3;
                *pixel = image::Rgb([average as u8; 3]);
            }
        }
        Commands::Reflect => {
            let width = img.width() - 1;

            for y in 0..img.height() {
                for x in 0..(img.width() / 2) {
                    let buffer = *img.get_pixel(width - x, y);
                    img.put_pixel(width - x, y, *img.get_pixel(x, y));
                    img.put_pixel(x, y, buffer);
                }
            }
        }
        Commands::Blur => {
            for y in 1..img.height() - 1 {
                for x in 1..img.width() - 1 {
                    if x + 3 < img.height() && y + 3 < img.width() {
                        let mut pixel = [0u64; 3];
                        img.view(x - 1, y - 1, 3, 3)
                            .pixels()
                            .for_each(|(_, _, tmp_pixel)| {
                                let values = tmp_pixel.channels().to_vec();
                                pixel[0] += values[0] as u64;
                                pixel[1] += values[1] as u64;
                                pixel[2] += values[2] as u64;
                            });
                        println!("{:?}", pixel);
                        img.put_pixel(
                            x,
                            y,
                            image::Rgb([
                                (pixel[0] / 9) as u8,
                                (pixel[1] / 9) as u8,
                                (pixel[2] / 9) as u8,
                            ]),
                        )
                    }
                }
            }
        }
        Commands::Edges => {
            let gradient_x: [[i16; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
            let gradient_y: [[i16; 3]; 3] = [[-1, -2, -1], [0; 3], [1, 2, 1]];

            let image_copy = img.clone();

            for y in 1..img.height() - 1 {
                for x in 1..img.width() - 1 {
                    let [mut gradient_x_red, mut gradient_x_green, mut gradient_x_blue] = [0i32; 3];
                    let [mut gradient_y_red, mut gradient_y_green, mut gradient_y_blue] = [0i32; 3];

                    for ky in 0..3usize {
                        for kx in 0..3usize {
                            let channels = image_copy
                                .get_pixel(x + kx as u32 - 1, y + ky as u32 - 1)
                                .channels();

                            gradient_x_red += (gradient_x[ky][kx] * channels[0] as i16) as i32;
                            gradient_x_green += (gradient_x[ky][kx] * channels[1] as i16) as i32;
                            gradient_x_blue += (gradient_x[ky][kx] * channels[2] as i16) as i32;

                            gradient_y_red += (gradient_y[ky][kx] * channels[0] as i16) as i32;
                            gradient_y_green += (gradient_y[ky][kx] * channels[1] as i16) as i32;
                            gradient_y_blue += (gradient_y[ky][kx] * channels[2] as i16) as i32;
                        }
                    }

                    let red_magnitude =
                        ((gradient_x_red.pow(2) + gradient_y_red.pow(2)) as f32).sqrt() as u8;
                    let green_magnitude =
                        ((gradient_x_green.pow(2) + gradient_y_green.pow(2)) as f32).sqrt() as u8;
                    let blue_magnitude =
                        ((gradient_x_blue.pow(2) + gradient_y_blue.pow(2)) as f32).sqrt() as u8;

                    img.put_pixel(
                        x,
                        y,
                        image::Rgb([red_magnitude, green_magnitude, blue_magnitude]),
                    )
                }
            }
        }
        Commands::EdgesView => {
            let gradient_x: [Vec<i16>; 3] = [vec![-1, 0, 1], vec![-2, 0, 2], vec![-1, 0, 1]];
            let gradient_y: [Vec<i16>; 3] = [vec![-1, -2, -1], vec![0; 3], vec![1, 2, 1]];

            let image_copy = img.clone();

            for y in 1..img.height() - 1 {
                for x in 1..img.width() - 1 {
                    let [mut gradient_x_red, mut gradient_x_green, mut gradient_x_blue] = [0i32; 3];
                    let [mut gradient_y_red, mut gradient_y_green, mut gradient_y_blue] = [0i32; 3];
                    image_copy
                        .view(x - 1, y - 1, 3, 3)
                        .pixels()
                        .for_each(|(tx, ty, pixel)| {
                            let values = pixel.channels().to_vec();
                            let tmp_x = tx as usize;
                            let tmp_y = ty as usize;

                            gradient_x_red += (gradient_x[tmp_x][tmp_y] * values[0] as i16) as i32;
                            gradient_x_green +=
                                (gradient_x[tmp_x][tmp_y] * values[1] as i16) as i32;
                            gradient_x_blue += (gradient_x[tmp_x][tmp_y] * values[2] as i16) as i32;

                            gradient_y_red += (gradient_y[tmp_x][tmp_y] * values[0] as i16) as i32;
                            gradient_y_green +=
                                (gradient_y[tmp_x][tmp_y] * values[1] as i16) as i32;
                            gradient_y_blue += (gradient_y[tmp_x][tmp_y] * values[2] as i16) as i32;
                        });

                    let red_magnitude =
                        ((gradient_x_red.pow(2) + gradient_y_red.pow(2)) as f32).sqrt() as u8;
                    let green_magnitude =
                        ((gradient_x_green.pow(2) + gradient_y_green.pow(2)) as f32).sqrt() as u8;
                    let blue_magnitude =
                        ((gradient_x_blue.pow(2) + gradient_y_blue.pow(2)) as f32).sqrt() as u8;

                    img.put_pixel(
                        x,
                        y,
                        image::Rgb([red_magnitude, green_magnitude, blue_magnitude]),
                    );
                }
            }
        }
    }

    img.save(args.output_path).unwrap();
}
