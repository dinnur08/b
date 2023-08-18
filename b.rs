use std::io;

struct Employee {
    employee_id: u32,
    employee_name: String,
    email: String,
    age: u32,
    phone_number: String,
}

impl Employee {
    fn new(id: u32, name: String, email: String, age: u32, phone: String) -> Employee {
        Employee {
            employee_id: id,
            employee_name: name,
            email,
            age,
            phone_number: phone,
        }
    }
}

fn get_employee_by_id(employees: &[Employee], target_id: u32) -> Option<&Employee> {
    employees.iter().find(|&e| e.employee_id == target_id)
}

fn get_employees_by_age(employees: &[Employee], target_age: u32) -> Vec<&Employee> {
    employees.iter().filter(|&e| e.age == target_age).collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    // Collect employee data
    loop {
        println!("Enter employee details or type 'exit' to finish:");
        
        println!("Employee ID:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "exit" {
            break;
        }
        let id: u32 = input.trim().parse().expect("Invalid input for ID");

        println!("Employee Name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Email:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read line");

        println!("Age:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read line");
        let age: u32 = age_input.trim().parse().expect("Invalid input for age");

        println!("Phone Number:");
        let mut phone = String::new();
        io::stdin().read_line(&mut phone).expect("Failed to read line");

        employees.push(Employee::new(id, name.trim().to_string(), email.trim().to_string(), age, phone.trim().to_string()));
    }

    // Prompt for employee_id and display details
    println!("Enter employee ID to retrieve details:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target_id: u32 = input.trim().parse().expect("Invalid input for employee ID");

    if let Some(employee) = get_employee_by_id(&employees, target_id) {
        println!("Employee Details:");
        println!("ID: {}", employee.employee_id);
        println!("Name: {}", employee.employee_name);
        println!("Email: {}", employee.email);
        println!("Age: {}", employee.age);
        println!("Phone: {}", employee.phone_number);
    } else {
        println!("Employee with ID {} not found", target_id);
    }

    // Prompt for age and display employees with the same age
    println!("Enter age to retrieve employees with the same age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let target_age: u32 = age_input.trim().parse().expect("Invalid input for age");

    let same_age_employees = get_employees_by_age(&employees, target_age);
    if same_age_employees.is_empty() {
        println!("No employees found with age {}", target_age);
    } else {
        println!("Employees with age {}:", target_age);
        for employee in same_age_employees {
            println!("ID: {}", employee.employee_id);
            println!("Name: {}", employee.employee_name);
            println!("Email: {}", employee.email);
            println!("Phone: {}", employee.phone_number);
            println!();
        }
    }
}
