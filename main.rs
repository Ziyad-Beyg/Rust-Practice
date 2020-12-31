                                                            // ASSIGNMENT 2

                                                            // Q1
// fn main(){
//     let x = String::from("HELLO");
//     let y = x;
//     println!("{}",x);
// }

                                                            // Q2
// fn main(){
//     let mut s = String::from("PAKISTAN");
//     println!("{}",s);
//     mutable_reference_of_s(&mut s);
//     println!("{}",s);
// }
// fn mutable_reference_of_s(s:&mut String){
//      s.push_str(" ZINDABAD");
// }

                                                            // Q3
// use std::io;
// fn main(){
//     let mut num1 = String::new();
//     let mut num2 = String::new();
//     let mut num3 = String::new();
//     io::stdin().read_line(&mut num1).expect("FAILED TO READ USER INPUT");
//     io::stdin().read_line(&mut num2).expect("FAILED TO READ USER INPUT");
//     io::stdin().read_line(&mut num3).expect("FAILED TO READ USER INPUT");
//     let number1:u32 = num1.trim().parse().expect("ENTER A DIGIT");
//     let number2:u32 = num2.trim().parse().expect("ENTER A DIGIT");
//     let number3:u32 = num3.trim().parse().expect("ENTER A DIGIT");
//     let average = (number1 + number2 + number3)/3;
//     println!("The average of '{}', '{}', '{}' is {}",number1, number2, number3, average);
// }

                                                            // Q4
// use std::io;
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("FAILED TO READ USER INPUT");
//     let mod_input = input.trim();
//     println!("Length of '{}' is {}", mod_input, length_calculator(&input));
// }
// fn length_calculator(x: &String)->usize{
//     x.len()
// }

                                                            // Q5
// use std::io;
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("FAILED TO READ USER INPUT");
//     let input_number:u32 = input.trim().parse().expect("ENTER A VALID DIGIT");
//     let mut x = 0;
//     while x <= input_number{
//         let mut y = 1;
//         while y <= x {
//             print!("*");
//             y = y + 1;
//         }
//         println!("");
//         x = x + 1;
//     }
// }

                                                            // Q6
// #[derive(Debug)]
// struct Student{
//     name: String,
//     email: String,
//     phone_number: u64,
//     gender: String,
// }
// fn main(){
//     let student1 = Student{
//         name:String::from("ALI ASLAM"),
//         email:String::from("aliaslam@gmail.com"),
//         phone_number:12345678910,
//         gender:String::from("Male")
//     };
//     let student2 = Student{
//         name:String::from("AYESHA ASLAM"),
//         email:String::from("atesahaslam@gmail.com"),
//         phone_number:10987654321,
//         gender:String::from("Female")
//     };
//     println!("{:#?}", student1.email);
//     println!("{:#?}", student2);
// }

                                                            // Q7
// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     height:u32
// }
// fn main(){
//     let mut rect1 = Rectangle{
//         width:50,
//         height:100,
//     };
//     println!("{:#?}",rect1);
//     rect1.width = 150;
//     println!("Update width of rectangle is {}",rect1.width);
// }

                                                            // Q8
// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     height:u32
// }
// fn main(){
//     let rect_1 = Rectangle{
//         width:50,
//         height:100
//     };
//     println!("{}", another_function(rect_1));
// }
// fn another_function(x:Rectangle)->u32{
//     x.width + x.height
// }

                                                            // Q9
// #[derive(Debug)]
// struct Triangle{
//     length1 : u32,
//     length2 : u32,
//     length3 : u32,
// }
// impl Triangle{
//     fn sum(&self)->u32{
//         self.length1 + self.length2 + self.length3 
//     }
//     fn greatest(&self)->u32{
//        let result = if self.length1 > self.length2 && self.length1 > self.length3{
//             self.length1
//         }
//         else if self.length2 > self.length1 && self.length2 > self.length3{
//             self.length2
//         }
//         else{
//             self.length3
//         };
//         result
//     }
// }
// fn main(){
//     let t1 = Triangle{
//         length1 : 25,
//         length2 : 80,
//         length3 : 60
//     };
//     println!("The sum of all three sides of a Triangle is: {}", t1.sum());
//     println!("The greatest side from all three sides of a Triangle is: {}",t1.greatest());
// }

                                                            // Q10
// #[derive(Debug)]
// struct PERSON{
//     name : String,
//     age : u32,
//     country : String,
// }
// use std::io;
// fn main(){
// //  USER INPUT
//     let mut name = String::new();
//     println!("Please Enter Your Name: ");
//     io::stdin().read_line(&mut name).expect("Invalid Input");
//     let mut age = String::new();
//     println!("Please Enter Your Age: ");
//     io::stdin().read_line(&mut age).expect("Invalid Input");
//     let mut country = String::new();
//     println!("Please Enter Your Country Name: ");
//     io::stdin().read_line(&mut country).expect("Invalid Input");
// //  STRUCT INSTANCE
//     let person1 = PERSON{
//         name: name.trim().to_string(),
//         age : age.trim().parse().expect("Can not convert string into integer!!!"),
//         country : country.trim().to_string()
//     };
// //  ARRAY CREATION 
//     let arr1 = [&person1.name.to_uppercase(), &person1.age.to_string(), &person1.country.to_uppercase()];
//     println!("User Input Is: {:#?}",arr1);
// }

                                                            // PRACTICE
// #[derive(Debug)]
// struct car_info{
//     company:String,
//     name:String,
//     car_type:String,
//     fuel_type:String,
//     price:u64,
// }

// use std::io;
// fn main(){

//     let car1 = car_info{
//         company:String::from("Honda") ,
//         name:String::from("Civic") ,
//         car_type:String::from("Manual") ,
//         fuel_type:String::from("Petrol") ,
//         price:2000000 ,
//     };
//     let car2 = car_info{
//         company:String::from("Honda") ,
//         name:String::from("Civic") ,
//         car_type:String::from("Automatic") ,
//         fuel_type:String::from("CNG") ,
//         price:2200000 ,
//     };
//     let car3 = car_info{
//         company:String::from("Honda") ,
//         name:String::from("City") ,
//         car_type:String::from("Manual") ,
//         fuel_type:String::from("CNG") ,
//         price:1900000 ,
//     };
//     let car4 = car_info{
//         company:String::from("Honda") ,
//         name:String::from("City") ,
//         car_type:String::from("Automatic") ,
//         fuel_type:String::from("Petrol") ,
//         price:2300000 ,
//     };
//     let car5 = car_info{
//         company:String::from("Toyota") ,
//         name:String::from("Corolla") ,
//         car_type:String::from("Manual") ,
//         fuel_type:String::from("Petrol") ,
//         price:2000000 ,
//     };
//     let car6 = car_info{
//         company:String::from("Toyota") ,
//         name:String::from("Corolla") ,
//         car_type:String::from("Automatic") ,
//         fuel_type:String::from("CNG") ,
//         price:2200000 ,
//     };
//     let car7 = car_info{
//         company:String::from("Toyota") ,
//         name:String::from("Camry") ,
//         car_type:String::from("Manual") ,
//         fuel_type:String::from("CNG") ,
//         price:1900000 ,
//     };
//     let car8 = car_info{
//         company:String::from("Toyota") ,
//         name:String::from("Camry") ,
//         car_type:String::from("Automatic") ,
//         fuel_type:String::from("Petrol") ,
//         price:2300000 ,
//     };

//     println!("ASSALAM O ALAIKUM,\n Wellcome to my shop, it is famous for it's selling and purchasing systems.\n\nWhat is your name?\n");
//     let mut user_name = String::new();
//     io::stdin().read_line(&mut user_name).expect("Enter correct name");
//     let mut user_1 = String::new();
//     println!(" {}, CHOOSE YOUR OPERATION \nA. Sell\nB. Purchase \n", user_name.to_uppercase());
//     io::stdin().read_line(&mut user_1).expect("WRONG ENTRY");


//     let mut user_struct = car_info{
//         company : String::new(),
//         name: String::new(),
//         car_type : String::new(),
//         fuel_type : String::new(),
//         price: 0,
//     };
    

//         if user_1.trim() == "a" || user_1 == "A" || user_1 == "sell" || user_1 == "Sell" || user_1 == "SELL"{
//             println!("\n\n{}, you are here to sell your car. We are so lucky to have a customer like you!!!", user_name.to_uppercase());
        
//             println!(" which car do you want to sell?\n ");
//             io::stdin().read_line(&mut user_struct.name).expect("WRONG ENTRY");
        
//             println!(" what is the transmition type of car?\n ");
//             io::stdin().read_line(&mut user_struct.car_type).expect("WRONG ENTRY");
        
//             println!(" what is the fuel type of your car? \n");
//             io::stdin().read_line(&mut user_struct.fuel_type).expect("WRONG ENTRY");
        
//             println!(" How much price are you asking for your car? \n");
//             let mut car_price =String::new();
//             io::stdin().read_line(&mut car_price).expect("WRONG ENTRY");
//             let new_var:u64 = car_price.trim().parse().expect("WRONG ENTRY");
//             user_struct.price = new_var;
//         }
//         else{
//             println!("!!!!!!!!!!!!!!!!!!!!");
            
//         }
// }