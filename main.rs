// fn main(){
//     println!("Hi there, my name is 'MIRZA ZIYAD AHMED BAIG'");
// }

// fn main(){
//     let mut age = 5_u8;
//     age = age + 15;
//     println!("MY AGE IS {}",age);
// }

// fn main(){
//     const PI:f32 = 3.142;
//     println!("IN MATHEMATICS THE VALUE OF PI IS {}", PI);
// }

// fn main(){
//     let blank_spaces = "     ";
//     let length = blank_spaces.len();
//     println!("length of string {}",length);
// }

// fn main(){
//     let integer:u8 = 5;
//     let float:f32 = 3.142;
//     let character:char = 'z';
//     let boolean:bool = true;
//     println!("I HAVE {} RUPEES",integer);
//     println!("PI VALUE IS {}",float);
//     println!("MY NAME STARTS WITH {}",character);
//     println!("I ALWAYS SPEAKS {}",boolean);
// }

// fn main(){
//     let tuple = (5, -6, 7.12, 'z');
//     println!("{}",tuple.0);
//     println!("{}",tuple.1);
//     println!("{}",tuple.2);
//     println!("{}",tuple.3);
//     let tuple:(char, f64, i32, u8) = ('z', 7.12, -6, 5);
//     println!("{}",tuple.0);
//     println!("{}",tuple.1);
//     println!("{}",tuple.2);
//     println!("{}",tuple.3);
// }

// fn main(){
//     let array = [1, 2, 3, 4, 5, 6];
//     println!("{}",array[0]);
//     println!("{}",array[1]);
//     println!("{}",array[2]);
//     println!("{}",array[3]);
//     println!("{}",array[4]);
//     println!("{}",array[5]);
// }

//                                      ASSIGNMENT 1

// Q1
// fn main(){
//     let pk ="PAKSITAN ZINDABAD";
//     println!("{} \nLENGTH OF {} IS {}",pk ,pk,pk.len());
// }

// Q2
// fn main(){
//     let unsigned_int:u64 = 85;
//     let signed_int:i16 = -550;
//     println!(" {}\n{}",unsigned_int, signed_int); 
// }

// Q3
// fn main(){
//     let f_variable:f32 = 56.6;
//     println!("{}",f_variable); 
// }

// Q4
// fn main(){
//     let a = 76;
//     let b = 23;

//     let addition = a + b;
//     let subtraction = a - b;
//     let division = a / b;
//     let multiplication = a * b;
//     let modulus = a % b;

// //  OUTPUT

//     println!("a + b = {}\na - b = {}\na / b = {}\na * b = {}\na % b = {}",addition,subtraction, division, multiplication,modulus);
// }

// Q5
// fn main(){
//     let arr = [100, 150, 200, 250, 300];
//     println!("{:?}\n{}\n{}", arr, arr[1], arr[3]);
// }

// Q6
// fn main(){
//     let tup = ("IOT", "AI", "CLOUD", 500.65, 8645, 65.4);
//     println!("{:?}\n{}\n{}\n{}", tup, tup.2, tup.4, tup.5);
// }

// Q7
// fn main(){
//     sum(10,20,30);
// }

// fn sum(x:u8, y:u8, z:u8){
//     println!("{}", x + y + z);
// }

// Q8
// fn main(){
//     let multiplication = multiply(5.6, 2.4, 10.2);
//     println!("{}", multiplication);
// }

// fn multiply(x:f32, y:f32, z:f32)->f32{
//     let multiply = x * y * z;
//     multiply
// }

// Q9
// fn main(){
//     let marks =62;
//     if marks >= 80{
//         println!("GRADE A+");
//     }
//     else if marks >= 70 && marks < 80{
//         println!("GRADE A");
//     }
//     else if marks >= 60 && marks < 70{
//         println!("GRADE B");
//     }
//     else if marks >= 50 && marks < 60{
//         println!("GRADE C");
//     }
//     else if marks >= 40 && marks <50{
//         println!("GRADE D");
//     }
//     else if marks < 40{
//         println!("GRADE F");
//     }
// }

// //                      ALTERNATE METHOD

// fn main(){
//     let marks =62;
//     let result = if marks >= 80 && marks <100{
//         "GRADE A+"
//     }
//     else if marks >= 70 && marks < 80{
//         "GRADE A"
//     }
//     else if marks >= 60 && marks < 70{
//         "GRADE B"
//     }
//     else if marks >= 50 && marks < 60{
//         "GRADE C"
//     }
//     else if marks >= 40 && marks <50{
//         "GRADE D"
//     }
//     else {
//         "GRADE F"
//     };

//     println!("{}",result);
// }

// Q10
// fn main(){
//     let year = 2013;
//     if year % 4 == 0{
//         println!("{} is a leap year!!", year);
//     }
//     else{
//         println!("{} is not a leap year!!", year);
//     }
// }

// practice
// fn main(){
//     let mut counter = 0;
//     let result = loop{
//         counter += 1;
//         println!("{}", counter);
//         if counter == 15{
//             break counter * 3
//         }
//     };
//     println!("RETURN VALUE IS {}", result);
// }

// Q11
// fn main(){
//     for i in 0..100{
//         if i % 2 == 0{
//             println!("{}",i);
//         }
//     }
// }

// Q12
// fn main(){
//     let mut counter = 0;
//     while counter < 100{
//         if counter % 2 != 0{
//             println!("{}",counter);
//         }
//         counter += 1;
//     }
// }

// Q13
// fn main(){
//     let number = 4;
//     let mut counter = 0;
//     while counter < 10{
//         counter += 1;
//         println!("{} * {} = {}",number, counter, number * counter);
//     }
// }

//              practice
// fn main(){
//     for i in 0..=10{
//         if i % 2 == 0{
//             println!("{} is an even number!!!", i);
//         }
//         else{
//             println!("{} is an odd number", i);
//         }
//     }
// }

//              practice
// fn main(){
//     println!("NOT true is {}","!true");
// }

// Q14
// fn main(){
//     let x = 95;
//     let n = 150;
//     let p = 100 * x / n ;
//     println!("Percentage of Student is {}%", p);
// }

// Q15
// fn main(){
//     let days =1329;
//     let one_year = 365;
//     let one_week = 7;
//     let yr = days / one_year;
//     let wk = days / one_week;
//     println!("Total Number of Days: {}\nYears: {} \nWeeks: {}", days, yr, wk);
// }