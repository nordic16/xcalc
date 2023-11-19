/// Mod that handles the ui aspect of the app

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Builder};

/// Starts event loop
pub fn start(app_id: &str) -> glib::ExitCode {
    let app = Application::builder().application_id(app_id).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {

    let src = include_str!("ui.ui");
    let builder = Builder::from_string(src);
    let window = builder.object::<ApplicationWindow>("window").expect("couldn't get window");
    
    window.set_application(Some(app));


    /*
    for rows in 0..3 {
        for cols in 0..3 {
            // Will produce the following output: 1 2 3 | 4 5 6 | 7 8 9
            let pos_x = cols + (3 * rows + 1);

            let button = gtk::Button::builder()
                .label(pos_x.to_string())
                .build();

            grid.attach(&button, cols, rows, 1, 1)
        }
    }
    layout.append(&field);
    layout.append(&grid); 

    window.set_child(Some(&layout));
*/
    // Present window
    
    let grid = builder.object::<gtk::Grid>("num_grid_3").expect("Couldn't find grid");
    let num_field = builder.object::<gtk::TextView>("num_field").expect("Couldn't find num field");

    // There may be a better way to this.    
    for x in grid.observe_children().into_iter() {
        let btn = x.unwrap().dynamic_cast::<gtk::Button>().expect("Couldn't get children.");

        // I really need to find a way to stop using .clone().
        let tmp = num_field.clone();

        btn.connect_clicked(move |y: &gtk::Button| {
            let value = y.label().expect("Couldn't get number from label (for some weird reason)")
                .parse::<i32>().unwrap();

            // Appends text to the buffer.
            let buf = tmp.buffer();
            let text = buf.text(&buf.start_iter(), &buf.end_iter(), true);
            tmp.buffer().set_text(&format!("{}{}", text, value));
        });
    }

    window.present();
}
