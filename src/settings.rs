use parking_lot::RwLock;
use std::sync::OnceLock;
#[cfg(feature = "pattern")]
use std::{borrow::Cow, sync::Arc};

#[cfg(feature = "pattern")]
use moka::sync::Cache;
#[cfg(feature = "pattern")]
use regex::Regex;

#[derive(Clone, Copy)]
pub enum FailureMode {
	FailFast,
	FailOncePerField,
	LastFailPerField,
	FullFail,
}

pub struct ValidationSettings {
	pub failure_mode: RwLock<FailureMode>,
	#[cfg(feature = "pattern")]
	pub regex_cache: RwLock<Cache<Cow<'static, str>, Arc<Regex>>>,
}

impl Default for ValidationSettings {
	fn default() -> Self {
		Self {
			failure_mode: RwLock::new(FailureMode::FailOncePerField),
			#[cfg(feature = "pattern")]
			regex_cache: RwLock::new(
				Cache::<Cow<'static, str>, Arc<Regex>>::builder()
					.max_capacity(100)
					.initial_capacity(10)
					.build(),
			),
		}
	}
}

static SETTINGS: OnceLock<ValidationSettings> = OnceLock::new();

impl ValidationSettings {
	pub fn get() -> &'static ValidationSettings {
		SETTINGS.get_or_init(ValidationSettings::default)
	}

	pub fn init(settings: ValidationSettings) -> Result<(), ValidationSettings> {
		SETTINGS.set(settings)
	}

	pub fn set_failure_mode(mode: FailureMode) {
		*Self::get().failure_mode.write() = mode;
	}

	pub fn get_failure_mode() -> FailureMode {
		*Self::get().failure_mode.read()
	}

	#[cfg(feature = "pattern")]
	pub fn set_regex_cache(cache: Cache<Cow<'static, str>, Arc<Regex>>) {
		*Self::get().regex_cache.write() = cache;
	}

	#[cfg(feature = "pattern")]
	pub fn get_regex_cache() -> Cache<Cow<'static, str>, Arc<Regex>> {
		Self::get().regex_cache.read().clone()
	}
}
