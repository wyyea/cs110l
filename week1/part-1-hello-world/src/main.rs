fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut arr: [i32; 4] = [1, 2, 3, 4];
    arr[0] = -2;
    for i in vec.iter(){
        println!("{}", i);
    }
    for i in arr{
        println!("{}", i);
    }
}
