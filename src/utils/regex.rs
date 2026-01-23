use std::{borrow::Cow, sync::Arc};

use regex::{Error, Regex};

use crate::settings::ValidationSettings;

pub struct RegexManager {}

impl RegexManager {
	pub fn get_uncached(pattern: impl Into<Cow<'static, str>>) -> Result<Regex, Error> {
		Regex::new(&pattern.into())
	}

	pub fn get_or_create(pattern: impl Into<Cow<'static, str>>) -> Result<Arc<Regex>, Error> {
		let key = pattern.into();
		let cache = ValidationSettings::get_regex_cache();

		if let Some(regex) = cache.get(&key) {
			return Ok(regex.clone());
		}

		let key_for_regex = key.clone();
		match cache
			.entry(key)
			.or_try_insert_with(|| Regex::new(&key_for_regex).map(Arc::new))
		{
			Ok(entry) => Ok(entry.value().clone()),
			Err(arc_erro) => Err((*arc_erro).clone()),
		}
	}

	pub fn remove(pattern: impl Into<Cow<'static, str>>) {
		ValidationSettings::get_regex_cache().remove(&pattern.into());
	}
}
