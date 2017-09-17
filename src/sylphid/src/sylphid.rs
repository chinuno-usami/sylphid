extern crate rand;
extern crate image;

use std::f32;
use std::slice;
use std::path::Path;
use self::rand::Rng;
use self::image::GenericImage;
use self::image::ImageBuffer;

type Pixel = Vec<u8>;

#[derive(Debug)]
pub struct Sylphid {
    img: Vec<Pixel>,
    result: Vec<Pixel>,
}

fn euclidean(pixel1: &Vec<u8>, pixel2: &Vec<u8>) -> f32 {
    (pixel1.iter()
    .zip(pixel2.iter())
    .fold(0,|acc, x| acc+(*x.0 as i32-*x.1 as i32)
    .pow(2)) as f32)
    .sqrt()
}

fn init_centers(img: &Vec<Pixel>, num: u32) -> Vec<Pixel> {
    let size = img.len();
    let mut centers = Vec::<Pixel>::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num {
        let rand_num = rng.gen::<usize>()%size;
        centers.push(img[rand_num].clone())
    }
    centers
}

fn recenter(pixels: &Vec<Pixel>) -> Pixel {
    let size = pixels.len() as u32;
    pixels.iter().skip(1).fold(
        pixels[0].iter()
        .fold(Vec::new(),|mut acc,x| {acc.push(*x as u32);acc})
        ,|acc, pixel| 
            acc.iter()
            .zip(pixel.iter()
            .fold(Vec::new(),|mut acc,x| {acc.push(*x as u32);acc})
            .iter())
            .map(|x| *x.0 as u32 + *x.1 as u32).collect())
            .iter()
                .map(|x| (*x/size) as u8).collect()
}

impl Sylphid {
    pub fn new() -> Sylphid {
        Sylphid{
            img: Vec::new(),
            result: Vec::new(),
        }
    }

    pub fn result_size(&self) -> usize {
        self.result.len()
    }

    pub fn result_at(&self, index: usize) -> Pixel {
        self.result[index].clone()
    }

    pub fn load<P: image::Pixel<Subpixel=u8>, I: GenericImage<Pixel=P>>(&mut self, image: &I) {
        self.img = image.pixels()
        .fold(Vec::new(),|mut vec, (_,_,x)| {vec.push(x.channels().iter()
        .fold(Vec::new(),|mut pixel, y| {pixel.push(*y);pixel})); vec});
    }

    pub fn load_from_file<P>(&mut self, path: P)  where P: AsRef<Path> {
        self.load(&image::open(&path).unwrap())
    }

    pub fn load_from_raw(&mut self, width: u32, height: u32, buf: *const u8) {
        let buffer = unsafe {
            assert!(!buf.is_null());
            slice::from_raw_parts(buf, (width*height*3) as usize)
        };
        let img:image::ImageBuffer<image::Rgb<u8>,Vec<u8>> =
        ImageBuffer::from_raw(width, height, buffer.to_vec()).unwrap();
        self.load(&img);
    }

    pub fn loaded(&self) -> bool {
        self.img.is_empty()
    }

    pub fn run(&mut self, num: u32, iter_time: u32, min_dist: u32) {
        let mut centers = init_centers(&self.img, num);
        let mut iter_count = 0;
        while iter_count < iter_time {
            let mut clusters: Vec<Vec<Pixel>> = vec![Vec::new();centers.len()];
            for pixel in self.img.iter() {
                let mut min_euc = f32::INFINITY;
                let mut min_idx = 0;
                for (idx,center) in centers.iter().enumerate() {
                    let euc = euclidean(center,&pixel);
                    if euc < min_euc {
                        min_euc = euc;
                        min_idx = idx;
                    }
                }
                clusters[min_idx].push(pixel.clone());
            }
            let mut max_dist = 0.0;
            for (center,cluster) in centers.iter_mut().zip(clusters.iter()) {
                let center_ori = center.clone();
                *center = recenter(&cluster);
                let cur_dist = euclidean(center,&center_ori);
                max_dist = if cur_dist > max_dist {cur_dist} else {max_dist};
            }

            if max_dist < min_dist as f32 {
                break;
            }
            iter_count += 1;
        }
        self.result = centers;
    }
}