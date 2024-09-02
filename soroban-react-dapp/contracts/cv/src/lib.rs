#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
pub struct CV {
    owner: Address,
    name: String,
    email: String,
    skills: Vec<String>,
    experience: Vec<String>,
    education: Vec<String>,
}

#[contract]
pub struct CVContract;

#[contractimpl]
impl CVContract {
    pub fn create_cv(env: Env, owner: Address, name: String, email: String) {

        env.storage().instance().set(&owner, &CV {
            owner: owner.clone(),
            name,
            email,
            skills: Vec::new(&env),
            experience: Vec::new(&env),
            education: Vec::new(&env),
        });
    }

    pub fn update_cv(env: Env, owner: Address, name: Option<String>, email: Option<String>, skills: Option<Vec<String>>, experience: Option<Vec<String>>, education: Option<Vec<String>>) {
        owner.require_auth();

        let mut cv = env.storage().instance().get::<Address, CV>(&owner)
            .unwrap_or(CV {
                owner: owner.clone(),
                name: String::from_str(&env, ""),
                email: String::from_str(&env, ""),
                skills: Vec::new(&env),
                experience: Vec::new(&env),
                education: Vec::new(&env),
            });

        if let Some(n) = name {
            cv.name = n;
        }

        if let Some(e) = email {
            cv.email = e;
        }

        if let Some(s) = skills {
            cv.skills = s;
        }

        if let Some(exp) = experience {
            cv.experience = exp;
        }

        if let Some(edu) = education {
            cv.education = edu;
        }

        env.storage().instance().set(&owner, &cv);
    }

    pub fn get_cv(env: Env, owner: Address) -> CV {
        let empty_cv = CV {
            owner: owner.clone(),
            name: String::from_str(&env, ""),
            email: String::from_str(&env, ""),
            skills: Vec::new(&env),
            experience: Vec::new(&env),
            education: Vec::new(&env),
        };
        env.storage().instance().get::<Address, CV>(&owner).unwrap_or(empty_cv)
    }
}
