//! 笔记应用主入口
//!
//! 这是 Rust 可执行文件的入口点。
//! 它负责初始化 GPUI 应用框架和 GPUI Component 组件库，
//! 然后启动应用主窗口。

// 引入 GPUI 核心库的所有公共项
// GPUI 是 Zed 编辑器开发的 GPU 加速 UI 框架
use gpui::*;

// 引入应用的主视图
// AppView 是整个应用的根组件
use crate::app::AppView;

// 声明其他模块
// 在 Rust 中，mod 关键字用于声明模块
// 每个模块通常对应一个 .rs 文件

/// 笔记数据模型模块
/// 包含 Note 结构体和相关方法
mod note;

/// 数据存储模块
/// 处理笔记的持久化存储（JSON 文件）
mod storage;

/// 主应用视图模块
/// 整合侧边栏和编辑器，管理全局状态
mod app;

/// 视图组件模块
/// 包含所有 UI 视图（侧边栏、编辑器）
mod views;

/// 应用入口函数
///
/// # 说明
/// - `fn main()` 是 Rust 可执行程序的入口点
/// - 返回 `anyhow::Result<()>` 允许使用 ? 操作符处理错误
/// - 错误会被自动打印到控制台
fn main() -> anyhow::Result<()> {
    // 创建 GPUI 应用实例
    // Application 是 GPUI 的核心类，管理整个应用生命周期
    // with_assets() 配置应用的资源加载器，这里使用 GPUI Component 的默认资源
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    // 启动应用
    // run() 方法接收一个闭包，在应用启动时执行
    // cx: &mut App 是 GPUI 的上下文，用于注册全局状态和打开窗口
    app.run(move |cx: &mut App| {
        // 初始化 GPUI Component
        // 必须在任何组件使用前调用，用于设置主题、字体等全局配置
        gpui_component::init(cx);

        // 打开应用主窗口
        // open_window() 创建一个新窗口，并传入视图初始化闭包
        cx.open_window(
            WindowOptions::default(),
            |_window, cx| {
                // 创建应用主视图
                // cx.new() 创建一个新的视图实体
                // 闭包中接收 window 和 cx，用于初始化视图
                let app_view = cx.new(|cx| {
                    // 创建 AppView 实例
                    // 如果初始化失败（如无法创建数据目录），会 panic
                    AppView::new(cx).expect("初始化应用失败")
                });

                // 返回视图实体，GPUI 将使用它作为窗口的根视图
                app_view
            },
        )
        .expect("创建窗口失败"); // 如果窗口创建失败，panic
    });

    // 应用正常退出
    Ok(())
}

// GPUI 关键概念说明：
//
// 1. Application
//    - GPUI 应用的入口点
//    - 管理全局事件循环和资源
//
// 2. AppContext (cx)
//    - 访问全局状态的上下文
//    - 用于创建实体、订阅事件、打开窗口等
//
// 3. Entity<T>
//    - GPUI 中共享状态的核心类型
//    - 类似 Rc<RefCell<T>>，但专为 UI 优化
//    - 可以通过 cx.new() 创建，通过 .update() 修改
//
// 4. Render trait
//    - 定义视图的渲染逻辑
//    - 每次状态变化都会重新调用 render() 方法
//
// 5. EventEmitter<T>
//    - 使视图可以发出事件
//    - 子视图通过 emit() 发送事件，父视图通过 subscribe() 监听
