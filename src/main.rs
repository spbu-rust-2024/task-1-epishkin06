use std::io;

fn string_to_array(input_string: String) -> Vec<i32>{
    let mut array: Vec<i32> = vec![];
    let mut temp_str: String = String::new();
    for ch in input_string.chars(){
        if ch == ' ' && temp_str != ""{
            let num = temp_str.trim().parse::<i32>().unwrap();
            array.push(num);
            temp_str.clear();
            continue;
        }
        temp_str.push(ch);
        }
    let num = temp_str.trim().parse::<i32>().unwrap();
    array.push(num);
    temp_str.clear();
    return array;
}

fn insertion_sort(arr: &mut [i32]){
    for i in 1..arr.len(){
        let temp = arr[i];
        let mut location = i as i32 - 1;
        while location >= 0 && arr[location as usize] > temp{
            arr[location as usize + 1] = arr[location as usize];
            location -= 1;
        }
        arr[(location + 1) as usize] = temp;
    }}

fn main() {
    println!("Введите строку чисел через пробел");
    let mut input_string: String = String::new();
    match io::stdin().read_line(&mut input_string){
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    let mut array_to_sort: Vec<i32> = string_to_array(input_string);
    insertion_sort(&mut array_to_sort);

    let mut final_result: String = String::new();
    for i in 0..array_to_sort.len(){
        let c = array_to_sort[i].to_string();
        final_result += &c;
        final_result.push(' ');
    }
    final_result.pop();
    println!("{}", final_result);
    }
