use rand::Rng;
use rand::rngs::ThreadRng;
fn main() {
    println!("Hello, world!");
    let arr:[i32;4] = [7,8,30,21];
    // let mut arr: [i32; 0] = [];

    // let arr:[i32;] = [4];

    bubble_sort(arr);
    
    //create random num
    let mut rng : ThreadRng = rand::thread_rng();

    let a : u8 = rng.gen();
    let b : i32 = rng.gen::<i32>();
    println!("random val is {a} and {b}.");
    
    let f = bubble_sort(arr);   
    match f {
        Ok(f)=> {
            println!("file found {:?}",f);
         },
         Err(e)=> {
            println!("file not found \n{:?}",e);   // 处理错误
         }
    }


    let mut guess = String::new();

}

fn bubble_sort(mut arr:[i32;4])->Result<bool,String>{
// fn bubble_sort(mut arr:[i32;4])->Result<bool,String>{

    let n = arr.len();
    if n > 1 {
        for index in 0..n-1{
            // println!("index is: {} & value is : {}",index,arr[index]);
            if arr[index] > arr[index+1] {
                let tmp = arr[index];
                arr[index] = arr[index+1];
                arr[index+1] = tmp;
            }
        }
        println!("result: {:?}",arr);
        return Ok(true);
    } else {
        return Err("please make sure the lengh of array more than 1".to_string());
    }
}

#[test]
fn test_main() {
    assert_eq!(true,false);
}

