

struct AdminUser {
    username: String,
    password: String
}
struct User {
    username: String,
    password: String
}

trait CanEdit {
    fn edit(&self) {
        println!("admin is editing");
    }
}
trait CanCreate {
    fn create(&self) {
        println!("admin is creating");
    }
}
trait CanDelete {
    fn delete(&self) {
        println!("admin is deleting");
    }
}

impl CanDelete for AdminUser {}
impl CanCreate for AdminUser {}
impl CanEdit for AdminUser {}

impl CanEdit for User {
    fn edit(&self) {
        println!("A standard user {} is editing", 
                 self.username);
    }
}

fn create<T: CanCreate>(user: &T) -> () {
    user.create();
}
fn edit<T: CanEdit>(user: &T) -> () {
    user.edit();
}
fn delete<T: CanDelete>(user: &T) -> () {
    user.delete();
}

fn main() {

    let admin = AdminUser{
        username: "admin".to_string(), 
        password: "password".to_string()
    };
    let user = User{
        username: "user".to_string(), 
        password: "password".to_string()
    };
    
    create(&admin);
    edit(&admin);
    edit(&user);
    delete(&admin);
}

