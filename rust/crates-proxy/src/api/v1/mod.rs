use actix_web::dev::{HttpServiceFactory};
use actix_web::{get, HttpResponse, Responder, web};

#[get("/{version}/download")]
async fn download(path: web::Path<(String, String)>) -> impl Responder {
    let (crate_name, version) = path.into_inner();

    let client = crates_io_api::AsyncClient::new(
        "open3pl (bmcwhirter@redhat.com)",
        std::time::Duration::from_millis(1000),
    ).unwrap();

    if let Ok(info) = client.get_crate(&*crate_name).await {
        if info.versions.iter().any(|e| {
            e.num == version
        }) {
            return HttpResponse::Ok().body(
                format!("download {crate_name} {version}")
            );
        }
    }

    HttpResponse::NotFound().body(
        format!("no such crate {crate_name} {version}")
    )
}

pub fn service() -> impl HttpServiceFactory {
    web::scope("/crates/{crate_name}")
        .service(download)
}