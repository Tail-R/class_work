use std::time::Instant;

fn h_check(
    mat: &mut [[[u8; NN as usize + 1]; NN as usize]; NN as usize],
    x: u8,
    y: u8,
    val: u8
    ) {

    for j in 0..NN {
        if j != x {
            mat[y as usize][j as usize][val as usize] = 1;
        }
    }
}

fn v_check(
    mat: &mut [[[u8; NN as usize + 1]; NN as usize]; NN as usize],
    x: u8,
    y: u8,
    val: u8
    ) {

    for i in 0..NN {
        if i != y {
            mat[i as usize][x as usize][val as usize] = 1;
        }
    }
}

fn block_check(
    mat: &mut [[[u8; NN as usize + 1]; NN as usize]; NN as usize],
    x: u8,
    y: u8,
    base_x: u8,
    base_y: u8,
    val: u8
    ) {

    for i in 0..N {
        for j in 0..N {
            if (base_y + i != y) || (base_x + j != x) {
                mat[(base_y + i) as usize][(base_x + j) as usize][val as usize] = 1;
            }
        }
    }
}

fn update_cell(
    mat: &mut [[[u8; NN as usize + 1]; NN as usize]; NN as usize],
    x: u8,
    y: u8
    ) -> bool {

    let mut flag :bool = false;


    if mat[y as usize][x as usize][0] == 0 {
        let sum :u8 = mat[y as usize][x as usize].iter().sum();

        if sum == (NN - 1) {
            flag = true;

            for k in 1..=NN {
                if mat[y as usize][x as usize][k as usize] == 0 {
                    mat[y as usize][x as usize][0] = k;
                    break;
                }
            }
        }
    }

    return flag;
}

fn update_board(
    mat: &mut [[[u8; NN as usize + 1]; NN as usize]; NN as usize]
    ) {
    
    let mut nof: u16 = 0;
    let mut tbl: [[bool; NN as usize]; NN as usize] =
        [[true; NN as usize]; NN as usize];
    
    loop {
        for i in 0..NN {
            for j in 0..NN {
                if update_cell(mat, j, i) { nof = 0  } else { nof += 1 }

                if nof > (NN as u16 * NN as u16 * 2) { return; }
                
                if tbl[i as usize][j as usize] &&
                    mat[i as usize][j as usize][0] != 0 {
                    
                    let val: u8 = mat[i as usize][j as usize][0];

                    h_check(mat, j, i, val);
                    v_check(mat, j, i, val);
                    block_check(
                        mat, j, i,
                        (j / N * N) as u8,
                        (i / N * N) as u8,
                        val);

                    tbl[i as usize][j as usize] = false;    
                }
            }
        }
    }
}

// fn h_is_valid(
//     mat: &[[[u8; NN as usize + 1]; NN as usize]; NN as usize],
//     y: u8,
//     val: u8
//     ) -> bool {
// 
//     for j in 0..NN {
//         if mat[y as usize][j as usize][0] == val {
//             return false;
//         }
//     }
// 
//     return true;
// }
// 
// fn v_is_valid(
//     mat: &[[[u8; NN as usize + 1]; NN as usize]; NN as usize],
//     x: u8,
//     val: u8
//     ) -> bool {
// 
//     for i in 0..NN {
//         if mat[i as usize][x as usize][0] == val {
//              return false;
//         }
//     }
// 
//     return true;
// }
// 
// fn block_is_valid(
//     mat: &[[[u8; NN as usize + 1]; NN as usize]; NN as usize],
//     base_x: u8,
//     base_y: u8,
//     val: u8
//     ) -> bool {
// 
//     for i in 0..N {
//         for j in 0..N {
//             if mat[(base_y + i) as usize][(base_x + j) as usize][0] == val {
//                 return false;
//             }
//         }
//     }
// 
//     return true;
// }

fn next_n(
    mat: &mut [[[u8; NN as usize + 1]; NN as usize]; NN as usize]
    ) -> i8 {

    let mut max :u8 = 0;
    let mut pos :u8 = 0;
    let mut n   :i8 = -1;
    
    for i in 0..NN {
        for j in 0..NN {
            if mat[i as usize][j as usize][0] == 0 {
                let sum :u8 = mat[i as usize][j as usize].iter().sum();
                
                if sum > max {
                    max = sum;
                    n = pos as i8;
                }
            }
            
            pos += 1;
        }
    }

    return n;
}

// fn next_n(
//     mat: &mut [[[u8; NN as usize + 1]; NN as usize]; NN as usize]
//     ) -> i16 {
//   
//     let mut n = 0;
//    
//     for i in 0..NN {
//         for j in 0..NN {
//             if mat[i as usize][j as usize][0] == 0 { return n; } else { n += 1; }
//         }
//     }
// 
//     return -1;
// }

fn backtrack(
    mat: &mut [[[u8; NN as usize + 1]; NN as usize]; NN as usize]
    ) {

    let n = next_n(mat);
    if n == -1 { print_mat_2d(&mat); return; }
    
    let x = n as u8 % NN;
    let y = n as u8 / NN;

    if mat[y as usize][x as usize][0] == 0 { // 0 means unknown
        for val in 1..=NN {
            if mat[y as usize][x as usize][val as usize] == 0 { // the val is valid
                let mut mat_clone:
                    [[[u8; NN as usize + 1]; NN as usize]; NN as usize] = *mat;
                
                mat_clone[y as usize][x as usize][0] = val; 
                update_board(&mut mat_clone);
                backtrack(&mut mat_clone);
            }
        }
    }
}

fn print_mat_2d(
    mat: &[[[u8; NN as usize + 1]; NN as usize]; NN as usize]
    ) {
    
    for i in 0..NN {
        for j in 0..NN {
            print!("{:<3}", mat[i as usize][j as usize][0]);
        }

        print!("\n");
    }
    
    print!("\n");
}

// fn print_mat_3d(
//     mat: &[[[u8; NN as usize + 1]; NN as usize]; NN as usize]
//     ) {
// 
//     for i in 0..NN {
//         for j in 0..NN {
//             print!("{}: ", mat[i as usize][j as usize][0]);
//             
//             for k in 1..=NN {
//                 print!("{} ", mat[i as usize][j as usize][k as usize]);
//             }
// 
//             print!(" ");
//         }
// 
//         print!("\n");
//     }
// 
//     print!("\n");
// }

const N: u8 = 3;
const NN: u8 = 9;

fn main() {
    let mut mat: [[[u8; NN as usize + 1]; NN as usize]; NN as usize] =
        [[[0; NN as usize + 1]; NN as usize]; NN as usize];
    
    {
        let mmat: [[u8; NN as usize]; NN as usize] =
            // [
            //       [8, 0, 0, 0, 0, 0, 0, 0, 0],
            //       [0, 0, 3, 6, 0, 0, 0, 0, 0],
            //       [0, 7, 0, 0, 9, 0, 2, 0, 0],

            //       [0, 5, 0, 0, 0, 7, 0, 0, 0],
            //       [0, 0, 0, 0, 4, 5, 7, 0, 0],
            //       [0, 0, 0, 1, 0, 0, 0, 3, 0],

            //       [0, 0, 1, 0, 0, 0, 0, 6, 8],
            //       [0, 0, 8, 5, 0, 0, 0, 1, 0],
            //       [0, 9, 0, 0, 0, 0, 4, 0, 0]
            // ];
            
            [
                [1, 2, 8, 0, 6, 0, 0, 7, 9],
                [0, 0, 0, 0, 2, 7, 0, 0, 4],
                [4, 0, 7, 0, 0, 0, 1, 0, 0],

                [8, 0, 0, 2, 0, 0, 9, 0, 7],
                [0, 0, 4, 0, 0, 1, 5, 0, 8],
                [0, 9, 5, 0, 0, 8, 2, 3, 1],

                [0, 8, 0, 0, 0, 0, 0, 0, 6],
                [0, 0, 0, 8, 0, 0, 0, 5, 0],
                [5, 0, 0, 0, 1, 0, 0, 9, 0]
            ];

        for i in 0..NN {
            for j in 0..NN {
                mat[i as usize][j as usize][0] =
                    mmat[i as usize][j as usize];
            }
        }
    }

    print_mat_2d(& mat);
    let start = Instant::now();

    update_board(&mut mat);
    backtrack(&mut mat);
     
    let end = start.elapsed();
    println!("time: {}.{}", end.as_secs(), end.subsec_nanos());

}

