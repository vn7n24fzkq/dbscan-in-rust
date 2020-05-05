extern crate image;

use image::GenericImageView;
use image::ImageBuffer;
use std::cmp;

fn main() {
    let target = [0, 0, 0, 255];
    let background = [255, 255, 255, 255];
    let distance = 1;
    let min_ptx = 5;
    let img = image::open("me.bmp").unwrap();
    let (width, height) = img.dimensions();
    let mut points = Vec::<Vec<bool>>::new();
    //turn image to vector
    for h in 0..height {
        let mut _p = Vec::<bool>::new();
        for w in 0..width {
            _p.push(&img.get_pixel(w, h).0 == &target);
        }
        points.push(_p);
    }
    let mut k = 0; //cluster count
    let mut curret_point = points.clone();
    let mut imgbuf = image::ImageBuffer::new(width, height); //output image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }
    for i in 0..points.len() {
        for j in 0..points[i].len() {
            if curret_point[i][j] {
                let mut _v = Vec::<[usize; 2]>::new();
                _v.push([i, j]);
                let mut neighbor_count_check = 0;
                while !_v.is_empty() {
                    let p = _v.pop().unwrap();
                    if !curret_point[p[0]][p[1]] {
                        continue;
                    }
                    curret_point[p[0]][p[1]] = false;
                    let start_x = cmp::max(0, p[0] - distance);
                    let start_y = cmp::max(0, p[1] - distance);
                    let end_x = cmp::min(points.len(), p[0] + distance + 1);
                    let end_y = cmp::min(points[0].len(), p[1] + distance + 1);
                    let mut neighbor_count = 0;
                    for _i in start_x..end_x {
                        //check how many neighbors in the distance
                        for _j in start_y..end_y {
                            if get_eculid_distance(_i as i32, _j as i32, p[0] as i32, p[1] as i32)
                                <= distance as f64
                                && points[_i][_j]
                            {
                                neighbor_count += 1;
                                _v.push([_i, _j]);
                            }
                        }
                    }
                    if neighbor_count >= min_ptx {
                        //if neighbor_count_check was hold
                        neighbor_count_check += 1;
                        *imgbuf.get_pixel_mut(p[1] as u32, p[0] as u32) =
                            image::Rgb(get_current_color(k));

                        //P = color
                    }
                }
                if neighbor_count_check > 0 {
                    k += 1;
                }
                curret_point[i][j] = false;
            }
        }
    }
    println!("cluster {}", k);
    imgbuf.save("dbscan.png").unwrap();
}
fn get_eculid_distance(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> f64 {
    return (((start_x - end_x).pow(2) + (start_y - end_y).pow(2)) as f64).sqrt();
}

fn get_current_color(k: i32) -> [u8; 3] {
    let color = (255 / (k + 1)) as u8;
    [color, color, color]
}
