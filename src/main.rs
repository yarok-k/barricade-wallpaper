use gtk::gio::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, Application, ApplicationWindow, CssProvider, StyleContext};
use std::cell::RefCell;
use std::rc::Rc;
use ui::Ui;

mod lib;
mod ui;

#[derive(Default)]
struct Wallpapers {
    started: bool,
    path: Option<String>, // последняя обоина, чтобы новые мониторы получали её же
    items: Vec<(gdk::Monitor, ApplicationWindow, Ui)>,
}

fn main() -> Result<(), anyhow::Error> {
    let app = Application::builder()
        .application_id("ru.bst.Barricade.wallpaper")
        .flags(gtk::gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    app.connect_startup(|app| {
        std::mem::forget(app.hold());
    });

    let state = Rc::new(RefCell::new(Wallpapers::default()));

    app.connect_command_line(move |app, cmd| {
        let path = match lib::flags::parse(cmd.arguments(), cmd.cwd()) {
            Ok(p) => p,
            Err(e) => {
                if e.use_stderr() {
                    eprintln!("{e}");
                    return 1;
                }
                println!("{e}");
                return 0;
            }
        };

        let mut st = state.borrow_mut();

        if let Some(p) = &path {
            st.path = Some(p.clone());
        }

        if !st.started {
            // Первый запуск: окна на все текущие мониторы
            st.started = true;
            let display = gdk::Display::default().expect("Нет gdk::Display");

            for i in 0..display.n_monitors() {
                if let Some(m) = display.monitor(i) {
                    add_monitor(app, &mut st, m);
                }
            }

            // Hotplug: подключили монитор — добавляем окно
            let app2 = app.clone();
            let state2 = state.clone();
            display.connect_monitor_added(move |_, m| {
                add_monitor(&app2, &mut state2.borrow_mut(), m.clone());
            });

            // Отключили монитор — закрываем его окно
            let state3 = state.clone();
            display.connect_monitor_removed(move |_, m| {
                state3.borrow_mut().items.retain(|(mon, win, _)| {
                    if mon == m {
                        win.close();
                        false
                    } else {
                        true
                    }
                });
            });
        } else if let Some(p) = &path {
            // Процесс уже запущен: меняем картинку на всех мониторах
            for (_, _, ui) in &st.items {
                if let Err(e) = ui.set_wallpaper(p) {
                    eprintln!("Не удалось загрузить {p}: {e}");
                    return 1;
                }
            }
        }
        0
    });

    app.run();
    Ok(())
}

fn add_monitor(app: &Application, st: &mut Wallpapers, monitor: gdk::Monitor) {
    match create_window(app, &monitor, st.path.as_deref()) {
        Ok((win, ui)) => st.items.push((monitor, win, ui)),
        Err(e) => eprintln!("Ошибка UI: {e}"),
    }
}

fn create_window(
    app: &Application,
    monitor: &gdk::Monitor,
    path: Option<&str>,
) -> Result<(ApplicationWindow, Ui), Box<dyn std::error::Error>> {
    let win = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .build();

    gtk_layer_shell::init_for_window(&win);
    gtk_layer_shell::set_monitor(&win, monitor); // <-- главное отличие
    gtk_layer_shell::set_namespace(&win, "wallpaper");
    gtk_layer_shell::set_layer(&win, gtk_layer_shell::Layer::Background);
    for edge in [
        gtk_layer_shell::Edge::Top,
        gtk_layer_shell::Edge::Right,
        gtk_layer_shell::Edge::Left,
        gtk_layer_shell::Edge::Bottom,
    ] {
        gtk_layer_shell::set_anchor(&win, edge, true);
    }
    gtk_layer_shell::set_exclusive_zone(&win, -1);
    gtk_layer_shell::set_keyboard_interactivity(&win, false);

    let ui = Ui::new(&win, app, path)?;
    win.show_all();
    Ok((win, ui))
}
