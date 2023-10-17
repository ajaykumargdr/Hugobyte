use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct Mail {
    from_name: String,
    to_name: String,
    to_id: String,
    smtp_username: String,
    smtp_password: String,
    smtp_relay: String,
    subject: String,
    body: String,
}

impl Mail {
    pub fn new(
        to_id: &str,
        smtp_username: &str,
        smtp_password: &str,
        smtp_relay: &str,
        subject: &str,
        body: &str,
    ) -> Mail {
        Mail {
            from_name: String::from(""),
            to_name: String::from(""),
            to_id: to_id.to_string(),
            smtp_username: smtp_username.to_string(),
            smtp_password: smtp_password.to_string(),
            smtp_relay: smtp_relay.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        }
    }

    pub fn set_from_name(&mut self, from_name: &str) {
        self.from_name = from_name.to_string()
    }

    pub fn set_to_name(&mut self, to_name: &str) {
        self.to_name = to_name.to_string()
    }

    pub fn send(&self) -> Result<String, std::io::Error> {
        let to_id = format!("{} <{}>", self.to_name, self.to_id);
        let from_id = format!("{} <{}>", self.from_name, self.smtp_username);

        let email = Message::builder()
            .from(from_id.parse().unwrap())
            .to(to_id.parse().unwrap())
            .subject(self.subject.as_str())
            .header(ContentType::TEXT_PLAIN)
            .body(self.body.clone())
            .unwrap();

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&self.smtp_relay)
            .unwrap()
            .credentials(Credentials::new(
                self.smtp_username.clone(),
                self.smtp_password.clone(),
            ))
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => Ok("Email sent successfully!".to_string()),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}

fn main() {

    let smtp_username = std::env::var("smtp_username").unwrap();
    let smtp_password = std::env::var("smtp_password").unwrap();
    let smtp_relay = std::env::var("smtp_relay").unwrap();
    
    let mut mail = Mail::new(
        "werff23rsdf23efc2@example.com",
        &smtp_username,
        &smtp_password,
        &smtp_relay,
        "Aurras Verification",
        "This is your temporary password: XXXXXXXX \n\nNote: Don't Share this with anyone",
    );

    mail.set_from_name("Aurras Team");
    mail.set_to_name("Aurras User");

    mail.send().unwrap();
}
