use std::fs;

use walkdir::WalkDir;
use warp::Filter;

#[derive(Debug, Clone)]
struct Page {
    file_path: String,
    file_name: String,
    file_ext: String,
    route_path: String,
    priority: u64
}

#[tokio::main]
async fn main() ->Result<(), Box<warp::Error>> {
    let pages_dir = "server/pages"; // Pasta onde os arquivos estão

    let mut routes: Vec<Page> = vec![];

    // Get all pages to generate routes for
    for entry in WalkDir::new(pages_dir).into_iter().filter_map(|entry| entry.ok()) {
        if entry.file_type().is_file() {
            let mut file_ext = "";
            
            // Check if page file extension is valid
            if let Some(file_ext_r) = entry.path().extension() {
                file_ext = file_ext_r.to_str().unwrap();

                if file_ext != "page" { continue;}
            } else {
                continue;
            }

            //Mount route
            let file_path = entry.path();
            let file_name = entry.file_name().to_str().unwrap();
            let (_, t1) = file_path.to_str().unwrap().split_at(pages_dir.len() + 1);
            let route_path = t1.trim_end_matches(&format!(".{}", file_ext.to_string()));

            let mut priority = 1;
            let file_path = file_path.to_str().unwrap().to_string();

            let paths = file_path.split("\\");
            paths.for_each(|path| {
                print!("{}", path);
                if path == "index.page" {
                    priority -= 1;
                } else {
                    priority += 1;
                }
            });

            routes.push(Page { 
                file_path, 
                file_name: file_name.to_string(), 
                file_ext: file_ext.to_string(), 
                route_path: route_path.to_string(),
                priority
            });
        }
    }

    routes.sort_by(|a, b| { //Sort order of routes by priority
        a.priority.cmp(&b.priority)
    });

    
    //Generate routes
    let body = r#"
        <html>
            <h1>404 page not found</h1>
        </html>
    "#;
    

    let route = warp::any()
    .map(move || {
        warp::http::Response::builder()
            .status(404)
            .body(body.to_string())
    }).boxed();

    

    let mut routes_f = route;

    
    for page in routes {
        let path = page.route_path;
        let file = page.file_path;

        println!("generating path: {}", path);

        let t = path.split("\\");
        let route = t.fold(warp::any().boxed(), |rt, ro| {
            if ro == "index" {
                rt.and(warp::path::end()).boxed()
            } else {
                rt.and(warp::path(format!("{}", ro))).boxed()
            }            
        });

        let route = warp::any().and(route.map(move || {
            let data = fs::read_to_string(file.clone()).expect("Unable to read file");
            
            warp::http::Response::builder()
                .status(200)
                .body(data)
        }))
        .boxed();

        routes_f = route.or(routes_f).unify().boxed();
    }


    // Iniciar o servidor com as rotas geradas
    println!("Server is open on addres 127.0.0.1:3030");
    warp::serve(routes_f).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}
