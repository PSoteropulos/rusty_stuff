fn main() {
    //* &str and String */
    let _fixed_str = "Fixed length String";
    let mut _flexible_str = String::from("This string will grow");

    //* Arrays
    let mut _array_1 = [4, 5, 6, 8, 9];

    //* Vectors
    // let _vec0: Vec<i32> = vec![4, 5, 6, 8, 9];
    type MyVector = Vec<i32>;
    let _vec1: MyVector = vec![4, 5, 6, 8, 9];

    //* Tuples
    let my_info = (String::from("Salary"), 40000, "Age", 40);
    let _salary_value = my_info.1;
    let (_salary, _salary_value, _age, _age_value) = my_info;
}
