// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-iwd: iwd component model.
//!
//! Real implementation: iwd_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IwdItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IwdError {
    EmptyIwdItemName,
    DuplicateIwdItem(String),
    EmptyEventSubject,
}

impl fmt::Display for IwdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIwdItemName => write!(f, "iwd_item name must not be empty"),
            Self::DuplicateIwdItem(id) => write!(f, "duplicate iwd_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for IwdError {}

pub struct Iwd {
    pub name: String,
    iwd_items: BTreeMap<String, IwdItem>,
    events: BTreeMap<String, Event>,
}

impl Iwd {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            iwd_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_iwd_item(&mut self, item: IwdItem) -> Result<(), IwdError> {
        if item.name.trim().is_empty() {
            return Err(IwdError::EmptyIwdItemName);
        }
        let id = item.id.clone();
        if self.iwd_items.contains_key(&id) {
            return Err(IwdError::DuplicateIwdItem(id.clone()));
        }
        self.iwd_items.insert(id, item);
        Ok(())
    }

    pub fn iwd_item_count(&self) -> usize {
        self.iwd_items.len()
    }

    pub fn get_iwd_item(&self, id: &str) -> Option<&IwdItem> {
        self.iwd_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), IwdError> {
        if event.subject.trim().is_empty() {
            return Err(IwdError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_iwd_item(id: &str, name: &str) -> IwdItem {
        IwdItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Iwd::new("Iwd");
        assert_eq!(m.name, "Iwd");
        assert_eq!(m.iwd_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_iwd_item() {
        let mut m = Iwd::new("Iwd");
        m.add_iwd_item(sample_iwd_item("a1", "Alpha")).unwrap();
        assert_eq!(m.iwd_item_count(), 1);
        assert!(m.get_iwd_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Iwd::new("Iwd");
        assert!(m.add_iwd_item(sample_iwd_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Iwd::new("Iwd");
        m.add_iwd_item(sample_iwd_item("x", "One")).unwrap();
        assert!(m.add_iwd_item(sample_iwd_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Iwd::new("Iwd");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
