use std::env;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};
use crate::participantes::Participante;

pub type EmailResult = Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error>;
fn send(to: String, selected: String, name : String) -> EmailResult
{
    let email = Message::builder()
        .from(std::env::var("SMTP_SENDER").unwrap().parse().unwrap())
        .to(to.parse().unwrap())
        .subject("O sorteio foi feito!")
        .header(ContentType::TEXT_HTML)
        .body(format!("Parabéns, {}! Você tirou {} <a style=\"hidden\" href={{unsubscribe}}></a>", name, selected))
        .unwrap();

    let credentials = Credentials::new(
        env::var("SMTP_USERNAME").unwrap().to_string(),
        env::var("SMTP_PASSWORD").unwrap().to_string()
    );

    let tls_param = TlsParameters::builder(env::var("SMTP_RELAY").unwrap().to_owned())
        .build()
        .unwrap();

    let mailer = SmtpTransport::relay(&env::var("SMTP_RELAY").unwrap())
        .unwrap()
        .port(2525)
        .tls(Tls::Required(tls_param))
        .credentials(credentials)
        .build();

    mailer.send(&email)
}

pub fn iter_send(p: Vec<Participante>) -> Vec<Option<lettre::transport::smtp::response::Response>>
{

    let n_participantes = p.len();
    let mut handles = vec![];

    for (idx, participante) in p.iter().enumerate() {
        let selected: String;
        let to = format!("{} <{}>", participante.nome, participante.email);

        if idx >= n_participantes - 1 {
            selected = p[0].nome.clone()
        } else {
            selected = p[idx + 1].nome.clone()
        }

        if participante.email == "_"  || participante.email == "null" {
            handles.push(None)
        } else {
            handles.push(Some(send(to, selected, participante.nome.clone()).unwrap()))
        }
        
    }

    handles
}