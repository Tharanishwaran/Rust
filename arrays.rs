fn main() {

    let mut array: [i32; 10] = [0; 10];

    
array[0] = 1;
array[1] = 2;
array[2] = 3;
array[3] = 4;
array[4] = 5;
array[5] = 6;
array[6] = 7;
array[7] = 8;
array[8] = 9;
array[9] = 10;



// This loop prints: 0 1 2 3 4
for x in &array {

    println!("{} ", x);

}

}

