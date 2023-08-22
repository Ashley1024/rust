use rand::Rng;
use rand::rngs::ThreadRng;
fn main() {

    random_mock();
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

    let mut list = vec![1, 50, 200, 34, 2, 100];
    select_sort(&mut list);
    println!("{:?}  ", list);

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
fn test_main() {
    assert_eq!(true,false);
}

