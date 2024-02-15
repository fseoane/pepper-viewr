// Take a look at the license at the top of the repository in the LICENSE file.

use glib::Cast;

use spice_client_glib::{prelude::*, *};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let session = Session::new();
    if let Some(uri) = args.get(1) {
        session.set_uri(Some(uri));
    } else {
        session.set_host(Some("localhost"));
        session.set_port(Some("5900"));
    }

    session.connect_channel_new(|_, channel| {
        if let Ok(display) = channel.clone().downcast::<DisplayChannel>() {
            ChannelExt::connect(&display);
            display.connect_channel_event(|channel, event| {
                dbg!((channel, event));
            });
            display.connect_gl_scanout_notify(|display| {
                dbg!(display.gl_scanout().unwrap().fd());
            });
            dbg!(display.monitors());
        }
    });

    session.connect();

    let main_context = glib::MainContext::default();
    let main_loop = glib::MainLoop::new(Some(&main_context), false);
    main_loop.run();
}