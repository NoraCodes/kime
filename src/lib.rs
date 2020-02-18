//! # Kime
//!
//! *kime*, 決め, noun form of "to decide".
//! In martial arts, an instantaneous tensing at the correct moment during an attack.

pub use crate::{config::Config, editor::Editor, error::Error};

pub mod ansi_escape;
mod config;
mod editor;
mod error;
mod row;
mod syntax;
mod terminal;
