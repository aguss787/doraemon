use lettre::{SmtpTransport, Transport};
use lettre_email::EmailBuilder;

pub async fn send_activation_mail(
    mut mailer: SmtpTransport,
    origin: String,
    email: String,
    activation_url: String,
) {
    let email = EmailBuilder::new()
        .to(email)
        .from(origin)
        .subject("Activation code")
        .text(format!("Here is your activation code!\n{}", activation_url))
        .build()
        .unwrap();

    let result = mailer.send(email.into());

    if result.is_err() {
        println!("Could not send activation email: {:?}", result);
    }
}

pub fn get_activation_url(base_url: &String, activation_code: &String) -> String {
    base_url.to_owned() + "/activate?code=" + activation_code.as_str()
}
