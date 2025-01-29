//! 摄像头的使用
//! cargo run --example video_capture


use opencv::{highgui, prelude::*, videoio, Result};

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width > 0 {
			highgui::imshow(window, &frame)?;
		}
		let key = highgui::wait_key(10)?;
		if key != -1 { 		//按任意键退出  //ESC键退出 : if key == 27 {
			break;
		}
	}
	Ok(())
}
