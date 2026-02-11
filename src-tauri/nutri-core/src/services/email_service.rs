use crate::models::AppConfig;
use crate::utils::error::{AppError, AppResult};
use lettre::message::{header, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

pub struct EmailService {
    config: AppConfig,
}

impl EmailService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub async fn send_plan_email(
        &self,
        to: &str,
        subject: &str,
        html_content: String,
    ) -> AppResult<()> {
        if self.config.smtp_host.is_empty() || self.config.smtp_user.is_empty() {
            return Err(AppError::Configuration("SMTP not configured".to_string()));
        }

        let email = Message::builder()
            .from(
                self.config
                    .smtp_user
                    .parse()
                    .map_err(|e| AppError::Configuration(format!("Invalid sender: {}", e)))?,
            )
            .to(to
                .parse()
                .map_err(|e| AppError::Validation(format!("Invalid recipient: {}", e)))?)
            .subject(subject)
            .header(header::ContentType::TEXT_HTML)
            .body(html_content)
            .map_err(|e| AppError::Internal(format!("Failed to build email: {}", e)))?;

        let creds = Credentials::new(
            self.config.smtp_user.clone(),
            self.config.smtp_password.clone(),
        );

        let mailer: AsyncSmtpTransport<Tokio1Executor> = if self.config.smtp_port == 465 {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.config.smtp_host)
                .map_err(|e| AppError::Internal(format!("SMTP relay error: {}", e)))?
                .credentials(creds)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.config.smtp_host)
                .map_err(|e| AppError::Internal(format!("SMTP relay error: {}", e)))?
                .port(self.config.smtp_port)
                .credentials(creds)
                .build()
        };

        mailer
            .send(email)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to send email: {}", e)))?;

        Ok(())
    }
}
