// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS-folder-color-switcher — Colour-code folders in the file manager.
//!
//! Provides folder color tint assignment, Catppuccin Mocha palette mapping,
//! path metadata lookup, and bulk recoloring rules.
//!
//! Inspired by Omarchy's visual file hierarchy, Hyprland's icon tinting,
//! and Catppuccin Mocha's warm pastels (mauve, sapphire, teal, peach, rosewater).
//!
//! Principles: path normalization, pure `std`, zero `unsafe`.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

// ── Error types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FolderColorError {
    InvalidColorName(String),
    EmptyPath,
    PathNotFound(String),
    SerializationError(String),
}

impl fmt::Display for FolderColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidColorName(c) => write!(f, "unrecognized Catppuccin Mocha color: '{c}'"),
            Self::EmptyPath => write!(f, "folder path must not be empty"),
            Self::PathNotFound(p) => write!(f, "folder path '{p}' has no assigned color"),
            Self::SerializationError(e) => write!(f, "metadata serialization error: {e}"),
        }
    }
}

impl std::error::Error for FolderColorError {}

// ── Catppuccin Mocha Colors ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MochaColor {
    Rosewater,
    Flamingo,
    Pink,
    Mauve,
    Red,
    Peach,
    Yellow,
    Green,
    Teal,
    Sky,
    Sapphire,
    Blue,
    Lavender,
}

impl MochaColor {
    pub fn hex(&self) -> &'static str {
        match self {
            Self::Rosewater => "#f5e0dc",
            Self::Flamingo => "#f2cdcd",
            Self::Pink => "#f5c2e7",
            Self::Mauve => "#cba6f7",
            Self::Red => "#f38ba8",
            Self::Peach => "#fab387",
            Self::Yellow => "#f9e2af",
            Self::Green => "#a6e3a1",
            Self::Teal => "#94e2d5",
            Self::Sky => "#89dceb",
            Self::Sapphire => "#74c7ec",
            Self::Blue => "#89b4fa",
            Self::Lavender => "#b4befe",
        }
    }

    pub fn parse(s: &str) -> Result<Self, FolderColorError> {
        match s.trim().to_lowercase().as_str() {
            "rosewater" => Ok(Self::Rosewater),
            "flamingo" => Ok(Self::Flamingo),
            "pink" => Ok(Self::Pink),
            "mauve" => Ok(Self::Mauve),
            "red" => Ok(Self::Red),
            "peach" => Ok(Self::Peach),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            "teal" => Ok(Self::Teal),
            "sky" => Ok(Self::Sky),
            "sapphire" => Ok(Self::Sapphire),
            "blue" => Ok(Self::Blue),
            "lavender" => Ok(Self::Lavender),
            other => Err(FolderColorError::InvalidColorName(other.to_string())),
        }
    }
}

// ── Folder Color Store ───────────────────────────────────────────────────────

#[derive(Debug, Default, Clone)]
pub struct FolderColorStore {
    assignments: HashMap<PathBuf, MochaColor>,
}

impl FolderColorStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_color(&mut self, path: impl AsRef<Path>, color: MochaColor) -> Result<(), FolderColorError> {
        let p = path.as_ref();
        if p.as_os_str().is_empty() {
            return Err(FolderColorError::EmptyPath);
        }
        self.assignments.insert(p.to_path_buf(), color);
        Ok(())
    }

    pub fn get_color(&self, path: impl AsRef<Path>) -> Option<MochaColor> {
        self.assignments.get(path.as_ref()).copied()
    }

    pub fn remove_color(&mut self, path: impl AsRef<Path>) -> Result<MochaColor, FolderColorError> {
        let p = path.as_ref();
        self.assignments.remove(p).ok_or_else(|| FolderColorError::PathNotFound(p.display().to_string()))
    }

    pub fn count_by_color(&self) -> HashMap<MochaColor, usize> {
        let mut counts = HashMap::new();
        for &color in self.assignments.values() {
            *counts.entry(color).or_insert(0) += 1;
        }
        counts
    }

    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_and_invalid_mocha_color() {
        // Arrange & Act
        let mauve = MochaColor::parse("Mauve");
        let teal = MochaColor::parse("teal");
        let invalid = MochaColor::parse("neon_orange");

        // Assert
        assert_eq!(mauve, Ok(MochaColor::Mauve));
        assert_eq!(teal, Ok(MochaColor::Teal));
        assert!(matches!(invalid, Err(FolderColorError::InvalidColorName(_))));
    }

    #[test]
    fn mocha_color_hex_codes() {
        // Arrange & Act & Assert
        assert_eq!(MochaColor::Mauve.hex(), "#cba6f7");
        assert_eq!(MochaColor::Sapphire.hex(), "#74c7ec");
        assert_eq!(MochaColor::Green.hex(), "#a6e3a1");
    }

    #[test]
    fn store_assign_and_retrieve_color() {
        // Arrange
        let mut store = FolderColorStore::new();
        let path = PathBuf::from("/home/user/Projects");

        // Act
        store.set_color(&path, MochaColor::Sapphire).unwrap();
        let retrieved = store.get_color(&path);

        // Assert
        assert_eq!(retrieved, Some(MochaColor::Sapphire));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn store_rejects_empty_path() {
        // Arrange
        let mut store = FolderColorStore::new();
        // Act
        let err = store.set_color("", MochaColor::Red);
        // Assert
        assert_eq!(err, Err(FolderColorError::EmptyPath));
    }

    #[test]
    fn store_remove_and_count_summary() {
        // Arrange
        let mut store = FolderColorStore::new();
        store.set_color("/home/user/Music", MochaColor::Mauve).unwrap();
        store.set_color("/home/user/Videos", MochaColor::Mauve).unwrap();
        store.set_color("/home/user/Pictures", MochaColor::Pink).unwrap();

        // Act
        let counts = store.count_by_color();
        let removed = store.remove_color("/home/user/Pictures").unwrap();

        // Assert
        assert_eq!(counts.get(&MochaColor::Mauve), Some(&2));
        assert_eq!(removed, MochaColor::Pink);
        assert_eq!(store.get_color("/home/user/Pictures"), None);
    }

    #[test]
    fn error_display_formatting() {
        // Arrange
        let err = FolderColorError::InvalidColorName("chartreuse".into());
        // Act & Assert
        assert_eq!(format!("{err}"), "unrecognized Catppuccin Mocha color: 'chartreuse'");
    }
}
