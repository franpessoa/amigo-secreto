use std::env;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, AsyncSmtpTransport, AsyncTransport};
use lettre::Tokio1Executor;
use crate::participants::Participant;

pub type EmailResult = Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error>;
pub async fn send(to: String, selected: String) -> EmailResult
{
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

    // return (email.clone(), mailer.send(email).await.map(| x | x.message().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")).map_err(| x | x.to_string()))
    return mailer.send(email).await
}

pub async fn iter_send(p: Vec<Participant>) -> Vec<EmailResult>
{

    let lenght = p.len();
    let p2 = p.clone();
    let mut handles = vec![];

    for i in p.into_iter().enumerate() {
        let selected: String;
        let participant = i.1;

        if i.0 >= lenght - 1 {
            selected = p2[0].name.clone()
        } else {
            selected = p2[i.0 + 1].name.clone()
        }

        handles.push(
            tokio::spawn(async move { send(participant.email.clone(), selected) }).await.unwrap()
        );
    }

    return futures::future::join_all(handles).await;
}