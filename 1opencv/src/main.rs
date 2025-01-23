use opencv::{highgui, imgcodecs, prelude::*, Result};

fn main() -> Result<()> {
    // 初始化OpenCV
    //let window = "Display window";

    // 加载图片
    let img = imgcodecs::imread_def("./src/1.jpg")?;//, imgcodecs::IMREAD_COLOR)?;

    // 检查图片是否加载成功
    if img.empty() {
        panic!("无法加载图片");
    }

    // 创建窗口
    //highgui::named_window("Display window", highgui::WINDOW_AUTOSIZE)?;

    // 显示图片
    highgui::imshow("Display window", &img)?;

    // 等待用户按键后退出
    highgui::wait_key(0)?;

    Ok(())
}