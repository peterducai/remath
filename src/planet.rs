struct Planet {
    emp_id: u32,
    emp_name: String,
    }

impl Planet {
    fn show_details(self: &Self) {
    println!("ID: {}", self.emp_id);
    println!("Planet {}", self.emp_name);
    }
    
    fn mutate_and_show_details(&mut self, new_name: String) {
    self.emp_name = new_name;
    println!("The new name for the ID {}, is {}", self.emp_id, self.emp_name);
    }
    
    fn create_mars(id: u32, name: String) -> Planet {
    Planet { emp_id: id, emp_name: name }
    }
}