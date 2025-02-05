//! 显示图片在窗口中
//! cargo run --example window 

use opencv::{highgui, imgcodecs, Result};

fn main() -> Result<()> {
	let image = imgcodecs::imread("examples/data/lena.jpg", 0)?;
	highgui::named_window("hello opencv!", 0)?;
	highgui::imshow("hello opencv!", &image)?;
	highgui::wait_key(10000)?;
	Ok(())
}
