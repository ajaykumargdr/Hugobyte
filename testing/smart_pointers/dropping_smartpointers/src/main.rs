struct CustomSmartPointer{
    data: String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Dopping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main(){

    let mut _cp1 = CustomSmartPointer{data:String::from("_Cp1")};

    let mut _cp2 = CustomSmartPointer{data:String::from("_Cp2")};

    // cannot use drop explicitly
    // _cp1.drop();

    // std::mem::drop
    drop(_cp1); // it works

    println!("CustomSmartPointers are created");

}
