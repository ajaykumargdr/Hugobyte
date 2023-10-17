use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::blocking::Client;
use serde_json;

pub struct Mail {
    from_name: String,
    from_id: String,
    to_name: String,
    to_id: String,
    mailing_api_key: String,
    mailing_api_url: String, 
    subject: String,
    body: String,
}

impl Mail {
    pub fn new(
        mailing_api_key: &str,
        mailing_api_url: &str,
        from_id: &str,
        to_id: &str,
        subject: &str,
        body: &str,
    ) -> Mail {
        Mail {
            from_id: from_id.to_string(),
            from_name: String::from(""),
            to_name: String::from(""),
            to_id: to_id.to_string(),
            mailing_api_key: mailing_api_key.to_string(),
            mailing_api_url: mailing_api_url.to_string(),
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

        let json_mail = serde_json::json!
        ({
            "personalizations": [{
                "to": 
            [{
                    "email":self.to_id,
                    "name":self.to_name
                }],
            }],
            "from": 
            {
                "email":self.from_id,
                "name":self.from_name
            },
            "subject": self.subject,
            "content": [{
                "type": "text/plain",
                "value": self.body
                }]
        });
    
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", self.mailing_api_key).parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    
        let _res = Client::builder()
            .default_headers(headers)
            .build().unwrap()
        .post(self.mailing_api_url)
            .body(json_mail.to_string())
            .send();
        
        println!("Email sent. Response: {}", _res.status());

        match _res {
            Ok(_) => Ok("Email sent successfully!".to_string()),
            Err(e) => panic!("Could not send email: {:?}", e),
        }

    }
}

fn main() {

    let mut mail = Mail::new(
        "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",
        "https://api.sendgrid.com/v3/mail/send",
        "ajaykumaroffcl@gmail.com",
        "ajaykumarncas@gmail.com",
        "Aurras Verification",
        "This is your temporary password: XXXXXXXX \n\nNote: Don't Share this with anyone",
    );

    mail.set_from_name("Aurras Team");
    mail.set_to_name("Aurras User");

    mail.send().unwrap();
}
