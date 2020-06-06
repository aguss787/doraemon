use lettre::{SmtpTransport, Transport};
use lettre_email::EmailBuilder;

pub async fn send_activation_mail(
    mut mailer: SmtpTransport,
    origin: String,
    email: String,
    activation_url: String,
) {
    EmailBuilder::new()
        .to(email)
        .from(origin)
        .subject("Activation code")
        .text(format!("Here is your activation code!\n{}", activation_url))
        .build()
        .map(Some)
        .unwrap_or_else(|err| {
            println!("Could not build activation email: {:?}", err);
            None
        })
        .map(|email| mailer.send(email.into()))
        .and_then(|result| result.err())
        .and_then(|err| {
            println!("Could not send activation email: {:?}", err);
            Some(())
        });
}

pub fn get_activation_url(base_url: &String, activation_code: &String) -> String {
    base_url.to_owned() + "/activate?code=" + activation_code.as_str()
}
