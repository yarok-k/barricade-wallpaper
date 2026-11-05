use gtk::gdk::prelude::GdkContextExt;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Ui {
    pixbuf: Rc<RefCell<Option<Pixbuf>>>,
    area: gtk::DrawingArea,
}

impl Ui {
    pub fn new(
        window: &gtk::ApplicationWindow,
        _app: &gtk::Application,
        image: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let main_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        window.add(&main_container);

        let pixbuf: Rc<RefCell<Option<Pixbuf>>> = Rc::new(RefCell::new(None));
        let area = gtk::DrawingArea::new();

        let pb = pixbuf.clone();
        area.connect_draw(move |area, cr| {
            if let Some(pixbuf) = pb.borrow().as_ref() {
                let w = area.allocated_width() as f64;
                let h = area.allocated_height() as f64;
                let pw = pixbuf.width() as f64;
                let ph = pixbuf.height() as f64;

                let scale = (w / pw).max(h / ph);
                cr.translate((w - pw * scale) / 2.0, (h - ph * scale) / 2.0);
                cr.scale(scale, scale);
                cr.set_source_pixbuf(pixbuf, 0.0, 0.0);
                let _ = cr.paint();
            }
            gtk::glib::Propagation::Proceed
        });
        main_container.pack_start(&area, true, true, 0);

        let ui = Self { pixbuf, area };
        if let Some(path) = image {
            ui.set_wallpaper(path)?;
        }
        Ok(ui)
    }

    /// Меняет обои на лету
    pub fn set_wallpaper(&self, path: &str) -> Result<(), gtk::glib::Error> {
        let new = Pixbuf::from_file(path)?;
        *self.pixbuf.borrow_mut() = Some(new);
        self.area.queue_draw();
        Ok(())
    }
}
