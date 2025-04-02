use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub id: TicketId,
    pub status: TicketStatus,
}

#[derive(Debug, PartialEq, PartialOrd, Error)]
#[error("Ticket ID mismatch, original: {original_id}, patch: {patch_id}")]
pub struct TicketUpdateError {
    pub original_id: TicketId,
    pub patch_id: TicketId,
}
impl Ticket {
    pub fn new(
        id: TicketId,
        title: TicketTitle,
        description: TicketDescription,
        status: TicketStatus,
    ) -> Self {
        Self {
            id,
            title,
            description,
            status,
        }
    }
    pub fn update(&mut self, patch: TicketPatch) -> Result<(), TicketUpdateError> {
        if self.id != patch.id {
            return Err(TicketUpdateError {
                original_id: self.id,
                patch_id: patch.id,
            });
        }
        if let Some(title) = patch.title {
            self.title = title;
        }
        if let Some(description) = patch.description {
            self.description = description;
        }
        if let Some(status) = patch.status {
            self.status = status;
        };
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TicketPatch {
    pub id: TicketId,
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<TicketStatus>,
}
impl TicketPatch {
    pub fn new(
        id: TicketId,
        title: Option<TicketTitle>,
        description: Option<TicketDescription>,
        status: Option<TicketStatus>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            status,
        }
    }
}

#[derive(Debug, Clone, Error, PartialEq, Serialize, Deserialize)]
pub enum TicketParseError {
    #[error(
        "`{0}` is not a valid status, lowercase should be one of these, todo, inprogress, done"
    )]
    StatusParseError(String),
    #[error("`{0}` is too long, for a title, max bytes is 50 btyes")]
    TitleTooLongError(String),
    #[error("`{0}` is too long, for a description, max bytes is 500 bytes")]
    DebugTooLongError(String),
    #[error("`{0}` is empty, for a title, min bytes is 1 byte")]
    TitleEmptyError(String),
    #[error("`{0}` is empty, for a description, min bytes is 1 byte")]
    DebugEmptyError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TicketStatus {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketId(u64);

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TicketTitle(String);

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TicketDescription(String);

impl TryFrom<&str> for TicketStatus {
    type Error = TicketParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Self::ToDo),
            "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(TicketParseError::StatusParseError(value.to_string())),
        }
    }
}
impl TryFrom<String> for TicketStatus {
    type Error = TicketParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Self::ToDo),
            "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(TicketParseError::StatusParseError(value)),
        }
    }
}
impl TryFrom<&str> for TicketTitle {
    type Error = TicketParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TicketParseError::TitleEmptyError(value.to_string()));
        }
        if value.len() > 50 {
            return Err(TicketParseError::TitleTooLongError(value.to_string()));
        }
        Ok(Self(value.to_string()))
    }
}
impl TryFrom<String> for TicketTitle {
    type Error = TicketParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl TryFrom<&str> for TicketDescription {
    type Error = TicketParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TicketParseError::DebugEmptyError(value.to_string()));
        }
        if value.len() > 500 {
            return Err(TicketParseError::DebugTooLongError(value.to_string()));
        }
        Ok(Self(value.to_string()))
    }
}
impl TryFrom<String> for TicketDescription {
    type Error = TicketParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl From<u64> for TicketId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl Display for TicketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    #[test]
    fn test_ticket_status() {
        assert_eq!(TicketStatus::try_from("todo").unwrap(), TicketStatus::ToDo);
        assert_eq!(
            TicketStatus::try_from("inprogress").unwrap(),
            TicketStatus::InProgress
        );
        assert_eq!(TicketStatus::try_from("done").unwrap(), TicketStatus::Done);
        assert_eq!(
            TicketStatus::try_from("invalid").unwrap_err(),
            TicketParseError::StatusParseError("invalid".to_string())
        );
    }
    #[test]
    fn test_ticket_title() {
        assert_eq!(
            TicketTitle::try_from("valid title").unwrap(),
            TicketTitle("valid title".to_string())
        );
        assert_eq!(
            TicketTitle::try_from("").unwrap_err(),
            TicketParseError::TitleEmptyError("".to_string())
        );
        assert_eq!(
            TicketTitle::try_from("a".repeat(51).as_str()).unwrap_err(),
            TicketParseError::TitleTooLongError("a".repeat(51))
        );
    }
    #[test]
    fn test_ticket_description() {
        assert_eq!(
            TicketDescription::try_from("valid description").unwrap(),
            TicketDescription("valid description".to_string())
        );
        assert_eq!(
            TicketDescription::try_from("").unwrap_err(),
            TicketParseError::DebugEmptyError("".to_string())
        );
        assert_eq!(
            TicketDescription::try_from("a".repeat(501).as_str()).unwrap_err(),
            TicketParseError::DebugTooLongError("a".repeat(501))
        );
    }
    #[test]
    fn test_ticket_id() {
        assert_eq!(TicketId::from(1), TicketId(1));
        assert_eq!(TicketId::from(0), TicketId(0));
        assert_eq!(TicketId::from(u64::MAX), TicketId(u64::MAX));
        assert_eq!(TicketId::try_from(42).unwrap(), TicketId(42));
    }
    #[test]
    fn test_ticket_serde() {
        let ticket = Ticket {
            title: TicketTitle("Test Ticket".to_string()),
            description: TicketDescription("This is a test ticket".to_string()),
            id: TicketId(1),
            status: TicketStatus::ToDo,
        };
        let serialized = serde_json::to_string(&ticket).unwrap();
        let deserialized: Ticket = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ticket, deserialized);
        eprintln!("Serialized: {}", serialized);
    }
    #[test]
    fn test_ticket_patch() {
        let patch = TicketPatch {
            id: TicketId(1),
            title: Some(TicketTitle("Updated Title".to_string())),
            description: None,
            status: Some(TicketStatus::InProgress),
        };
        assert_eq!(patch.id, TicketId(1));
        assert_eq!(
            patch.title.unwrap(),
            TicketTitle("Updated Title".to_string())
        );
        assert_eq!(patch.status.unwrap(), TicketStatus::InProgress);
    }
    #[test]
    fn test_ticket_patch_serde() {
        let patch = TicketPatch {
            id: TicketId(1),
            title: Some(TicketTitle("Updated Title".to_string())),
            description: None,
            status: Some(TicketStatus::InProgress),
        };
        let serialized = serde_json::to_string(&patch).unwrap();
        let deserialized: TicketPatch = serde_json::from_str(&serialized).unwrap();
        assert_eq!(patch, deserialized);
        eprintln!("Serialized: {}", serialized);
    }
    #[test]
    fn test_ticket_patch_serde_no_description() {
        let json_str = r#"{"id":1,"title":"Updated Title","status":"InProgress"}"#;
        let deserialized: TicketPatch = serde_json::from_str(json_str).unwrap();
        assert_eq!(deserialized.id, TicketId(1));
        assert_eq!(
            deserialized.title,
            Some(TicketTitle("Updated Title".to_string()))
        );
        assert_eq!(deserialized.status, Some(TicketStatus::InProgress));
        assert_eq!(deserialized.description, None);
        eprintln!("Deserialized: {:?}", deserialized);
    }
    #[test]
    fn test_update_ticket() {
        let mut ticket = Ticket {
            title: TicketTitle("Original Title".to_string()),
            description: TicketDescription("Original Description".to_string()),
            id: TicketId(1),
            status: TicketStatus::ToDo,
        };
        let patch = TicketPatch {
            id: TicketId(1),
            title: Some(TicketTitle("Updated Title".to_string())),
            description: None,
            status: Some(TicketStatus::InProgress),
        };
        ticket.update(patch).unwrap();
        assert_eq!(ticket.title, TicketTitle("Updated Title".to_string()));
        assert_eq!(ticket.status, TicketStatus::InProgress);
    }
    #[test]
    fn test_update_ticket_id_mismatch() {
        let mut ticket = Ticket {
            title: TicketTitle("Original Title".to_string()),
            description: TicketDescription("Original Description".to_string()),
            id: TicketId(1),
            status: TicketStatus::ToDo,
        };
        let patch = TicketPatch {
            id: TicketId(2),
            title: Some(TicketTitle("Updated Title".to_string())),
            description: None,
            status: Some(TicketStatus::InProgress),
        };
        let result = ticket.update(patch);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TicketUpdateError {
                original_id: TicketId(1),
                patch_id: TicketId(2)
            }
        );
    }
}
