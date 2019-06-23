use std::collections::HashMap;

struct Userdetails {
    user_name: String,
    user_dept: String,
}

fn main() {
    let mut department = Vec::new();
    let mut emp_dept_map =  HashMap::new();

    department.push("Engineering");
    department.push("Sales");
    department.push("Admin");
    department.push("HR");
    department.push("Finance");

    let user_str = String::from("Add Sally Gupta to Engineering");
    let user_info = get_user_and_department(&user_str, &department);
    emp_dept_map.insert(user_info.user_name, user_info.user_dept);

    let user_str = String::from("Add Amir Mahajan to Sales");
    let user_info = get_user_and_department(&user_str, &department);
    emp_dept_map.insert(user_info.user_name, user_info.user_dept);
    
    println!("{:?}", emp_dept_map);
}


fn get_user_and_department(user_str: &str, department: &Vec<&str>)->Userdetails {
    let mut user_department = String::new();
    let mut name = String::new();
    let mut flag = false;

    for word in user_str.split_whitespace() {
        if word == "Add" || word == "to" {
            continue;
        }
        for dept in department {
            if word == *dept {
                user_department.push_str(word);
                flag = true;
                break;
            }
        }
        if flag == false {
            name.push_str(word);
        }
    }

    Userdetails{user_name: name, user_dept: user_department}
}
