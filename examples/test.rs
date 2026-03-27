use vulnex::{App};

fn main() {
    App::new().run(|app| {
        app.clear(0.5, 0., 0.);
    });
}
