use std::collections::HashMap;
use rand::Rng;
use rand::rngs::ThreadRng;

//task: hong
fn main() {
    let mut list = vec![1, 50, 200, 34, 2, 100];
    select_sort(&mut list);
    println!("{:?}  ", list);

    //create random num
    let arr:[i32;4] = [7,8,30,21];

    let f = bubble_sort(arr);   
    match f {
        Ok(f)=> {
            println!("sort result: {:?}",f);
         },
         Err(e)=> {
            println!("illeagal input: \n{:?}",e);   // 处理错误
         }
    }

}


fn bubble_sort(mut arr:[i32;4])->Result<bool,String>{
    // fn bubble_sort<T>(mut arr: T)->Result<bool,String>{
    
    
        let n = arr.len();
        if n > 1 {
            for index in 0..n-1{
                // println!("index is: {} & value is : {}",index,arr[index]);
                if arr[index] > arr[index+1] {
                    // let tmp = arr[index];
                    // arr[index] = arr[index+1];
                    // arr[index+1] = tmp;
                    arr.swap(index,index+1);
                }
            }
            println!("result: {:?}",arr);
            return Ok(true);
        } else {
            return Err("please make sure the lengh of array more than 1".to_string());
        }
    }
    
fn select_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for i in 0..list.len() {
        let mut element = list[i];
        let mut chang_index = 0;
        for j in 0..list.len() {
            if let Some(elem) = list.get_mut(j) {
                //从待排序的数据中寻找最小值
                if j > i && *elem < element {
                    element = *elem;
                    chang_index = j; //记录最小值的序列
                }
            }
        }
        if chang_index > 0 {
            list.swap(i, chang_index); //将其与序列最左边的数字进行交换
        }
    }
    list
}
    
//collection: vector and hashmap,https://www.jc2182.com/rust/rust-collections.html    
#[test]
fn map_func(){
    let words = ["losing", "my", "edge"];
    let lengths = words.iter().map(|word| (word, word.len()));
    
    println!("lengths:{:?}",lengths);
    //output: lengths:Map { iter: Iter(["losing", "my", "edge"]) }
    let map: HashMap<_, _> = lengths.clone().collect();

    println!("map:{:?}",map);
    //output: {"my": 2, "losing": 6, "edge": 4}
    let vec: Vec<_> = lengths.collect();
    // [("losing", 6), ("my", 2), ("edge", 4)]

    let instance_name: HashMap<&str, &str> = HashMap::new();

}

#[test]
fn vector_func(){
    // Vector 是一个可调整大小的数组。它将值存储在连续的内存块中。预定义的结构 Vec 可用于创建向量。Vector 的一些重要特征是 -
    //Vector 的内存分配在堆中。

    let mut instance_name: Vec<i32> = Vec::new();
    instance_name.push(40);
   println!("size of vector is :{}",instance_name.len());
   let v = vec![1,2,3];

}

#[test]
fn swap(){
    let mut v = ["a", "b", "c", "d"];
    v.swap(1, 3);
    //output:
    assert!(v == ["a", "d", "c", "b"]);


    // let tmp = arr[index];
    // arr[index] = arr[index+1];
    // arr[index+1] = tmp;
    
    // arr.swap(index,index+1);
 }
//move,copy,clone,Reference
#[test]
fn move_func(){
    let vec_num: Vec<i32> = Vec::new();
    let vec_num_pass = vec_num;
    // println!("v's length is {}", vec_num);//error: borrow of moved value: `v`

    let int_num: i32 = 42;
    let int_num_pass = int_num;
    println!("v is {}", int_num);//compiles fine, no error!

    //引用是 Rust 中最常见的指针类型。引用是指向其他数据类型（如数组、切片或结构）的不可变或可变引用。以下是 Rust 中引用的语法：
    let array = [1, 2, 3, 4, 5];
    let reference = &array;

    //为了避免访问越界错误，在 Rust 中，建议使用 `get` 方法来访问数组，而不是使用直接索引：
    //使用 `get` 方法来访问数组的第 4 个元素。如果元素存在，则输出其值。如果元素不存在，则输出“Value not found!”消息。
    let array = [1, 2, 3];
    match array.get(4) {
        Some(value) => println!("{}", value),
        None => println!("Value not found!")
    }

    //slice
    let data = [10,20,30,40,50];
    use_slice(&data[1..4]);
    fn use_slice(slice:&[i32]) { 
        // is taking a slice or borrowing a part of an array of i32s
        println!("length of slice is {:?}",slice.len());
        println!("{:?}",slice);
     }
    //this is effectively borrowing elements for a while
    //output:length of slice is 3   [20, 30, 40]

    //默认情况下 切片 是不可变更的,定义切片的时候添加了 &mut 关键字，那么我们就可以通过更改切片的元素来影响原数据。
}

#[test]
fn random_mock() {
    let mut rng : ThreadRng = rand::thread_rng();

    //random tuple
    let rand_tuple = (
        rng.gen_range(0..100),
        rng.gen::<bool>(),
        rng.gen_range(0.0..1.0)
    );
    println!("Random tuple: {:?}", rand_tuple);

    //random array without range
    let my_array: [u64; 8] = rand::random();
    println!("Random array without range:{:?}", my_array);

    //random array with range
    //method 1
    let a = [(); 8].map(|_| rng.gen_range(0.0..1.0));
    println!("The array of random float numbers between 0.0 and 1.0 is: {:?}", a);
    //method 2
    let my_array_with_range:[i32;3]= [
                rng.gen_range(0..100),
                rng.gen_range(0..100),
                rng.gen_range(0..100),
    ];

    println!("Random array within 100:{:?}", my_array_with_range);
    

}

#[test]
fn test_loop() {
   let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);

   for val in arr.iter(){
      println!("value is :{}",val);
   }

   for index in 0..arr.len()-1{
    println!("index is: {} & value is : {}",index,arr[index]);
   }
}
#[test]
fn test_main() {
    assert_eq!(true,true);
}