struct User {
    pub _id: u64,
    pub funds: f32,
}

fn get_user_from_db(number: &u64) -> Result<Option<User>, String> {
    let is_ok: bool = rand::random();
    match is_ok {
        true => {
            if rand::random() {
                Ok(Some(User {
                    _id: *number,
                    funds: rand::random(),
                }))
            } else {
                Ok(None)
            }
        }
        false => Err("Could not connect to database".to_string()),
    }
}

fn main() {
    let user_id = 2;
    let user = match get_user_from_db(&user_id) {
        Ok(option) => match option {
            Some(user) => user,
            None => {
                println!("No user found");
                // match has to return here or error will be thrown
                // as match can only return one type and one branch
                // higher we return User struct
                return;
            }
        },
        Err(e) => {
            println!("{}", e);
            // same as above, in case of error we need to return
            // because code below uses user variable which would be
            // uninitialized in this branch
            return;
        }
    };
    println!("User has {}$", user.funds);
}
