pub mod app;
pub mod components;
pub mod models;
pub mod pages;
pub mod server_functions;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::*;

        use actix_files::Files;
        use actix_web::*;

        use sqlx::{
            sqlite::SqlitePoolOptions,
            SqlitePool,
            migrate::MigrateDatabase,
            Sqlite
        };

        use leptos::*;
        use leptos_actix::{
            generate_route_list,
            LeptosRoutes,
        };

        const DB_URL: &str = "sqlite://Todos.db";

        pub fn pool(cx: leptos::Scope) -> Result<SqlitePool, ServerFnError> {
            use_context::<SqlitePool>(cx)
                .ok_or("Pool missing.")
                .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))
        }

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./style/output.css").await
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
                println!("Creating database {}", DB_URL);
                match Sqlite::create_database(DB_URL).await {
                    Ok(_) => println!("Create db success"),
                    Err(error) => panic!("error: {}", error),
                }
            }

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(DB_URL).await.unwrap();

            sqlx::migrate!()
                .run(&pool)
                .await
                .expect("could not run SQLx migrations");

            crate::server_functions::register_server_functions();

            let conf = get_configuration(None).await.unwrap();
            let addr = conf.leptos_options.site_addr.clone();

            let routes = generate_route_list(|cx| view! { cx, <App/> });

            HttpServer::new(move || {
                // https://github.com/rust-lang/rfcs/issues/2407
                let pool = pool.clone();
                let pool2 = pool.clone();

                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
                    .app_data(web::Data::new(pool.clone()))
                    .service(css)
                    .route(
                        "/api/{tail:.*}",
                        leptos_actix::handle_server_fns_with_context(move |cx| {
                            provide_context(cx, pool2.clone());
                        })
                    )
                    .leptos_routes_with_context(
                        leptos_options.to_owned(),
                        routes.to_owned(),
                        move |cx| provide_context(cx, pool.clone()),
                        |cx| view! { cx, <App /> }
                    )
                    .service(Files::new("/", &site_root))
                    .wrap(middleware::Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }
    }
    else {
        pub fn main() {}
    }
}
