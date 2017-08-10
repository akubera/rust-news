
use pencil;
// use news;

pub fn serve()
{
  let mut app = pencil::Pencil::new(".");
  app.static_folder = "client".into();
//   news::add_news_routes(&mut app);
//   app.enable_static_file_handling();
//   std::thread::spawn(|| {
//     let _ = webbrowser::open("http://127.0.0.1:5000");
//   });
  app.run("127.0.0.1:5000");
}
