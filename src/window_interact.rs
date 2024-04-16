extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window, WindowType};


pub async unsafe fn window_interact() {
   // Initialise GTK
   gtk::init().expect("Failed to initialize GTK.");

   // Crée une nouvelle fenêtre
   let window = Window::new(WindowType::Toplevel);
   let aha = "yee";
   window.set_title(&aha);
   window.set_default_size(350, 70);

   // Crée un dialog de message
   let dialog = MessageDialog::new(
       Some(&window),
       DialogFlags::DESTROY_WITH_PARENT,
       MessageType::Info,
       ButtonsType::Ok,
       "Ceci est un message dans une fenêtre en Rust !",
   );

   // Connecte le signal pour fermer la fenêtre lors de la fermeture du dialog
   dialog.connect_response(|dialog, _| {
       dialog.close();
   });

   // Affiche la fenêtre et le dialog
   dialog.show_all();

   // Démarre la boucle principale de l'application
   gtk::main();
}
