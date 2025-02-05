# readme
1. 命令: cargo run --example video_capture
   1. 目录: 项目/examples/video_capture
2. 命令: cargo run --bin main1    
   1. 目录: 项目/src/bin/main1.rs
3. 
//! 图像校准
//! cargo run --example camera_calibration 
//! 1, 读取目录下多个文件

//! 给图像建立遮罩
//! cargo run --example create_mask ./examples/data/lena.jpg
//! 1, 命令行参数,使用方法示例
//! 2, 窗口消息,鼠标事件处理

//! CUDA（Compute Unified Device Architecture）是由NVIDIA开发的一种并行计算平台和编程模型。
//! 本程序分别使用 opencl 和 CPU 两种方式实现来对比性能表现. 
//! UMat：用于OpenCL实现，利用GPU加速。适用于大规模并行计算
//! Mat： 用于CPU实现，依赖CPU进行计算。适用于小量计算
//! 它们在使用上只有 UMat 与 Mat 的区别
//! 本程序CPU速度优于OPENCL//! 
//! cargo run --example cuda ./examples/data/lena.jpg

//! 傅利叶变换 (编译通过,执行出错)
//! cargo run --example discrete_fourier_transform ./examples/data/test.jpg

//! 面像检测与识别
//! cargo run --example dnn_face_detect -- --help//! 
//! # detect on camera input
//! cargo run --example dnn_face_detect//! 
//! # detect on an image
//! cargo run --example dnn_face_detect -- -i=/path/to/image -v//! 
//! # get help messages
//! cargo run --example dnn_face_detect -- -h//! 
//! 1, 需要将两个模型文件拷贝到data文件夹
//! 2, cargo run --example dnn_face_detect -- --fd=./examples/data/face_detection_yunet_2023mar.onnx --fr=./examples/data/face_recognition_sface_2021dec.onnx
//! 
//! 1, 复杂的命令行输入提取
//! 2, 图像叠加文字和几何图形

//! gapi是依靠硬件实现的OPENCV 管线(任务流水线)加速, 本机没有安装, 
//! #[cfg(ocvrs_has_module_gapi)]检测gapi模块是否安装, 并决定后面的代码是否执行地
//! cargo run --example gapi_api_example 

//! 霍夫计算
//! cargo run --example hough_circle -- examples/data/stuff.jpg 
//! 按键 'q' 退出 
//! 本程序特点:
//! 1,如何使用GUI 滑动条,用于输入参数调节的 create_trackbar 

//! 霍夫计算
//! cargo run --example hough_lines -- examples/data/stuff.jpg 

//! 本程序分别使用 opencl 和 CPU 两种方式实现来对比性能表现. 
//! UMat：用于OpenCL实现，利用GPU加速。适用于大规模并行计算
//! Mat： 用于CPU实现，依赖CPU进行计算。适用于小量计算
//! 它们在使用上只有 UMat 与 Mat 的区别
//! 本程序CPU速度优于OPENCL
//! cargo run --example opencl -- examples/data/stuff.jpg 

//! 文本检测
//! cargo run --example text_detection //不完整

//! 摄像头图像串流到 http://127.0.0.1:8080
//! cargo run --example video_capture_http_stream 

//! 摄像头的使用
//! cargo run --example video_capture

//! 面像识别, 人脸检测
//! cargo run --example video_facedetect 

//! 视频彩色转灰色
//!  cargo run --example video_to_gray 

//! 透视变换
//! cargo run --example warp_perspective_demo -- examples/data/stuff.jpg 
//! 1, 设置鼠标回调函数 highgui::set_mouse_callback(...)
//! 2, 处理键盘输入
//! 3, 文字与图形的显示


//! 显示图片在窗口中
//! cargo run --example window 