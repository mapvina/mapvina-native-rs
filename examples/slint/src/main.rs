mod mapvina;

slint::include_modules!();

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

fn main() {
    let ui = MainWindow::new().unwrap();
    let map =
        mapvina::create_map(Size { width: DEFAULT_WIDTH as f32, height: DEFAULT_HEIGHT as f32 });

    mapvina::init(&ui, &map);

    ui.run().unwrap();
}
