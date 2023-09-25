mod types;

use std::{env, path::PathBuf};

use types::{Edges,EdgePoints,Counter,BoundingBox};

use image::{open, GrayImage, Pixel, GenericImageView, ImageError};
use image::imageops::{rotate90, blur, grayscale};

use imageproc::edges::canny;

fn main() -> Result<(), ImageError> {
    let args: Vec<String> = env::args().collect();
    let src = PathBuf::from(&args[1]);
    let dst = PathBuf::from(&args[2]);

    assert!(src.is_file());
    assert!(dst.is_dir());

    process_img(&src, &dst)?;

    Ok( () )
}


fn process_img(src: &PathBuf, dst: &PathBuf) -> Result<(), ImageError> {
	// Open the image, and rotate it 90 degrees
	let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
		= rotate90(&open(src)?);

	// Canny filter pass over the grayscale image
	let procced_img: image::ImageBuffer<image::Luma<u8>, Vec<u8>>
		= canny(
			&blur(
				&grayscale(&img),
				13.0
				),
			7.0, 8.0
		);

	// Find edges in the canny filtered image
	let bounds: BoundingBox = edge_finder(&procced_img)?;

	// Copy a subimage from the SOURCE image, bounded by the edges
	let cropped: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
		= img.view(bounds.x, bounds.y, bounds.width, bounds.height).to_image();
    
    let mut out = dst.clone();
    out.push(src.file_name().unwrap());

    cropped.save(out.as_path())?;

	Ok( () )
}


pub fn edge_finder(img: &GrayImage) -> Result<BoundingBox, ImageError> {
	// Search the inner 2 quarters of the image
	// Increment through the image by 1/200th of the size
	let h_counter: Counter = Counter::new(img.height()/200);
	let v_counter: Counter = Counter::new(img.width()/200);

	let mut edges: Edges = Edges::new(
		h_counter.max(),
		v_counter.max()
	);

	let mut ep: EdgePoints = EdgePoints::new();

	loop {
		// Grab a single row of the image
		let h_img_slice: image::SubImage<&image::ImageBuffer<image::Luma<u8>, Vec<u8>>>
			= img.view(0, h_counter.position.get(), img.width(), 1);

		// Grab a single column of the image
		let v_img_slice: image::SubImage<&image::ImageBuffer<image::Luma<u8>, Vec<u8>>>
			= img.view(v_counter.position.get(), 0, 1, img.height());

		// Reset counters for edgiest points in the img slices
		ep.reset();

		for (x, _y, luma) in h_img_slice.pixels() {
			if luma.channels()[0] != 0 {
				// Set left to the first live pixel
				if ep.left == 0 {
					ep.left = x;
				}

				// Set right to the last live pixel
				ep.right = x;
			}
		}

		for (_x, y, luma) in v_img_slice.pixels() {
			if luma.channels()[0] != 0 {
				// Set left to the first live pixel
				if ep.top == 0 {
					ep.top = y;
				}

				// Set right to the last live pixel
				ep.bottom = y;
			}
		}

		edges.push(&ep); //0547

		h_counter.next();
		v_counter.next();

		if h_counter.done() { break; }
		if v_counter.done() { break; }
	}

	// Return the image bounds
	Ok( edges.to_bounds() )
}

