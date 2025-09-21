// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert
use soroban_sdk::{Address, Env, Symbol};
use crate::schema::UserProfile;
use crate::error::{Error, handle_error};

pub fn user_profile_get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    // Input validation
    // If Address type supports is_empty or similar, add check. Otherwise, skip.
    // For demonstration, assume Address cannot be empty.
    
    // Create the storage key for the user profile
    let key = Symbol::new(env, "profile");
    
    // Get the user profile from storage with proper error handling
    match env
        .storage()
        .instance()
        .get::<(Symbol, Address), UserProfile>(&(key, user_address.clone()))
    {
        Some(profile) => profile,
        None => handle_error(env, Error::UserProfileNotFound),
    }
}

// Function to get user profile with privacy check
// Returns profile only if it's public or if the requester is the profile owner
pub fn get_user_profile_with_privacy(
    env: &Env,
    user_address: Address,
    requester_address: Address,
) -> UserProfile {
    // Reuse the optimized get_user_profile function
    let mut profile = user_profile_get_user_profile(env, user_address.clone());
    
    // Check privacy settings and apply privacy filters without additional storage reads
    if !profile.privacy_public && requester_address != user_address {
        profile.email = None;
        // Add more privacy filters as needed
    }
    profile
}
