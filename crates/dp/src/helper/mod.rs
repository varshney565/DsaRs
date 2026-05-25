pub fn display1d(dp: &Vec<i32>) {
    // dp.iter()
    //     .enumerate()
    //     .for_each(|(idx, val)| print!("{}:{}, ", idx, val));

    for val in dp {
        print!("{}, ", val);    
    }
}


pub fn display2d(dp : &Vec<Vec<i32>>) {
    // dp.iter()
    //     .enumerate()
    //     .for_each(|(idx,arr)| {
    //         print!("{} ::: ",idx);
    //         arr.iter()
    //             .enumerate()
    //             .for_each(|(idx2,val)| {
    //                 print!("{}:{}, ",idx2,val);
    //             });
    //         print!("\n");
    //     });

    for x in dp {
        for y in x {
            print!(" {},",y);
        }
        println!();
    }
}