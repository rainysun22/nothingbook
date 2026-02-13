use gpui::*;

use crate::app::AppView;
mod app;
mod note;
mod note_list;
mod storage;
mod views;

fn main() -> anyhow::Result<()> {
    let app = Application::new().with_assets(gpui_component_assets::Assets);
    app.run(|cx| {
        gpui_component::init(cx);
        cx.open_window(WindowOptions::default(), |window, cx| {
            let app_view = cx.new(|cx| AppView::new(cx).expect("初始化应用失败"));
            let root: Entity<gpui_component::Root> =
                cx.new(|cx| gpui_component::Root::new(app_view.clone(), window, cx));
            root
        })
        .expect("创建窗口失败");
    });

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
