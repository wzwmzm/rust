//! Tests that will not be run in CI on OpenCV 4.2.0, 4.5.4 and 3.4.16 due to missing function
#![cfg(ocvrs_opencv_branch_4)]

use opencv::core::{Point2f, RotatedRect, Size2f, Vector};
use opencv::Result;

#[test]
fn rotated_rect_points_vec() -> Result<()> {
	let rect = RotatedRect::new(Point2f::new(100., 100.), Size2f::new(100., 100.), 90.)?;
	let mut vec_pts = [Point2f::new(0.0, 0.0); 4];//Vector::new();
	//rect.points_vec(&mut vec_pts)?;
	rect.points(&mut vec_pts)?;
	assert_eq!(Point2f::new(50., 50.), *vec_pts.get(0).unwrap());//?);
	assert_eq!(Point2f::new(150., 50.), *vec_pts.get(1).unwrap());//?);
	assert_eq!(Point2f::new(150., 150.), *vec_pts.get(2).unwrap());//?);
	assert_eq!(Point2f::new(50., 150.), *vec_pts.get(3).unwrap());//?);

	Ok(())
}
