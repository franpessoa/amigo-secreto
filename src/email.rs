use std::env;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, AsyncTransport, AsyncSmtpTransport, Tokio1Executor};

pub async fn send(to: String, selected: String) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
    let email = Message::builder()
        .from(std::env::var("SMTP_SENDER").unwrap().parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Teste")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(format!("Parabéns! Você tirou {}", selected)))
        .unwrap();

    let credentials = Credentials::new(
        env::var("SMTP_USERNAME").unwrap().to_string(),
        env::var("SMTP_PASSWORD").unwrap().to_string()
    );

    let tls_param = TlsParameters::builder(env::var("SMTP_RELAY").unwrap().to_owned())
        .build()
        .unwrap();

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&env::var("SMTP_RELAY").unwrap())
        .unwrap()
        .port(2525)
        .tls(Tls::Required(tls_param))
        .credentials(credentials)
        .build();

    mailer.send(email).await
}