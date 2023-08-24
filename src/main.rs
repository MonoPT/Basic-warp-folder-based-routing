mod routing;
use routing::generate_dynamic_static_routes;
use warp::Filter;

#[tokio::main]
async fn main() ->Result<(), Box<warp::Error>> {
    let routes = generate_dynamic_static_routes("server/pages");

    // Iniciar o servidor com as rotas geradas
    println!("Server is open on addres 127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}
