// use std::fmt;
// use std::fmt::Result;
#[derive(Clone, Copy, Debug)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

#[derive(Clone, Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        match name {
            "Anita" => Ok(Employee {
                name: "Anita".to_string(),
            }),
            "Brody" => Ok(Employee {
                name: "Brody".to_string(),
            }),
            "Catherine" => Ok(Employee {
                name: "Catherine".to_string(),
            }),
            _ => Err(String::from("employee not found")),
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
        match employee.name.as_str() {
            "Anita" => Ok(KeyCard { access_level: 1000 }),
            "Brody" => Ok(KeyCard { access_level: 500 }),
            other => Err(format!("{other} doesnot have a keycard")),
        }
    }
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    let db = Database::connect()?;
    let employee = db.find_employee(employee_name)?;
    let keycard = db.get_keycard(&employee)?;

    if keycard.access_level >= location.required_access_level() {
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse);
    match anita_authorized {
        Ok(AuthorizationStatus::Allow) => println!("Access Granted for Anita"),
        Ok(AuthorizationStatus::Deny) => println!("Access Denied! insufficient previlages"),
        _ => (),
    }
    let brody_authorization = authorize("Brody", ProtectedLocation::Office);
    match brody_authorization {
        Ok(AuthorizationStatus::Allow) => println!("Access Granted for Brody"),
        Ok(AuthorizationStatus::Deny) => println!("Access Denied for Brody"),
        _ => (),
    }
}
