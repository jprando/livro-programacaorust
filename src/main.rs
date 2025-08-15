use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, post,
    web::{self, Data},
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let request_id_ctrl = Arc::new(Mutex::new(0u64));
    let app_data = Data::new(request_id_ctrl);
    let app = move || {
        App::new()
            .app_data(app_data.clone())
            .service(get_index)
            .service(post_maior_divisor_comum)
    };

    println!("[teste-web] disponivel em http://localhost:3000");

    HttpServer::new(app).bind(("127.0.0.1", 3000))?.run().await
}

#[derive(Deserialize)]
struct MaiorDivisorComumParametros {
    numero1: u64,
    numero2: u64,
}

#[get("/")]
async fn get_index(request_id: Data<Arc<Mutex<u64>>>) -> impl Responder {
    {
        let mut id = request_id.lock().unwrap();
        *id += 1;
        println!("[{}] 200 GET /", id);
    }
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title> MDC calculadora </title>
            <form action="/maior_divisor_comum" method="post">
                <input type="text" name="numero1" placeholder="informe um numero" />
                <input type="text" name="numero2" placeholder="informe um numero" />
                <button type="submit"> calcular MDC </button>
            </form>
        "#,
    )
}

#[post("/maior_divisor_comum")]
async fn post_maior_divisor_comum(
    request_id: Data<Arc<Mutex<u64>>>,
    form: web::Form<MaiorDivisorComumParametros>,
) -> impl Responder {
    {
        let mut id = request_id.lock().unwrap();
        *id += 1;
        println!("[{}] 200 POST /maior_divisor_comum ", id);
    }
    HttpResponse::Ok().content_type("text/html").body(format!(
        "O maior divisor comum do numero {} e {} eh <b>{}</b>",
        form.numero1,
        form.numero2,
        calcular_maior_divisor_comum(form.numero1, form.numero2)
    ))
}

fn calcular_maior_divisor_comum(mut numero1: u64, mut numero2: u64) -> u64 {
    assert!(numero1 != 0 && numero2 != 0);

    while numero2 != 0 {
        if numero2 < numero1 {
            let t = numero2;
            numero2 = numero1;
            numero1 = t;
        }
        numero2 = numero2 % numero1;
    }

    numero1
}

#[test]
fn test_calcular_maior_divisor_comum() {
    assert_eq!(calcular_maior_divisor_comum(14, 15), 1);

    assert_eq!(calcular_maior_divisor_comum(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11)
}
