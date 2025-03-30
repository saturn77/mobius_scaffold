// src/main.rs.tera

fn main() -> Result<(), eframe::Error> {
    
    env_logger::init();
    

    

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_buttons_shown(true)
            .with_inner_size([900, 800])
            .with_min_inner_size([600, 400])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "egui_treeview",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            let colors = LogColors::load();

            let mut dock_state = DockState::new(vec![
                Tab::new(TabKind::Control, SurfaceIndex::main(), NodeIndex(0)),
                Tab::new(TabKind::About, SurfaceIndex::main(), NodeIndex(1)),
                Tab::new(TabKind::Taffy, SurfaceIndex::main(), NodeIndex(2)),
            ]);

            let [left, _] = dock_state.main_surface_mut().split_right(
                NodeIndex::root(),
                0.3,
                vec![Tab::new(TabKind::Logger, SurfaceIndex::main(), NodeIndex(3))],
            );
            let [_, _] = dock_state.main_surface_mut().split_below(
                left,
                0.7,
                vec![Tab::new(TabKind::Settings, SurfaceIndex::main(), NodeIndex(4))],
            );

            let colors = Arc::new(Mutex::new(colors));
            let state = Arc::new(AppState::new(cc.egui_ctx.clone()));
            let terminal_widget = Dynamic::new(state.terminal_widget.lock().unwrap().clone());

            let mut runtime_manager = RuntimeManager::new(state.clone());
            runtime_manager.start(cc.egui_ctx.clone());

            Ok(Box::new(MyApp {
                dock_state,
                terminal_widget,
                slider_value: 1.0,
                selected_option: 0,
                is_running: false,
                colors,
                state,
                runtime_manager,
            }))
        })
    )
}
