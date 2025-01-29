//! Port of code from the tutorial at: https://docs.opencv.org/4.x/dc/dbb/tutorial_py_calibration.html
//! 
//! 图像校准
//! cargo run --example camera_calibration 
//! 读取目录下多个文件

use std::error::Error;
use std::fs;
use opencv::core::{no_array, Point2f, Point3f, Size, TermCriteria, TermCriteria_EPS, TermCriteria_MAX_ITER, Vector};
use opencv::prelude::*;
use opencv::{calib3d, highgui, imgcodecs, imgproc};
use std::env;

fn main() -> Result<(), Box<dyn Error>> {

	// 获取当前工作目录
	let current_dir = env::current_dir().expect("无法获取当前目录");
	// 打印当前工作目录的路径
	println!("当前路径: {}", current_dir.display());

	// 设置校准的终止条件，包括最大迭代次数和最小精度
	let criteria = TermCriteria {
		typ: TermCriteria_EPS + TermCriteria_MAX_ITER, 	// 终止条件类型
		max_count: 30,									// 最大迭代次数
		epsilon: 0.001,									// 精度阈值
	};

	// 准备三维世界中的物体点，例如棋盘格的角点, like (0,0,0), (1,0,0), (2,0,0) ....,(6,5,0)
	let objp_len = 6 * 7;	// 棋盘格的尺寸为6x7
	let objp = Vector::from_iter((0..objp_len).map(|i| Point3f::new(
		(i % 7) as f32, 	// x坐标
		(i / 7) as f32, 	// y坐标
		0.)));				// z坐标，假设棋盘格位于z=0的平面上

	// 读取当前目录下所有.jpg文件
	let images = fs::read_dir("./examples/data")?
		.into_iter()
		.flatten()
		.filter(|entry| entry.path().extension().map_or(false, |ext| ext == "jpg"));

	//对每张图片进行校正
	for image in images {
		// 初始化用于存储所有图像的物体点和图像点的数组
		let mut objpoints = Vector::<Vector<Point3f>>::new(); // 三维世界中的点
		let mut imgpoints = Vector::<Vector<Point2f>>::new(); // 图像平面中的点
		
		// 读取原始图像并显示 
		println!("准备读取图片: {}", image.path().to_string_lossy());
		let mut img = imgcodecs::imread_def(image.path().to_string_lossy().as_ref())?; 
		highgui::imshow("Origin", &img)?;
		highgui::wait_key(5000)?;

		
		let mut gray = Mat::default();
		// 将图像转换为灰度图
		imgproc::cvt_color_def(&img, &mut gray, imgproc::COLOR_BGR2GRAY)?;

		let mut corners = Vector::<Point2f>::default();
		// 在灰度图中查找棋盘格角点
		let ret = calib3d::find_chessboard_corners_def(&gray, Size::new(7, 6), &mut corners)?;
		if ret {
			println!("找到角点, 正在对该图校正");
			// 如果找到了角点，将其添加到物体点数组
			objpoints.push(objp.clone()); 
			// 使用亚像素级精度细化角点位置
			imgproc::corner_sub_pix(&gray, &mut corners, Size::new(11, 11), Size::new(-1, -1), criteria)?;

			// 在图像上绘制角点并显示
			calib3d::draw_chessboard_corners(&mut img, Size::new(7, 6), &corners, ret)?;
			highgui::imshow("Source", &img)?;
			// 将角点添加到图像点数组
			imgpoints.push(corners);

			// 相机校准
			let mut mtx = Mat::default();			// 相机内参矩阵
			let mut dist = Mat::default();			// 畸变系数
			let mut rvecs = Vector::<Mat>::new();	// 旋转向量
			let mut tvecs = Vector::<Mat>::new();	// 平移向量
			calib3d::calibrate_camera_def(
				&objpoints,
				&imgpoints,
				gray.size()?,
				&mut mtx,
				&mut dist,
				&mut rvecs,
				&mut tvecs,
			)?;

			//下面使用两种方法去除图像的畸变：
			// 方法1,使用cv.undistort()去除图像畸变
			let mut dst_undistort = Mat::default();
			calib3d::undistort_def(&img, &mut dst_undistort, &mtx, &dist)?;
			highgui::imshow("Result using undistort", &dst_undistort)?;

			// 方法2,使用remapping方法去除图像畸变
			let mut mapx = Mat::default();
			let mut mapy = Mat::default();
			calib3d::init_undistort_rectify_map(
				&mtx,
				&dist,
				&no_array(),
				&no_array(),
				img.size()?,
				f32::opencv_type(),
				&mut mapx,
				&mut mapy,
			)?;
			let mut dst_remap = Mat::default();
			imgproc::remap_def(&img, &mut dst_remap, &mapx, &mapy, imgproc::INTER_LINEAR)?;
			// 显示使用remap方法去畸变后的结果
			highgui::imshow("Result using remap", &dst_undistort)?;

			// 等待按键，然后继续处理下一张图像
			highgui::wait_key_def()?;
		}else {
			println!("没有找到角点, 该图不能进行校正");
		}
	}
	// 销毁所有OpenCV创建的窗口
	highgui::destroy_all_windows()?;
	Ok(())
}
