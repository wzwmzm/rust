//! Port of https://github.com/opencv/opencv/blob/master/samples/cpp/tutorial_code/core/discrete_fourier_transform/discrete_fourier_transform.cpp

//! 傅利叶变换 (编译通过,执行出错)
//!  cargo run --example discrete_fourier_transform ./examples/data/test.jpg

use std::env;

use opencv::{
	core::{self, Rect, Scalar},
	highgui, imgcodecs,
	prelude::*,
};

fn main() -> opencv::Result<()> {
	#![allow(non_snake_case)]
	// 获取命令行参数
	let filename = env::args().nth(1).expect("Must supply image filename"); // 从命令行获取图像文件名
	// 读取图像
	let I = imgcodecs::imread(&filename, imgcodecs::IMREAD_GRAYSCALE)?; // 以灰度模式读取图像
	if I.empty() {
		panic!("Error opening image: {filename}"); // 检查图像是否成功打开
	}
	// 计算最佳DFT尺寸 
	let mut padded = Mat::default(); // 创建一个默认的矩阵用于填充
	let m = core::get_optimal_dft_size(I.rows())?; // 获取最佳DFT行数
	let n = core::get_optimal_dft_size(I.cols())?; // 获取最佳DFT列数
	// 填充图像到最佳DFT尺寸
	core::copy_make_border_def(&I, &mut padded, 0, m - I.rows(), 0, n - I.cols(), core::BORDER_CONSTANT)?; // 将图像填充到最佳尺寸
	// 创建复数图像	
	let plane_size = padded.size()?; // 获取填充后图像的尺寸
	let mut planes = core::Vector::<Mat>::new(); // 创建一个向量用于存储平面
	let mut padded_f32 = Mat::default(); // 创建一个默认的矩阵用于存储转换后的图像
	padded.convert_to_def(&mut padded_f32, f32::opencv_type())?; // 将填充后的图像转换为浮点型
	planes.push(padded_f32); // 将转换后的图像添加到平面向量中
	planes.push(Mat::zeros_size(plane_size, f32::opencv_type())?.to_mat()?); // 添加一个零矩阵作为虚部
	let mut complexI = Mat::default(); // 创建一个默认的复数矩阵
	core::merge(&planes, &mut complexI)?; // 合并实部和虚部
	let mut complexI_tmp = Mat::default(); // 创建一个临时矩阵用于存储DFT结果
	core::dft_def(&complexI, &mut complexI_tmp)?; // 计算DFT
	complexI = complexI_tmp; // 更新复数矩阵
	core::split(&complexI, &mut planes)?; // 分离实部和虚部
	let mut magI = Mat::default(); // 创建一个默认的矩阵用于存储幅度
	core::magnitude(&planes.get(0)?, &planes.get(1)?, &mut magI)?; // 计算幅度
	let mut magI_tmp = Mat::default(); // 创建一个临时矩阵用于存储幅度结果
	core::add_def(&magI, &Scalar::all(1.), &mut magI_tmp)?; // 幅度加1以避免对数计算中的负值
	magI = magI_tmp; // 更新幅度矩阵
	let mut magI_log = Mat::default(); // 创建一个默认的矩阵用于存储对数幅度
	core::log(&magI, &mut magI_log)?; // 计算对数幅度
	let magI_log_rect = Rect::new(0, 0, magI_log.cols() & -2, magI_log.rows() & -2); // 创建一个矩形区域用于裁剪
	let mut magI = Mat::roi_mut(&mut magI_log, magI_log_rect)?; // 裁剪对数幅度矩阵
	let cx = magI.cols() / 2; // 计算中心点x坐标
	let cy = magI.rows() / 2; // 计算中心点y坐标

	// 打印 cx, cy, magI_log_rect.width 的值
	println!("cx: {}, cy: {}, magI_log_rect.width: {}", cx, cy, magI_log_rect.width);

	let (mut top, mut bottom) = Mat::roi_2_mut( // 将幅度矩阵分为上下两部分
		&mut magI,
		Rect::new(0, 0, magI_log_rect.width, cy),
			Rect::new(0, cy, magI_log_rect.width, magI_log_rect.height.min(cy*2 - cy)),
	)?;
	let (mut q0, mut q1) = Mat::roi_2_mut(&mut top, Rect::new(0, 0, cx, cy), Rect::new(cx, 0, magI_log_rect.width, cy))?; // 将上部分分为左上和右上
	let (mut q2, mut q3) = Mat::roi_2_mut(&mut bottom, Rect::new(0, cy, cx, magI_log_rect.height.min(cy*2 - cy)), Rect::new(cx, cy, magI_log_rect.width, magI_log_rect.height.min(cy*2 - cy)))?; // 确保不超出边界
	let mut tmp = Mat::default(); // 创建一个临时矩阵用于交换
	q0.copy_to(&mut tmp)?; // 将左上部分复制到临时矩阵
	q3.copy_to(&mut q0)?; // 将右下部分复制到左上部分
	tmp.copy_to(&mut q3)?; // 将临时矩阵复制到右下部分
	q1.copy_to(&mut tmp)?; // 将右上部分复制到临时矩阵
	q2.copy_to(&mut q1)?; // 将左下部分复制到右上部分
	tmp.copy_to(&mut q2)?; // 将临时矩阵复制到左下部分
	let mut magI_tmp = Mat::default(); // 创建一个默认的矩阵用于归一化幅度
	core::normalize(&magI, &mut magI_tmp, 0., 1., core::NORM_MINMAX, -1, &core::no_array())?; // 归一化幅度
	let magI = magI_tmp; // 更新幅度矩阵
	highgui::imshow("Input Image", &I)?; // 显示输入图像
	highgui::imshow("spectrum magnitude", &magI)?; // 显示幅度谱
	highgui::wait_key_def()?; // 等待按键
	Ok(())
}
