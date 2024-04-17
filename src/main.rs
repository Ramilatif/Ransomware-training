extern crate winapi;
mod XOR_encrypt;
mod disk_enum;
mod window_interact;
use disk_enum::disk_enum;
use XOR_encrypt::XOR_encrypt;
use window_interact::window_interact;
use fltk::image::{PngImage};
use fltk::{prelude::*, *};
use fltk::{app, enums::FrameType, frame::Frame, image::SvgImage, prelude::*, window::Window,text::{TextBuffer, TextDisplay}};
use fltk::window;


fn main() {
    unsafe {
            let app = app::App::default();
            let icon_image = PngImage::load("..\\extra\\img\\icon.png").unwrap();
            let mut image = PngImage::load("..\\extra\\img\\background.png").unwrap();
            let (width, height) = (image.w(), image.h());
            let mut text_display = TextDisplay::new(50, 50, 300, 200, "");
             let mut buf = text::TextBuffer::default();

            let mut wind = window::Window::new(100, 100, width, height, "Osakaware");
           
            
           // wind.set_resizable(false);
           

            wind.set_icon(Some(icon_image));


 let mut frame = Frame::default().with_size(width, height);
    frame.set_frame(FrameType::EngravedBox);
  
    frame.draw(move |f| {
        
        
        image.draw(f.x(), f.y(), f.w(), f.h());
    });
   
    wind.flush();
    wind.end();
    wind.show();

    app.run().unwrap();
        
    }
}
