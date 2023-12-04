use anyhow::Result;
use dialoguer::Select;

#[cfg(feature = "imap-backend")]
use crate::imap;
#[cfg(feature = "notmuch-backend")]
use crate::notmuch;
#[cfg(feature = "smtp-sender")]
use crate::smtp;
use crate::{config::wizard::THEME, maildir, sendmail};

use super::{config::BackendConfig, BackendKind};

const DEFAULT_BACKEND_KINDS: &[BackendKind] = &[
    #[cfg(feature = "imap-backend")]
    BackendKind::Imap,
    BackendKind::Maildir,
    #[cfg(feature = "notmuch-backend")]
    BackendKind::Notmuch,
];

const SEND_MESSAGE_BACKEND_KINDS: &[BackendKind] = &[
    #[cfg(feature = "smtp-sender")]
    BackendKind::Smtp,
    BackendKind::Sendmail,
];

pub(crate) async fn configure(account_name: &str, email: &str) -> Result<Option<BackendConfig>> {
    let kind = Select::with_theme(&*THEME)
        .with_prompt("Default email backend")
        .items(DEFAULT_BACKEND_KINDS)
        .default(0)
        .interact_opt()?
        .and_then(|idx| DEFAULT_BACKEND_KINDS.get(idx).map(Clone::clone));

    let config = match kind {
        Some(kind) if kind == BackendKind::Maildir => Some(maildir::wizard::configure()?),
        #[cfg(feature = "imap-backend")]
        Some(kind) if kind == BackendKind::Imap => {
            Some(imap::wizard::configure(account_name, email).await?)
        }
        #[cfg(feature = "notmuch-backend")]
        Some(kind) if kind == BackendKind::Notmuch => Some(notmuch::wizard::configure()?),
        _ => None,
    };

    Ok(config)
}

pub(crate) async fn configure_sender(
    account_name: &str,
    email: &str,
) -> Result<Option<BackendConfig>> {
    let kind = Select::with_theme(&*THEME)
        .with_prompt("Backend for sending messages")
        .items(SEND_MESSAGE_BACKEND_KINDS)
        .default(0)
        .interact_opt()?
        .and_then(|idx| SEND_MESSAGE_BACKEND_KINDS.get(idx).map(Clone::clone));

    let config = match kind {
        Some(kind) if kind == BackendKind::Sendmail => Some(sendmail::wizard::configure()?),
        #[cfg(feature = "smtp-sender")]
        Some(kind) if kind == BackendKind::Smtp => {
            Some(smtp::wizard::configure(account_name, email).await?)
        }
        _ => None,
    };

    Ok(config)
}
