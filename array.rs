fn main() {
    let a1 = [ 1i32, 2, 3 ];
    let a2 = [ 1.0f64, 2.0f64, 3.1f64 ];
    let a3 = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ];

    let mut index = 0;

    println!("a1 has {} things!", a1.len());

    for item in a1.iter() {
        println!("a1[{}]={}", index, item);
        index = index + 1;
    }

    for item in a2.iter() {
        println!("a2[{}]={}", index, item);
        index = index + 1;
    }

    let sliced_a = &a3[3..6];

    for item in sliced_a.iter() {
        println!("sliced_a[{}]={}", index, item);
        index = index + 1;
    }

}
