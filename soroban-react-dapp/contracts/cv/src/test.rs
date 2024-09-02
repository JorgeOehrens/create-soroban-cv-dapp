#![no_std]
use soroban_sdk::{test, Address, Env, String, Vec, panic_with_error};

#[test]
fn test_multiple_initializations() {
    let env = Env::default();
    let owner = Address::new("owner1");
    
    CVContract::create_cv(&env, owner.clone(), String::from_str(&env, "John Doe"), String::from_str(&env, "john.doe@example.com"));
    
    let result = panic::catch_unwind(|| {
        CVContract::create_cv(&env, owner.clone(), String::from_str(&env, "John Doe"), String::from_str(&env, "john.doe@example.com"));
    });
    
    assert!(result.is_err(), "CV creation should fail if it already exists");
}

#[test]
fn test_security_concerns() {
    let env = Env::default();
    let owner1 = Address::new("owner1");
    let owner2 = Address::new("owner2");
    
    CVContract::create_cv(&env, owner1.clone(), String::from_str(&env, "John Doe"), String::from_str(&env, "john.doe@example.com"));
    
    let result = panic::catch_unwind(|| {
        CVContract::update_cv(&env, owner1.clone(), Some(String::from_str(&env, "Jane Doe")), None, None, None, None);
    });
    
    assert!(result.is_err(), "Unauthorized update should fail");

    CVContract::create_cv(&env, owner2.clone(), String::from_str(&env, "Alice Smith"), String::from_str(&env, "alice.smith@example.com"));
    CVContract::update_cv(&env, owner2.clone(), Some(String::from_str(&env, "Alice Johnson")), None, None, None, None);
    
    let cv = CVContract::get_cv(&env, owner2.clone());
    assert_eq!(cv.name.to_string(), "Alice Johnson", "Name should be updated for owner2");
}

#[test]
fn test_error_handling() {
    let env = Env::default();
    let owner = Address::new("owner3");

    let result = panic::catch_unwind(|| {
        CVContract::update_cv(&env, owner.clone(), Some(String::from_str(&env, "")), None, None, None, None);
    });
    
    assert!(result.is_err(), "Update should fail if name is empty");

    CVContract::create_cv(&env, owner.clone(), String::from_str(&env, "Bob Lee"), String::from_str(&env, "bob.lee@example.com"));
    let result = panic::catch_unwind(|| {
        CVContract::update_cv(&env, owner.clone(), None, Some(String::from_str(&env, "")), None, None, None);
    });
    
    assert!(result.is_err(), "Update should fail if email is empty");
}

#[test]
fn test_get_cv() {
    let env = Env::default();
    let owner = Address::new("owner4");

    CVContract::create_cv(&env, owner.clone(), String::from_str(&env, "Emma Watson"), String::from_str(&env, "emma.watson@example.com"));
    
    let cv = CVContract::get_cv(&env, owner.clone());
    
    assert_eq!(cv.name.to_string(), "Emma Watson", "Name should match");
    assert_eq!(cv.email.to_string(), "emma.watson@example.com", "Email should match");
}
