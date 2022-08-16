use actix_web::{web,HttpServer,HttpResponse,App};
use actix_files::{Files};
use serde_json::json;
use handlebars::Handlebars;

async fn home(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let posts = json!({
       
        "posts":[
            {
                "title":"How to generate free energy using Wind",
                "author":"Ramesh Vyas",
                "image_path":"/static/images/wind.jpg"
            },
            {
                "title":"Why Bees are important for human beings",
                "author":"Nirvana",
                "image_path": "/static/images/bee.jpg"
            },
            {
                "title":"Try vegan for your rest life!",
                "author":"Shiva",
                "image_path":"/static/images/calf.jpg"
            },
            
        ]


    });

    let body = hb.render("posts", &posts).unwrap();
    HttpResponse::Ok().body(body)


}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut hbars = Handlebars::new();
    hbars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let hbars_ref = web::Data::new(hbars);

    
    HttpServer::new( move || {
        App::new()
            .app_data(hbars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(home))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}