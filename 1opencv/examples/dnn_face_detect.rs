//! Port of https://github.com/opencv/opencv/blob/4.9.0/samples/dnn/face_detect.cpp
//! Tutorial: https://docs.opencv.org/4.9.0/d0/dd4/tutorial_dnn_face.html

//! 面像检测与识别
//! cargo run --example dnn_face_detect -- --help
//! 
//! # detect on camera input
//! cargo run --example dnn_face_detect
//! 
//! # detect on an image
//! cargo run --example dnn_face_detect -- -i=/path/to/image -v
//! 
//! # get help messages
//! cargo run --example dnn_face_detect -- -h
//! 
//! 1, 需要将两个模型文件拷贝到data文件夹
//! 2, cargo run --example dnn_face_detect -- --fd=./examples/data/face_detection_yunet_2023mar.onnx --fr=./examples/data/face_recognition_sface_2021dec.onnx
//! 
//! 1, 复杂的命令行输入提取
//! 2, 图像叠加文字和几何图形

use std::env; // 导入环境模块，用于获取命令行参数

use objdetect::FaceDetectorYN; // 导入人脸检测器
use opencv::core::{CommandLineParser, Point, Point2f, Rect2f, Size, StsBadArg, StsError, TickMeter}; // 导入 OpenCV 核心模块
use opencv::objdetect::{FaceRecognizerSF, FaceRecognizerSF_DisType}; // 导入人脸识别器
use opencv::prelude::*; // 导入 OpenCV 的预定义模块
use opencv::{core, highgui, imgcodecs, imgproc, objdetect, videoio, Error, Result}; // 导入其他 OpenCV 模块




// 可视化检测到的人脸及其特征
fn visualize(input: &mut Mat, frame: i32, faces: &Mat, fps: f64, thickness: i32) -> Result<()> {
	let fps_string = format!("FPS : {:.2}", fps); // 格式化 FPS 字符串
	if frame >= 0 {
		println!("Frame {}, ", frame); // 打印当前帧数
	}
	println!("FPS: {}", fps_string); // 打印 FPS

	// 遍历检测到的人脸
	for i in 0..faces.rows() {
		// 打印人脸的位置信息和得分
		println!(
			"Face {i}, top-left coordinates: ({}, {}), box width: {}, box height: {}, score: {:.2}",
			faces.at_2d::<f32>(i, 0)?, // 左上角 x 坐标
			faces.at_2d::<f32>(i, 1)?, // 左上角 y 坐标
			faces.at_2d::<f32>(i, 2)?, // 宽度
			faces.at_2d::<f32>(i, 3)?, // 高度
			faces.at_2d::<f32>(i, 14)? // 得分
		);

		// 绘制边界框
		let rect = Rect2f::new(
			*faces.at_2d::<f32>(i, 0)?, // 左上角 x 坐标
			*faces.at_2d::<f32>(i, 1)?, // 左上角 y 坐标
			*faces.at_2d::<f32>(i, 2)?, // 宽度
			*faces.at_2d::<f32>(i, 3)?, // 高度
		)
		.to::<i32>() // 转换为整数
		.ok_or_else(|| Error::new(StsBadArg, "Invalid rect"))?; // 检查矩形有效性
		imgproc::rectangle(input, rect, (0., 255., 0.).into(), thickness, imgproc::LINE_8, 0)?; // 绘制矩形

		// 绘制人脸特征点
		for j in 0..5 {
			let color = match j {
				0 => (255., 0., 0.), // 红色
				1 => (0., 0., 255.), // 蓝色
				2 => (0., 255., 0.), // 绿色
				3 => (255., 0., 255.), // 品红色
				4 => (0., 255., 255.), // 青色
				_ => (0., 0., 0.), // 默认黑色
			};
			imgproc::circle(
				input,
				Point2f::new(*faces.at_2d::<f32>(i, 4 + j * 2)?, *faces.at_2d::<f32>(i, 5 + j * 2)?) // 特征点坐标
					.to::<i32>()
					.ok_or_else(|| Error::new(StsBadArg, "Invalid point"))?,
				2, // 圆的半径
				color.into(), // 圆的颜色
				thickness, // 线条厚度
				imgproc::LINE_8, // 线条类型
				0,
			)?;
		}
	}

	// 在图像上绘制 FPS 信息
	imgproc::put_text(
		input,
		&fps_string, // 要绘制的文本
		Point::new(0, 15), // 文本位置
		imgproc::FONT_HERSHEY_SIMPLEX, // 字体类型
		0.5, // 字体大小
		(0., 255., 0.).into(), // 字体颜色
		thickness, // 线条厚度
		imgproc::LINE_8, // 线条类型
		false, // 是否为斜体
	)?;
	Ok(())
}

fn main() -> Result<()> {
	let args = env::args().collect::<Vec<_>>(); // 获取命令行参数
	let args = args.iter().map(|arg| arg.as_str()).collect::<Vec<_>>(); // 转换为字符串切片
	let parser = CommandLineParser::new(
		&args,
		concat!(
			"{help  h           |            | Print this message}",
			"{image1 i1         |            | Path to the input image1. Omit for detecting through VideoCapture}",
			"{image2 i2         |            | Path to the input image2. When image1 and image2 parameters given then the program try to find a face on both images and runs face recognition algorithm}",
			"{video v           | 0          | Path to the input video}",
			"{scale sc          | 1.0        | Scale factor used to resize input video frames}",
			"{fd_model fd       | face_detection_yunet_2023mar.onnx| Path to the model. Download yunet.onnx in https://github.com/opencv/opencv_zoo/tree/master/models/face_detection_yunet}",
			"{fr_model fr       | face_recognition_sface_2021dec.onnx | Path to the face recognition model. Download the model at https://github.com/opencv/opencv_zoo/tree/master/models/face_recognition_sface}",
			"{score_threshold   | 0.9        | Filter out faces of score < score_threshold}",
			"{nms_threshold     | 0.3        | Suppress bounding boxes of iou >= nms_threshold}",
			"{top_k             | 5000       | Keep top_k bounding boxes before NMS}",
			"{save s            | false      | Set true to save results. This flag is invalid when using camera}"),
	)?;
	
	// 如果用户请求帮助，打印帮助信息并退出
	if parser.has("help")? {
		parser.print_message()?;
		return Ok(());
	}

	// 获取模型路径和参数
	let fd_model_path = parser.get_str_def("fd_model")?; // 人脸检测模型路径
	println!("1******fd_model_path: {}", fd_model_path); // 打印模型路径
	let fr_model_path = parser.get_str_def("fr_model")?; // 人脸识别模型路径
	println!("2******fr_model_path: {}", fr_model_path); // 打印模型路径

	// 获取阈值参数
	let score_threshold = parser.get_f64_def("score_threshold")? as f32; // 得分阈值
	let nms_threshold = parser.get_f64_def("nms_threshold")? as f32; // NMS 阈值
	let top_k = parser.get_i32_def("top_k")?; // 保留的边界框数量

	let save = parser.get_bool_def("save")?; // 是否保存结果
	let scale = parser.get_f64_def("scale")?; // 缩放因子

	// 定义相似度阈值
	let cosine_similar_thresh = 0.363; // 余弦相似度阈值
	let l2norm_similar_thresh = 1.128; // L2 范数相似度阈值

	// 初始化人脸检测器
	let mut detector = FaceDetectorYN::create(
		&fd_model_path, // 模型路径
		"",
		Size::new(320, 320), // 输入图像大小
		score_threshold, // 得分阈值
		nms_threshold, // NMS 阈值
		top_k, // 保留的边界框数量
		0, // 其他参数
		0,
	)?;

	// 打印模型层信息
	let net = opencv::dnn::read_net_from_onnx(&fd_model_path)?; // 从 ONNX 文件读取网络
	for (i, layer_name) in net.get_layer_names()?.iter().enumerate() {
		println!("Layer {}: {}", i, layer_name); // 打印每一层的名称
	}
	

	let mut tm = TickMeter::default()?; // 初始化计时器

	// 如果输入是图片
	if parser.has("image1")? {
		let input1 = parser.get_str_def("image1")?; // 获取第一张图片路径
		let image1 = imgcodecs::imread_def(&input1)?; // 读取图片
		if image1.empty() {
			eprintln!("无法读取图片: {}", input1); // 如果图片为空，打印错误信息
			return Err(Error::new(StsBadArg, "无法读取图片"));
		}

		// 根据缩放因子调整图片大小
		let image_width = (f64::from(image1.cols()) * scale) as i32; // 计算新宽度
		let image_height = (f64::from(image1.rows()) * scale) as i32; // 计算新高度
		let mut image1_out = Mat::default(); // 创建输出图像
		imgproc::resize(
			&image1, // 输入图像
			&mut image1_out, // 输出图像
			Size::new(image_width, image_height), // 新大小
			0., // x 方向缩放因子
			0., // y 方向缩放因子
			imgproc::INTER_LINEAR, // 插值方法
		)?;
		let mut image1 = image1_out; // 更新图像

		tm.start()?; // 开始计时

		// 设置输入大小以进行推理
		detector.set_input_size(image1.size()?)?;

		let mut faces1 = Mat::default(); // 创建存储检测到的人脸的矩阵
		detector.detect(&image1, &mut faces1)?; // 检测人脸
		if faces1.rows() < 1 {
			eprintln!("在 {input1} 中找不到人脸"); // 如果没有检测到人脸，打印错误信息
			return Err(Error::new(StsError, "找不到人脸"));
		}

		tm.stop()?; // 停止计时

		// 在输入图像上绘制结果
		visualize(&mut image1, -1, &faces1, tm.get_fps()?, 2)?; // 可视化检测结果

		// 如果需要保存结果
		if save {
			println!("保存 result.jpg ...");
			imgcodecs::imwrite_def("result.jpg", &image1)?; // 保存结果图像
		}

		// 显示结果
		highgui::imshow("image1", &image1)?; // 显示图像
		highgui::poll_key()?; // 等待用户按键

		// 如果有第二张图片
		if parser.has("image2")? {
			let input2 = parser.get_str_def("image2")?; // 获取第二张图片路径
			let mut image2 = imgcodecs::imread_def(&input2)?; // 读取第二张图片
			if image2.empty() {
				eprintln!("无法读取 image2: {input2}"); // 如果图片为空，打印错误信息
				return Err(Error::new(StsBadArg, "无法读取 image2"));
			}

			tm.reset()?; // 重置计时器
			tm.start()?; // 开始计时
			detector.set_input_size(image2.size()?)?; // 设置输入大小

			let mut faces2 = Mat::default(); // 创建存储检测到的人脸的矩阵
			detector.detect(&image2, &mut faces2)?; // 检测人脸
			if faces2.rows() < 1 {
				eprintln!("在 {input2} 中找不到人脸"); // 如果没有检测到人脸，打印错误信息
				return Err(Error::new(StsError, "找不到人脸"));
			}
			tm.stop()?; // 停止计时
			visualize(&mut image2, -1, &faces2, tm.get_fps()?, 2)?; // 可视化检测结果
			if save {
				println!("保存 result2.jpg ...");
				imgcodecs::imwrite_def("result2.jpg", &image2)?; // 保存结果图像
			}
			highgui::imshow("image2", &image2)?; // 显示图像
			highgui::poll_key()?; // 等待用户按键

			// 初始化人脸识别器
			let mut face_recognizer = FaceRecognizerSF::create_def(&fr_model_path, "")?;

			// 对检测到的第一张人脸进行对齐和裁剪
			let mut aligned_face1 = Mat::default(); // 创建存储对齐人脸的矩阵
			let mut aligned_face2 = Mat::default(); // 创建存储对齐人脸的矩阵
			face_recognizer.align_crop(&image1, &faces1.row(0)?, &mut aligned_face1)?; // 对齐第一张人脸
			face_recognizer.align_crop(&image2, &faces2.row(0)?, &mut aligned_face2)?; // 对齐第二张人脸

			// 进行特征提取
			let mut feature1 = Mat::default(); // 创建存储特征的矩阵
			let mut feature2 = Mat::default(); // 创建存储特征的矩阵
			face_recognizer.feature(&aligned_face1, &mut feature1)?; // 提取第一张人脸特征
			let feature1 = feature1.try_clone()?; // 克隆特征
			face_recognizer.feature(&aligned_face2, &mut feature2)?; // 提取第二张人脸特征
			let feature2 = feature2.try_clone()?; // 克隆特征

			// 进行匹配
			let cos_score = face_recognizer.match_(&feature1, &feature2, FaceRecognizerSF_DisType::FR_COSINE.into())?; // 计算余弦相似度
			let l2_score = face_recognizer.match_(&feature1, &feature2, FaceRecognizerSF_DisType::FR_NORM_L2.into())?; // 计算 L2 范数

			// 根据相似度阈值判断身份
			if cos_score >= cosine_similar_thresh {
				println!("他们是同一个人;");
			} else {
				println!("他们是不同的人;");
			}
			println!(
				"余弦相似度: {cos_score}, 阈值: {cosine_similar_thresh}. (值越高表示相似度越高，最大值为 1.0)"
			);

			if l2_score <= l2norm_similar_thresh {
				println!("他们是同一个人;");
			} else {
				println!("他们是不同的人。");
			}
			println!(
				"L2 范数距离: {l2_score}, 阈值: {l2norm_similar_thresh}. (值越低表示相似度越高，最小值为 0.0)"
			);
		}
		println!("按任意键退出...");
		highgui::wait_key(0)?; // 等待用户按键
	} else {
		// 如果输入是视频
		println!("3===输入是视频");
		let frame_width; // 帧宽
		let frame_height; // 帧高
		let mut capture = videoio::VideoCapture::default()?; // 创建视频捕获对象
		//println!("===OK1");
		let video = parser.get_str_def("video")?; // 获取视频路径
		//println!("===OK2");
		// 检查视频路径是否为数字
		if video.len() == 1 && video.chars().next().map_or(false, |c| c.is_ascii_digit()) {
			println!("4===parser.get_i32_def(video) = {}", parser.get_i32_def("video")?);
			println!("*******下面的警告是摄像头不支持位置查询,可以忽略***********");
			//capture.open_def(0)?;
			capture.open_def(parser.get_i32_def("video")?)?; // 打开视频捕获
			println!("5===打开视频捕获");
		} else {
			capture.open_file_def(&core::find_file_or_keep_def(&video)?)?; // 打开视频文件
		}
		if capture.is_opened()? {
			// 获取视频帧的宽度和高度
			frame_width = (capture.get(videoio::CAP_PROP_FRAME_WIDTH)? * scale) as i32;
			frame_height = (capture.get(videoio::CAP_PROP_FRAME_HEIGHT)? * scale) as i32;
			println!("视频 {video}: 宽度={frame_width}, 高度={frame_height}");
		} else {
			println!("无法初始化视频捕获: {video}"); // 如果无法打开视频，打印错误信息
			return Err(Error::new(StsError, "无法初始化视频捕获"));
		}

		detector.set_input_size(Size::new(frame_width, frame_height))?; // 设置输入大小

		println!("按 'SPACE' 保存帧，按其他任意键退出...");
		let mut n_frame = 0; // 帧计数器
		loop {
			// 获取帧
			println!("*********1,进入loop循环,获取帧*********");
			let mut frame = Mat::default(); // 创建默认帧矩阵
			if !capture.read(&mut frame)? { // 读取帧
				eprintln!("无法抓取帧! 停止"); // 如果无法读取帧，打印错误信息
				break;
			}
			println!("******2,成功抓取帧******");

			let mut frame_out = Mat::default(); // 创建输出帧矩阵
			imgproc::resize_def(&frame, &mut frame_out, Size::new(frame_width, frame_height))?; // 调整帧大小
			let frame = frame_out; // 更新帧
			println!("******3,成功更新帧******");

			println!("Frame type: {:?}", frame.typ());
			println!("Frame channels: {:?}", frame.channels());

			// 进行推理
			let mut faces = Mat::default(); // 创建存储检测到的人脸的矩阵
			tm.start()?; // 开始计时
			println!("******!!!!!!!!!!!!!!!******");
			println!("Input frame size: {:?}", frame.size()?);
			println!("Faces before detection: {:?}", faces.rows());
			detector.detect(&frame, &mut faces)?; // 检测人脸
			println!("Faces detected: {:?}", faces.rows());
			println!("******&&&&&&&&&&&&&&&******");
			tm.stop()?; // 停止计时
			println!("******4,人脸检测结束******");

			let mut result = frame.try_clone()?; // 克隆帧以进行结果显示
			// 在输入图像上绘制结果
			visualize(&mut result, n_frame, &faces, tm.get_fps()?, 2)?; // 可视化检测结果
			println!("******5,可视化人脸检测结果******");

			// 显示结果
			highgui::imshow("Live", &result)?; // 显示实时结果
			let mut key = highgui::wait_key(1)?; // 等待用户按键
			let mut save_frame = save; // 保存帧的标志
			if key == ' ' as i32 { // 如果按下空格键
				save_frame = true; // 设置保存帧标志为真
				key = 0; // 处理空格键
			}
			println!("******6,显示实时结果并等待用户按键******");

			// 如果需要保存帧
			if save_frame {
				let frame_name = format!("frame_{:05}.png", n_frame); // 生成帧文件名
				let result_name = format!("result_{:05}.jpg", n_frame); // 生成结果文件名
				println!("保存 '{frame_name}' 和 '{result_name}' ...");
				imgcodecs::imwrite_def(&frame_name, &frame)?; // 保存帧
				imgcodecs::imwrite_def(&result_name, &result)?; // 保存结果
			}

			n_frame += 1; // 增加帧计数
			if key > 0 { // 如果按下任意键
				break; // 退出循环
			}
		}
		println!("处理了 {n_frame} 帧"); // 打印处理的帧数
	}
	println!("完成."); // 打印完成信息
	Ok(()) // 返回成功
}
