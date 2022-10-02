use std::{env, time::Duration};

use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use rand::{thread_rng, Rng};

fn rand_secret() -> Vec<u8> {
	let mut rng = thread_rng();
	let mut arr = [0u8; 128];
	rng.fill(&mut arr);
	arr.to_vec()
}

pub fn get_session_layer() -> SessionLayer<MemoryStore> {
	let store = MemoryStore::new();

	let secret = env::var("SESSION_SECRET")
		.map(|s| s.into_bytes())
		.unwrap_or_else(|_| rand_secret());

	SessionLayer::new(store, &secret)
		.with_cookie_name("stump_session")
		.with_session_ttl(Some(Duration::from_secs(3600 * 24 * 3)))
		.with_cookie_path("/")
		.with_same_site_policy(SameSite::Lax)
		.with_secure(false)
}