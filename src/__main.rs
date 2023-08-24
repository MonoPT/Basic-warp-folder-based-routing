use warp::Filter;

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| warp::reply::html("Hello, World!"));

    // Filtro para a rota "/utilizador"
    let user_route = warp::path("utilizador")
    .map(|| warp::reply::html("Página de Utilizador"));

    // Filtro para a rota "/utilizador/username"
    let user_details_route = warp::path("utilizador")
        .and(warp::path::param())
        .map(|username: String| {
            warp::reply::html(format!("Detalhes do utilizador: {}", username))
        });

    // Combinar os filtros em uma cadeia de filtro
    let routes = root.or(user_details_route).or(user_route).or(warp::fs::dir("assets"));


    // Inicia o servidor.
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
