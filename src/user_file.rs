use crate::*;
use std::{
    fs,
    io::{Read, Write},
};

fn get_user_file_path() -> std::io::Result<std::path::PathBuf> {
    let mut user_file_path = std::env::current_exe()?;
    user_file_path.pop();
    user_file_path.push("user.json");
    Ok(user_file_path)
}

/// Updates the user.json file
pub fn update_user_file(user: &types::user::User) -> std::io::Result<()> {
    let mut user_file = fs::File::create(get_user_file_path()?)?;
    user_file.write_all(&serde_json::to_vec(user).expect("impossible error"))?;
    Ok(())
}

/// Gets the existing data in the user.json file
pub fn load_user_file() -> std::io::Result<types::user::User> {
    let mut buf = Vec::new();
    let user_file_path = get_user_file_path()?;
    fs::File::open(&user_file_path)?.read_to_end(&mut buf)?;
    let user = serde_json::from_slice(&buf);
    if let Ok(user) = user {
        Ok(user)
    } else {
        fs::File::create(user_file_path)?;
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "user.json has a bad format and has been cleared",
        ))
    }
}
