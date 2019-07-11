 pub fn single_module(){
    println!("single module");

}
 pub mod c_module{
      pub fn java_module(){
         println!("java module");
    }

     pub mod cplus_module{
         pub fn cplus_module(){
             println!("cplus module");
         }
     }
 }
