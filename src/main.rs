use gtk::gio::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, Application, ApplicationWindow, CssProvider, StyleContext};
use std::cell::RefCell;
use std::rc::Rc;
use ui::Ui;

mod lib; // у вас flags лежит в lib/, оставьте как есть
mod ui;

fn main() -> Result<(), anyhow::Error> {
    let app = Application::builder()
        .application_id("ru.bst.Barricade.wallpaper")
        .flags(gtk::gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    app.connect_startup(|_| load_css());

    // Здесь живёт Ui в первом (основном) процессе
    let ui: Rc<RefCell<Option<Ui>>> = Rc::new(RefCell::new(None));

    app.connect_command_line(move |app, cmd| {
        let path = match lib::flags::parse(cmd.arguments(), cmd.cwd()) {
            Ok(p) => p,
            Err(e) => {
                // --help / --version тоже приходят как "ошибка" clap
                if e.use_stderr() {
                    eprintln!("{e}");
                    return 1;
                }
                println!("{e}");
                return 0;
            }
        };

        let mut ui = ui.borrow_mut();
        if let Some(existing) = ui.as_ref() {
            // Процесс уже запущен: просто меняем картинку
            if let Some(path) = path {
                if let Err(e) = existing.set_wallpaper(&path) {
                    eprintln!("Не удалось загрузить {path}: {e}");
                    return 1;
                }
            }
        } else {
            // Первый запуск: создаём окно
            match create_window(app, path.as_deref()) {
                Ok(new_ui) => *ui = Some(new_ui),
                Err(e) => {
                    eprintln!("Ошибка UI: {e}");
                    return 1;
                }
            }
        }
        0
    });

    app.run();
    Ok(())
}

fn create_window(app: &Application, path: Option<&str>) -> Result<Ui, Box<dyn std::error::Error>> {
    let win = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .build();

    gtk_layer_shell::init_for_window(&win);
    gtk_layer_shell::set_layer(&win, gtk_layer_shell::Layer::Background);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Right, true);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Bottom, true);
    gtk_layer_shell::set_keyboard_interactivity(&win, false);

    let ui = Ui::new(&win, app, path)?;
    win.show_all();
    Ok(ui)
}
fn load_css() {
    let provider = CssProvider::new();
    provider
        .load_from_data(include_bytes!("style.css"))
        .expect("Не удалось загрузить src/style.css");

    StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Нет gdk::Screen"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
