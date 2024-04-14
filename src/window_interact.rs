use image::{ DynamicImage, GenericImageView };
use winit::{
    event::{ Event, WindowEvent },
    event_loop::{ ControlFlow, EventLoop },
    window::{ WindowBuilder, Window },
};

pub unsafe fn window_interact() {
    // icon initalization
    let icon_image = image::open("..\\extra\\img\\icon.jpg").unwrap();

    let icon_rgba = icon_image.to_rgba8();
    let dimensions = icon_rgba.dimensions();
    let icon = winit::window::Icon
        ::from_rgba(icon_rgba.into_raw(), dimensions.0, dimensions.1)
        .unwrap();

    //Window initalization
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    window.set_window_icon(Some(icon));
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    //loop to run the window
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                window.set_title("Osakaware");
                window.set_decorations(true);
            }
            _ => (),
        }
    });
}
