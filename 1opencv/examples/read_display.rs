use opencv::{highgui, imgcodecs, prelude::*};
use std::fs;
use std::error::Error;
use std::env;

fn main() -> std::result::Result<(), Box<dyn Error>>  {
    // 获取当前工作目录
	let current_dir = env::current_dir().expect("无法获取当前目录");
	println!("当前路径: {}", current_dir.display());

    // 加载图片
    let img = imgcodecs::imread_def("./examples/data/1.jpg")?;
    if img.empty() {
        panic!("无法加载图片, 内容为空");
    }
    highgui::imshow("Display window", &img)?;
    // 等待用户按键后退出
    highgui::wait_key(0)?;

    println!("单独显示图片完毕, 下面显示全目录图片");

	// 读取./examples目录下所有.jpg文件
	let images = fs::read_dir("./examples/data")?
		.into_iter()
		.flatten()
		.filter(|entry| entry.path().extension().map_or(false, |ext| ext == "jpg"));

        for image in images {
            println!("准备读取图片: {}", image.path().to_string_lossy());        
            let img = imgcodecs::imread_def(image.path().to_string_lossy().as_ref())?;
            highgui::imshow("Display window", &img)?;
            highgui::wait_key(0)?;
        
        
        
        }



    Ok(())
}