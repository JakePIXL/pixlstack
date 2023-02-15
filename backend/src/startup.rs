use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::passwordless_client::PasswordlessClient;
use crate::routes::{
    create_category, create_contact_request, get_article, get_articles, get_category, health_check,
    register_user,
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");

        let timeout = configuration.email_client.timeout();

        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let passwordless_timeout = configuration.passwordless.timeout();

        let passwordless_client = PasswordlessClient::new(
            configuration.passwordless.base_url,
            configuration.passwordless.authorization_secret,
            passwordless_timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            connection_pool,
            email_client,
            passwordless_client,
            configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    passwordless_client: PasswordlessClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let passwordless_client = Data::new(passwordless_client);
    let base_url = Data::new(ApplicationBaseUrl(base_url));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/articles")
                            .route("", web::get().to(get_articles))
                            .route("/{slug}", web::get().to(get_article)),
                    )
                    .service(
                        web::scope("/categories")
                            .route("", web::post().to(create_category))
                            .route("/{slug}", web::get().to(get_category)),
                    )
                    .service(web::scope("/contact").route("", web::post().to(create_contact_request)))
                    .service(web::scope("/register").route("", web::post().to(register_user)))
                    .route("/health_check", web::get().to(health_check))
            )
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(passwordless_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
