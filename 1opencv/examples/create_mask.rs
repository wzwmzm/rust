//! # 本程序演示如何创建遮罩和如何处理鼠标事件
//! Reference: [opencv/samples/cpp/create_mask.cpp](https://github.com/opencv/opencv/blob/4.9.0/samples/cpp/create_mask.cpp)
//! 
/* 
程序主要逻辑:

1, 注册窗口回调函数highgui::set_mouse_callback(SOURCE_WINDOW, Some(Box::new(mouse_event_dispatcher)))后, 如果在SOURCE_WINDOW窗口内发生鼠标事件, 操作系统会将此事件分发给mouse_event_dispatcher函数处理. 调用这个函数会需要四个参数,即(event: i32, x: i32, y: i32, flags: i32), 这是由操作系统根据实际情况提供.
2, mouse_event_dispatcher函数几乎原封不动地传给&mouse_event_data, 并将should_handle_mouse_event标志置true, 等待程序来处理.
3, 主程序就是一个大循环loop{}. 里面首先就是查询上面的should_handle_mouse_event标志是否为true, 如果不是就进入下一循环, 如果为 true 就去获取*mouse_event_data的值, 并赋给 (mouse_event, x, y, _).
4, 接下来就是根据当前状态和鼠标事件更新绘图状态
drawing_state = state_transform(drawing_state, mouse_event);,然后进入状态机的状态流转.
  */

  //! 给图像建立遮罩
  //! cargo run --example create_mask ./examples/data/lena.jpg
  //! 命令行参数,使用方法示例
  //! 窗口消息,鼠标事件处理
  //! 
  //! 
  //! 
  //! 
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{env, process};

use opencv::core::{bitwise_and, find_file, CommandLineParser, Point, Scalar, Vec3b};
use opencv::highgui::imshow;
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::prelude::*;
use opencv::{highgui, imgproc, not_opencv_branch_4, opencv_branch_4, Result};

// 根据OpenCV版本选择正确的宏
opencv_branch_4! {
	use opencv::imgproc::LINE_8;
}
not_opencv_branch_4! {
	use opencv::core::LINE_8;
}

const SOURCE_WINDOW: &str = "Source image";// 源图像窗口名称常量
// 绘图状态枚举
#[derive(Debug)]
enum DrawingState {
	Init,						// 初始状态
	DrawingMarkerPoint,			// 绘制标记点
	DrawingMarkerPointFinished,	// 标记点绘制完成
	DrawingMask,				// 绘制遮罩
	DrawingMaskFinished,		// 遮罩绘制完成
	Resetting,					// 重置
}

fn main() -> Result<()> {
	// 获取命令行参数
	let args: Vec<String> = env::args().collect();
	let argv = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
	// 创建命令行解析器
	let mut parser = CommandLineParser::new(&argv, "{@input | lena.jpg | input image}")?;
	//设置程序的 about 信息
	parser.about("This program demonstrates using mouse events\n")?;
	//打印程序的 about 信息
	parser.print_message()?;
	println!(
		"\n\tleft mouse button - set a point to create mask shape\n\
            \tright mouse button - create mask from points\n\
            \tmiddle mouse button - reset"
	);

	//获取输入图像路径
	let input_image = argv.into_iter().nth(2).unwrap_or("./examples/data/1.jpg"); //lena.jpg");	
	//校验文件是否存在
	let input_image_path = find_file(input_image, true, false)
		.map(|path| {
			println!("find input_image {} in : {}", input_image, path);
			path
		})
		.unwrap_or_else(|_| panic!("Cannot find input_image: {}", input_image));

	// 初始化图像变量
	let [src, mut next_frame, mut mask, mut final_img]: [Mat; 4];
	//读取图片文件
	src = imread(&input_image_path, IMREAD_COLOR)?;	
	if src.empty() {
		eprintln!("Error opening image: {}", input_image);
		process::exit(-1);
	}
	// 创建源图像窗口
	// 必须加上 highgui::WINDOW_GUI_NORMAL 去除右键菜单,否则鼠标右键会引起混乱
	highgui::named_window(SOURCE_WINDOW, highgui::WINDOW_AUTOSIZE|highgui::WINDOW_GUI_NORMAL)?;
	// 初始化鼠标事件数据
	let mouse_event_data = (highgui::MouseEventTypes::EVENT_MOUSEWHEEL, 0, 0, 0);
	// 创建用于同步的原子布尔和互斥锁
	//移动鼠标和本程序都会处理鼠标事件,所以需要互斥锁
	let (mouse_event_data, should_handle_mouse_event) = (Arc::new(Mutex::new(mouse_event_data)), Arc::new(AtomicBool::new(false)));
	
	// 创建鼠标事件分发器. 定义了一个闭包函数mouse_event_dispatcher, 接受四个参数:event: i32, x: i32, y: i32, flags: i32
	let mouse_event_dispatcher = {
		let mouse_data = Arc::clone(&mouse_event_data);
		let should_handle_mouse_event = Arc::clone(&should_handle_mouse_event);

		move |event: i32, x: i32, y: i32, flags: i32| {		//注意:这里的四个参数其实是mouse_event_dispatcher需要调用的参数
			// can intercept specific mouse events here to don't update the mouse_data
			//尝试将 event 转换成 highgui::MouseEventTypes 枚举
			if let Ok(mouse_event) = highgui::MouseEventTypes::try_from(event) {
				//尝试获取 mouse_data 的互斥锁
				if let Ok(mut mouse_data) = mouse_data.lock() {
					*mouse_data = (mouse_event, x, y, flags);
				}
			}
			should_handle_mouse_event.store(true, Ordering::Relaxed);
		}
	};
	// 设置鼠标回调函数. 将鼠标事件绑定到窗口
	highgui::set_mouse_callback(SOURCE_WINDOW, Some(Box::new(mouse_event_dispatcher))).expect("Cannot set mouse callback");

	//显示最初图片
	imshow(SOURCE_WINDOW, &src)?;
	//初始化标记点和绘图状态 
	let (mut marker_points, mut drawing_state) = (Vec::<Point>::new(), DrawingState::Init);
	// 创建一个与源图像大小相同的黑色图像
	next_frame = Mat::zeros_size(src.size()?, Vec3b::opencv_type())?.to_mat()?;
	// 主事件循环
	loop {
		// Press Esc to exit 如果按下Esc键，则退出循环
		if highgui::wait_key(10)? == 27 {
			break Ok(());
		}
		// 获取并处理鼠标事件
		//
		let (mouse_event, x, y, _) = {	//这一段只是根据取到的mouse_event_data值赋给 (mouse_event, x, y, _)
			if !should_handle_mouse_event.load(Ordering::Relaxed) {
				continue;	// 如果没有鼠标事件需要处理，则继续下一次循环
			} else {
				should_handle_mouse_event.store(false, Ordering::Relaxed);// 重置鼠标事件处理标志

				//下面这句将新变量mouse_event_data绑定到锁上, 并通过解引用获取到值, 并将值赋给了最初的(mouse_event, x, y, _)
				if let Ok(mouse_event_data) = mouse_event_data.lock() {
					*mouse_event_data	// 获取鼠标事件数据					
				} else {
					continue;			// 如果无法获取鼠标事件数据，则继续下一次循环
				}
			}
		};
		// 根据当前状态和鼠标事件更新绘图状态
		drawing_state = state_transform(drawing_state, mouse_event);
		// 根据绘图状态执行不同的操作
		match drawing_state {
			DrawingState::Init | DrawingState::DrawingMarkerPointFinished => { /* do nothing */ }
			DrawingState::DrawingMarkerPoint => {
				// 如果标记点列表为空，则复制源图像到下一帧
				if marker_points.is_empty() {
					next_frame = src.clone();
				}
				// 创建一个新的标记点并添加到列表中
				let point = Point::new(x, y);
				imgproc::circle(
					&mut next_frame, 
					point, 
					2, 
					Scalar::new(0., 0., 255., 0.), 
					-1, 
					LINE_8, 
					0)?;
				marker_points.push(point);
				// 如果标记点列表中有两个以上的点，则绘制线段
				if marker_points.len() > 1 {
					imgproc::line(
						&mut next_frame,
						marker_points[marker_points.len() - 2],
						point,
						Scalar::new(0., 0., 255., 0.),
						2,
						LINE_8,
						0,
					)?;
				}
				// 显示下一帧图像
				imshow(SOURCE_WINDOW, &next_frame)?;
			}
			DrawingState::DrawingMask => {
				// 如果标记点列表不为空，则复制源图像到下一帧并绘制多边形
				if !marker_points.is_empty() {
					next_frame = src.clone();

					imgproc::polylines(
						&mut next_frame,
						&Mat::from_slice(marker_points.as_slice())?,
						true,
						Scalar::new(0., 0., 0., 0.),
						2,
						LINE_8,
						0,
					)?;
					// 显示下一帧图像
					imshow(SOURCE_WINDOW, &next_frame)?;
				}
			}
			DrawingState::DrawingMaskFinished => {
				// 如果标记点列表不为空，则创建遮罩和最终图像
				if !marker_points.is_empty() {
					final_img = Mat::zeros_size(src.size()?, Vec3b::opencv_type())?.to_mat()?;
					mask = Mat::zeros_size(src.size()?, u8::opencv_type())?.to_mat()?;

					imgproc::fill_poly_def(&mut mask, &Mat::from_slice(marker_points.as_slice())?, Scalar::all(255.))?;

					bitwise_and(&src, &src, &mut final_img, &mask)?;// 使用遮罩对源图像进行位运算

					imshow("Mask", &mask)?;				// 显示遮罩图像
					imshow("Result", &final_img)?;		// 显示最终图像
					imshow(SOURCE_WINDOW, &next_frame)?;	// 显示下一帧图像
				}
			}
			DrawingState::Resetting => {
				// 如果标记点列表不为空，则清空列表并复制源图像到下一帧
				if !marker_points.is_empty() {
					marker_points.clear();
					next_frame = src.clone();
					// 显示下一帧图像
					imshow(SOURCE_WINDOW, &next_frame)?;
				}
			}
		}
	}
}

fn state_transform(drawing_state: DrawingState, mouse_event: highgui::MouseEventTypes) -> DrawingState {
	use opencv::highgui::MouseEventTypes::*;

	use self::DrawingState::*;
	// 使用 match 表达式来根据当前绘图状态和鼠标事件类型进行模式匹配
	match (&drawing_state, mouse_event) {
		// 如果当前状态是 Init 并且发生的事件是鼠标左键按下，则转换为 DrawingMarkerPoint 状态
		(Init, EVENT_LBUTTONDOWN) => DrawingMarkerPoint,
		// 如果当前状态是 DrawingMarkerPoint 并且发生的事件是鼠标左键释放，则转换为 DrawingMarkerPointFinished 状态
		(DrawingMarkerPoint, EVENT_LBUTTONUP) => DrawingMarkerPointFinished,
		// 如果当前状态是 DrawingMarkerPointFinished 并且发生的事件是鼠标左键按下，则转换回 DrawingMarkerPoint 状态
		(DrawingMarkerPointFinished, EVENT_LBUTTONDOWN) => DrawingMarkerPoint,
		// 如果当前状态是 DrawingMarkerPointFinished 并且发生的事件是鼠标右键按下，则转换为 DrawingMask 状态
		(DrawingMarkerPointFinished, EVENT_RBUTTONDOWN) => DrawingMask,
		// 如果当前状态是 DrawingMask 并且发生的事件是鼠标右键释放，则转换为 DrawingMaskFinished 状态
		(DrawingMask, EVENT_RBUTTONUP) => DrawingMaskFinished,
		// 如果当前状态是 Init、DrawingMarkerPointFinished 或 DrawingMaskFinished 并且发生的事件是鼠标中键按下，则转换为 Resetting 状态
		(Init | DrawingMarkerPointFinished | DrawingMaskFinished, EVENT_MBUTTONDOWN) => Resetting,
		// 如果当前状态是 Resetting 并且发生的事件是鼠标中键释放，则转换为 Init 状态
		(Resetting, EVENT_MBUTTONUP) => Init,
		// 如果没有匹配到任何上述情况，则打印一条错误信息，并返回当前的绘图状态
		_ => {
			println!(
				"Invalid state transition from {:?} with event {:?}",
				&drawing_state, mouse_event
			);
			drawing_state	// 返回当前状态不变
		}
	}
}
