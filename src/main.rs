use std::{fmt::Error, fs::{File, FileType}, io::BufWriter};

use macroquad::file;
use ril::{encodings::png::FilterType, prelude::*};

use savefile::{prelude::*, save_compressed};
use std::string::ToString;


#[macro_use]
extern crate savefile_derive;

const SAVEFILE_VERSION: u32 =  1;

#[derive(Savefile)]
enum ComColor {
    Red,
    Green,
    Blue,
    White,
    Black,
    Yellow,
}
#[derive(Savefile)]
struct CompImage {
    data: Vec<ComColor>,
    height: u16,
    width: u16,
    blur: u8
}

impl CompImage {
fn compress_rli_image(image: Image<ril::Rgb>) -> ril::Result<CompImage> {
    
    let mut file_data: Vec<ComColor> = vec![];
    let mut new_color: ComColor;
    for rgb in image.data.iter() {
        let r = (rgb.r as u32).pow(2);
        let g = (rgb.g as u32).pow(2);
        let b = (rgb.b as u32).pow(2);
        let total = r+g+b;
        let rms = ((total) as f32).sqrt();

        let k = fastrand::u32(0..=total);
        if k < r {
            new_color = ComColor::Red;
        } else if k < g + r {
            new_color = ComColor::Green;
        } else {
            new_color = ComColor::Blue;
        }
        
        if rms < 200.0*fastrand::f32() {
            new_color = ComColor::Black;
        } 
        if rms > 550.0*fastrand::f32() {
            new_color = ComColor::White;
        } 
        if rms > 380.0 && fastrand::f32() > 0.3 {
            if fastrand::f32() < 0.85 * (rms/370.0) {
                new_color = ComColor::White;
            }else {
            new_color = ComColor::Black;
            }
        } 

    let yellow_dif = ((r as f32 - 250.0).powi(3) + (g as f32 - 250.0).powi(3) + (b as f32 - 50.0).powi(3)).sqrt();

    if yellow_dif < 160.0 && yellow_dif * fastrand::f32() < 180.0 {
            new_color = ComColor::Yellow;
        }

        file_data.push(new_color);
    }

    


    Ok(CompImage { data: file_data, 
        height: image.height() as u16, width: image.width()  as u16,
        blur: 4,
    })
}


    fn decompress_to_rli(&self) -> Image<ril::Rgb> {

        let mut image: Image<ril::Rgb> = Image::new(self.width as u32, self.height as u32, Rgb::white());

        for (i, p) in image.data.iter_mut().enumerate() {
            *p = match self.data[i] {
                ComColor::Red => Rgb::from_rgb_tuple((255,0,0)),
                ComColor::Green => Rgb::from_rgb_tuple((0,255,0)),
                ComColor::Blue => Rgb::from_rgb_tuple((0,0,255)),
                ComColor::White => Rgb::white(),
                ComColor::Black => Rgb::black(),
                ComColor::Yellow => Rgb::from_rgb_tuple((255,255,50)),
            }
        }

        // let mut layer2 = image.clone().resized(image.width() / (self.blur as u32 * 4), image.height() / (self.blur as u32 * 4), ResizeAlgorithm::Bilinear);
        // layer2.resize(image.width() * (self.blur as u32 * 2), image.height() * (self.blur as u32 * 2), ResizeAlgorithm::Bilinear);
        
        image.resize(image.width() / self.blur as u32, image.height() / self.blur as u32, ResizeAlgorithm::Hamming);
        image.resize(image.width() * self.blur as u32, image.height() * self.blur as u32, ResizeAlgorithm::Bilinear);
        

        // for (i,p) in image.data.iter_mut().enumerate() {
        //     *p = p.merge_with_alpha(layer2.data[i], 10);
        // }

        return image;

    }


    fn save<T: ToString>(&self, path: T) -> std::io::Result<()> {
        let mut path = path.to_string();
        if path.contains(".") {
            path = path.split(".").next().unwrap().to_string();
        }
        path += ".crunch";
        let mut f =  BufWriter::new(File::create(path)?);
        match save_compressed(&mut f, SAVEFILE_VERSION, self) {
            Ok(_) => {},
            Err(a) => return Err(std::io::Error::other(format!("save_compressed error {a}")))
        }
        Ok(())
    }
}



fn main() -> ril::Result<()> {
    let image: Image<ril::Rgb> = Image::open("face.jpg")?;
    // let image: Image<ril::Rgb> = Image::open("sample.png")?;
    // let image: Image<ril::> = Image::open("sample.png")?;

    // image.resize(image.width()*4, image.height() * 4, ResizeAlgorithm::Hamming);

    let mut comp = CompImage::compress_rli_image(image)?;

    comp.blur = 6;
    // comp.save("out");
    comp.decompress_to_rli().save_inferred("out.png")?;

    Ok(())
}