#[derive(Debug,Clone)]
struct Employee(i32, String, usize);   // Id, Name, S

#[derive(Debug,Clone)]
struct Employees{
    employee_list: [Employee;5],
    count: usize
    
}


impl Employees{

    pub fn add_employee(&mut self, emp:Employee){
        self.employee_list[self.count] = emp;
        self.count += 1;
    }

    ///////
    pub fn remove_employee(&mut self, emp_id:i32){
        
        for i in 0..self.count{
            
            if self.employee_list[i].0 == emp_id {
                self.employee_list[i] = Employee(0, "N/A".to_string(), 0);
            }

        }

    }

    pub fn get_employee_details(&self, emp_id:i32)-> Employee{
        for i in 0..self.count{
            if self.employee_list[i].0 == emp_id {
                return self.employee_list[i].clone()
            }

        }
        return Employee(0, "N/A".to_string(), 0)
    }

    pub fn get_employee_name(&self, emp_id:i32) -> String{
        return self.get_employee_details(emp_id).1
    }

    pub fn get_net_salary(&self, emp_id: i32) -> usize{

        let salary  = self.get_employee_details(emp_id).2; 

        let da:usize = salary * 12/100;
        const HRA:usize = 150;
        const TA:usize = 120;

        return salary + da + HRA + TA
    }

    pub fn print_record(&self){

        for i in 0..self.count {
            println!("{:?}", self.employee_list[i as usize]);
        }
    }

}

fn main() {
    
     let emp0 = Employee(0, String::from("N/A"), 0);

    let mut office = Employees{
        employee_list:[emp0.clone(), emp0.clone(),  emp0.clone(),emp0.clone() , emp0.clone()],
        count:0
    };

    // Adding 

    office.add_employee(Employee(76, String::from("Emp1"), 15000));
    office.add_employee(Employee(76, String::from("Emp2"), 20000));
    office.add_employee(Employee(3, String::from("Emp3"), 18000));

    // Removing
    office.remove_employee(2);

    // Get Employee by Id
    let employee_x =  office.get_employee_details(1);
    println!("\nDetails of Employee Id:1 {:?} \n", employee_x);

    // Get Employee Name by Id
    println!("Employee Name of Id:3 is {} \n", office.get_employee_name(3));

    // Get net salary
    println!("Net Salary of Employee Emp3 {} \n", office.get_net_salary(3) );

    // Print_record();
    office.print_record();
}
