//! Port of https://github.com/opencv/opencv/blob/4.9.0/samples/cpp/warpPerspective_demo.cpp

use std::env;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use opencv::core::{Point2f, Size, Vector};
use opencv::prelude::*;
use opencv::{core, highgui, imgcodecs, imgproc, not_opencv_branch_4, opencv_branch_4};

opencv_branch_4! {
	use opencv::imgproc::LINE_8;
}
not_opencv_branch_4! {
	use opencv::core::LINE_8;
}

fn help() {
	// print a welcome message, and the OpenCV version
	println!();
	println!("This is a demo program shows how perspective transformation applied on an image,");
	println!("Using OpenCV version {}", core::CV_VERSION);
	println!();
	println!(
		"Usage:\n{} [image_name -- Default data/right.jpg]",
		env::args().next().unwrap()
	);
	println!();
	println!("Hot keys:");
	println!("\tESC, q - quit the program");
	println!("\tr - change order of points to rotate transformation");
	println!("\tc - delete selected points");
	println!("\ti - change order of points to inverse transformation");
	println!("\nUse your mouse to select a point and move it to see transformation changes");
}

static WINDOW_TITLE: &str = "Perspective Transformation Demo";
static LABELS: [&str; 4] = ["TL", "TR", "BR", "BL"];

fn main() -> Result<(), Box<dyn Error>> {
	let roi_corners = Arc::new(Mutex::new(Vector::<Point2f>::new()));
	let mut midpoints = [Point2f::default(); 4];
	let mut dst_corners = Vector::<Point2f>::with_capacity(4);
	for _ in 0..4 {
		dst_corners.push(Point2f::default());
	}
	let validation_needed = Arc::new(AtomicBool::new(true));

	help();
	let filename = env::args().nth(1).unwrap_or_else(|| "data/right.jpg".to_string());
	let filename = core::find_file_def(&filename)?;
	let original_image = imgcodecs::imread_def(&filename)?;
	let mut image;
	let original_image_cols = original_image.cols() as f32;
	let original_image_rows = original_image.rows() as f32;
	{
		let mut roi_corners = roi_corners.lock().unwrap();
		roi_corners.push(Point2f::new(original_image_cols / 1.7, original_image_rows / 4.2));
		roi_corners.push(Point2f::new(original_image_cols / 1.15, original_image_rows / 3.32));
		roi_corners.push(Point2f::new(original_image_cols / 1.33, original_image_rows / 1.1));
		roi_corners.push(Point2f::new(original_image_cols / 1.93, original_image_rows / 1.36));
	}
	highgui::named_window(WINDOW_TITLE, highgui::WINDOW_NORMAL)?;
	highgui::named_window_def("Warped Image")?;
	highgui::move_window("Warped Image", 20, 20)?;
	highgui::move_window(WINDOW_TITLE, 330, 20)?;
	highgui::set_mouse_callback(
		WINDOW_TITLE,
		Some(Box::new({
			let mut dragging = false;
			let mut selected_corner_index = 0;
			let roi_corners = Arc::clone(&roi_corners);
			let validation_needed = Arc::clone(&validation_needed);
			move |event, x, y, _flags| {
				let (x, y) = (x as f32, y as f32);
				let mut roi_corners = roi_corners.lock().unwrap();
				// Action when left button is pressed
				if roi_corners.len() == 4 {
					for (i, roi_corner) in roi_corners.iter().enumerate() {
						if event == highgui::EVENT_LBUTTONDOWN && (roi_corner.x - x).abs() < 10. && (roi_corner.y - y).abs() < 10. {
							selected_corner_index = i;
							dragging = true;
						}
					}
				} else if event == highgui::EVENT_LBUTTONDOWN {
					roi_corners.push(Point2f::new(x, y));
					validation_needed.store(true, Ordering::Relaxed);
				}
				// Action when left button is released
				if event == highgui::EVENT_LBUTTONUP {
					dragging = false;
				}
				// Action when left button is pressed and mouse has moved over the window
				if event == highgui::EVENT_MOUSEMOVE && dragging {
					roi_corners.set(selected_corner_index, Point2f::new(x, y)).unwrap();
					validation_needed.store(true, Ordering::Relaxed);
				}
			}
		})),
	)?;
	let mut end_program = false;
	while !end_program {
		let roi_corners_len = roi_corners.lock().unwrap().len();
		if validation_needed.load(Ordering::Relaxed) && roi_corners_len < 4 {
			validation_needed.store(false, Ordering::Relaxed);
			image = original_image.clone();
			{
				let roi_corners = roi_corners.lock().unwrap();
				for (i, roi_corner) in roi_corners.iter().enumerate() {
					let roi_corner = roi_corner.to::<i32>().ok_or("Can't cast to Point")?;
					imgproc::circle(&mut image, roi_corner, 5, (0, 255, 0).into(), 3, LINE_8, 0)?;
					if i > 0 {
						imgproc::line(
							&mut image,
							roi_corners.get(i - 1)?.to::<i32>().ok_or("Can't cast to Point")?,
							roi_corner,
							(0, 0, 255).into(),
							2,
							LINE_8,
							0,
						)?;
						imgproc::circle(&mut image, roi_corner, 5, (0, 255, 0).into(), 3, LINE_8, 0)?;
						imgproc::put_text(
							&mut image,
							LABELS[i],
							roi_corner,
							highgui::QT_FONT_NORMAL,
							0.8,
							(255, 0, 0).into(),
							2,
							LINE_8,
							false,
						)?;
					}
				}
			}
			highgui::imshow(WINDOW_TITLE, &image)?;
		}
		if validation_needed.load(Ordering::Relaxed) && roi_corners_len == 4 {
			image = original_image.clone();
			{
				let roi_corners = roi_corners.lock().unwrap();
				for (i, roi_corner) in roi_corners.iter().enumerate() {
					let roi_corner = roi_corner.to::<i32>().ok_or("Can't cast to Point")?;
					imgproc::line(
						&mut image,
						roi_corner,
						roi_corners.get((i + 1) % 4)?.to::<i32>().ok_or("Can't cast to Point")?,
						(0, 0, 255).into(),
						2,
						LINE_8,
						0,
					)?;
					imgproc::circle(&mut image, roi_corner, 5, (0, 255, 0).into(), 3, LINE_8, 0)?;
					imgproc::put_text(
						&mut image,
						LABELS[i],
						roi_corner,
						highgui::QT_FONT_NORMAL,
						0.8,
						(255, 0, 0).into(),
						2,
						LINE_8,
						false,
					)?;
				}
				highgui::imshow(WINDOW_TITLE, &image)?;

				midpoints[0] = (roi_corners.get(0)? + roi_corners.get(1)?) / 2.;
				midpoints[1] = (roi_corners.get(1)? + roi_corners.get(2)?) / 2.;
				midpoints[2] = (roi_corners.get(2)? + roi_corners.get(3)?) / 2.;
				midpoints[3] = (roi_corners.get(3)? + roi_corners.get(0)?) / 2.;

				dst_corners.set(0, Point2f::new(0., 0.))?;
				dst_corners.set(1, Point2f::new((midpoints[1] - midpoints[3]).norm() as f32, 0.))?;
				dst_corners.set(
					2,
					Point2f::new(dst_corners.get(1)?.x, (midpoints[0] - midpoints[2]).norm() as f32),
				)?;
				dst_corners.set(3, Point2f::new(0., dst_corners.get(2)?.y))?;
				let warped_image_size = Size::new(dst_corners.get(2)?.x.round() as i32, dst_corners.get(2)?.y.round() as i32);
				let roi_corners_mat = Mat::from_slice(roi_corners.as_slice())?;
				let dst_corners_mat = Mat::from_slice(dst_corners.as_slice())?;
				not_opencv_branch_4! {
					let m = imgproc::get_perspective_transform(&roi_corners_mat, &dst_corners_mat)?;
				}
				opencv_branch_4! {
					let m = imgproc::get_perspective_transform_def(&roi_corners_mat, &dst_corners_mat)?;
				}
				let mut warped_image = Mat::default();
				imgproc::warp_perspective_def(&original_image, &mut warped_image, &m, warped_image_size)?; // do perspective transformation
				highgui::imshow("Warped Image", &warped_image)?;
			}
		}
		if let Ok(c) = u8::try_from(highgui::wait_key(10)?) {
			let c = char::from(c);
			if c == 'q' || c == 'Q' || c == '\x1B' {
				end_program = true;
			}
			if c == 'c' || c == 'C' {
				roi_corners.lock().unwrap().clear();
			}
			if c == 'r' || c == 'R' {
				{
					let mut roi_corners = roi_corners.lock().unwrap();
					let t = roi_corners.get(0)?;
					roi_corners.push(t);
					roi_corners.remove(0)?;
				}
			}
			if c == 'i' || c == 'I' {
				{
					let mut roi_corners = roi_corners.lock().unwrap();
					roi_corners.swap(0, 1)?;
					roi_corners.swap(2, 3)?;
				}
			}
		}
	}
	Ok(())
}
