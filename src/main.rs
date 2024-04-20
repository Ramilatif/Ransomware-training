extern crate winapi;
mod XOR_encrypt;
mod disk_enum;
mod window_interact;
use disk_enum::disk_enum;
use XOR_encrypt::XOR_encrypt;
use window_interact::window_interact;
use fltk::image::{PngImage};
use fltk::{prelude::*, *,app, enums::{FrameType,Color}, frame::Frame, image::SvgImage, prelude::*, window::Window,text::{TextBuffer, TextDisplay}};
use fltk::window;
use fltk::{button::Button, enums, prelude::*};
use fltk::app::set_font_size;


fn main() {
    unsafe {
            let app = app::App::default();
            let icon_image = PngImage::load("..\\extra\\img\\icon.png").unwrap();
            let mut image = PngImage::load("..\\extra\\img\\background.png").unwrap();
            let (width, height) = (image.w(), image.h());
            let mut wind = window::Window::new(100, 100, width, height, "Osakaware");
           
           

            wind.set_icon(Some(icon_image));


 let mut frame = Frame::default().with_size(width, height);
    frame.set_frame(FrameType::EngravedBox);
  
    frame.draw(move |f| {
        image.draw(f.x(), f.y(), f.w(), f.h());
        draw::set_draw_color(Color::from_u32(0xFFFFFF));
        draw::draw_text("Get it together", f.x() + 350, f.y() + 30);
        
       
        draw::draw_text("Osaka has encrypted your data!!!! If you want to decipher them, give her Chiyo's quilts!!!", f.x() + 50, f.y() + 350);
    }); 
    
    wind.flush();
    wind.end();
    wind.show();

   

    app.run().unwrap();
        
    }
}
