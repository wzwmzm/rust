//! CUDA（Compute Unified Device Architecture）是由NVIDIA开发的一种并行计算平台和编程模型。
//! 本程序分别使用 opencl 和 CPU 两种方式实现来对比性能表现. 
//! UMat：用于OpenCL实现，利用GPU加速。适用于大规模并行计算
//! Mat： 用于CPU实现，依赖CPU进行计算。适用于小量计算
//! 它们在使用上只有 UMat 与 Mat 的区别
//! 本程序CPU速度优于OPENCL
//! 
//! cargo run --example cuda ./examples/data/lena.jpg
//! 



use std::{env, time};
use time::Instant;

use opencv::{
    core::{self, Size, UMat, UMatUsageFlags, Mat, BORDER_DEFAULT},
    imgcodecs, imgproc,
    prelude::*,
    Result,
};

const ITERATIONS: usize = 1000;

fn main() -> Result<()> {
    let img_file = env::args().nth(1).expect("Please supply image file name");

    // 尝试启用OpenCL
    core::set_use_opencl(true)?;

    let use_opencl = core::use_opencl()?;

    println!("OpenCL is {}", if use_opencl { "enabled" } else { "disabled" });

    let img = imgcodecs::imread(&img_file, imgcodecs::IMREAD_COLOR)?;

    // OpenCL实现
    println!("Timing OpenCL implementation...");
    let img_mat = Mat::from(img.clone());  // 创建Mat实例
    let mut img_um = UMat::new(UMatUsageFlags::USAGE_DEFAULT);  // 创建空的UMat实例
    img_mat.copy_to(&mut img_um)?;  // 将Mat的数据复制到UMat

    let start_opencl = Instant::now();
    for _ in 0..ITERATIONS {
        let mut gray = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
        imgproc::cvt_color(&img_um, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        let mut blurred = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
        imgproc::gaussian_blur(&gray, &mut blurred, Size::new(7, 7), 1.5, 0.0, BORDER_DEFAULT)?;

        let mut edges = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
        imgproc::canny(&blurred, &mut edges, 0., 50., 3, false)?;
    }
    let duration_opencl = start_opencl.elapsed();
    println!("OpenCL implementation took: {:?}", duration_opencl);

    // CPU实现
    println!("Timing CPU implementation...");
    let img_mat = Mat::from(img);  // 创建Mat实例

    let start_cpu = Instant::now();
    for _ in 0..ITERATIONS {
        let mut gray = Mat::default();
        imgproc::cvt_color(&img_mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        let mut blurred = Mat::default();
        imgproc::gaussian_blur(&gray, &mut blurred, Size::new(7, 7), 1.5, 0.0, BORDER_DEFAULT)?;

        let mut edges = Mat::default();
        imgproc::canny(&blurred, &mut edges, 0., 50., 3, false)?;
    }
    let duration_cpu = start_cpu.elapsed();
    println!("CPU implementation took: {:?}", duration_cpu);

    Ok(())
}
