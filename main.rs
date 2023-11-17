use std::time::Instant;
// use rand::Rng;

fn _h_check(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    x: u8,
    y: u8,
    val: u8
    ) {

    for j in 0..N {
        for _ in 0..N {
            if j != x {
                mat[y as usize][j as usize][val as usize] = 1;
            }
        }
    }
}

fn _h_check_push_mask(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    mask_stack: &mut Vec<[bool; N as usize]>,
    x: u8,
    y: u8,
    val: u8
    ) {
        
    let mut mask: [bool; N as usize] = [false; N as usize];
     
    for j in 0..N {     
        for _ in 0..N {
            if j != x {
                if mat[y as usize][j as usize][val as usize] == 0 {
                    mat[y as usize][j as usize][val as usize] = 1;
                    mask[j as usize] = mask[j as usize] || true;
                }
            }
        }
    }

    /*** Debugging ***/
    // println!("x = {}, y = {}, {}, {:?}", x, y, val, mask);

    mask_stack.push(mask);
}

fn _h_uncheck(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    mask: [bool; N as usize],
    y: u8,
    val: u8
    ) {
                
    for j in 0..N {
        if mask[j as usize] {
            mat[y as usize][j as usize][val as usize] = 0;
        }
    }
}

fn _v_check(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    x: u8,
    y: u8,
    val: u8
    ) {
    
    for i in 0..N {
        for _ in 0..N {
            if i != y {
                mat[i as usize][x as usize][val as usize] = 1;
            }
        }
    }
}

fn _v_check_push_mask(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    mask_stack: &mut Vec<[bool; N as usize]>,
    x: u8,
    y: u8,
    val: u8
    ) {
        
    let mut mask: [bool; N as usize] = [false; N as usize];
     
    for i in 0..N {     
        for _ in 0..N {
            if i != y {
                if mat[i as usize][x as usize][val as usize] == 0 {
                    mat[i as usize][x as usize][val as usize] = 1;
                    mask[i as usize] = mask[i as usize] || true;
                }
            }
        }
    }
    
    /*** Debugging ***/
    // println!("x = {}, y = {}, {}, {:?}", x, y, val, mask);
    
    mask_stack.push(mask);
}

fn _v_uncheck(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    mask: [bool; N as usize],
    x: u8,
    val: u8
    ) {
                
    for i in 0..N {
        if mask[i as usize] {
            mat[i as usize][x as usize][val as usize] = 0;
        }
    }
}

fn _block_check(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    x: u8,
    y: u8,
    base_x: u8,
    base_y: u8,
    val: u8
    ) {

    for i in 0..NN {
        for j in 0..NN {
            if (base_y + i != y) || (base_x + j != x) {
                mat[(base_y + i) as usize][(base_x + j) as usize][val as usize] = 1;
            }
        }
    }
}

fn _cell_init(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    x: u8,
    y: u8,
    ) -> bool {
    
    let mut flag :bool = false;

    
    if mat[y as usize][x as usize][0] == 0 {
        let sum :u8 = mat[y as usize][x as usize].iter().sum();
        
        if sum == (N - 1) {
            flag = true; 
            
            for k in 1..=N {
                if mat[y as usize][x as usize][k as usize] == 0 {
                    mat[y as usize][x as usize][0] = k;
                }
            }
        }
    }

    flag
}

fn _h_val_is_ok(
    mat: &[[[u8; N as usize + 1]; N as usize]; N as usize],
    y: u8,
    val: u8
    ) -> bool {
    
    for j in 0..N {
        if mat[y as usize][j as usize][0] == val {
            return false;
        }
    }
        
    return true;
}

fn _v_val_is_ok(
    mat: &[[[u8; N as usize + 1]; N as usize]; N as usize],
    x: u8,
    val: u8
    ) -> bool {
    
    for i in 0..N {
        if mat[i as usize][x as usize][0] == val {
             return false;
        }
    }

    return true;
}

fn _block_val_is_ok(
    mat: &[[[u8; N as usize + 1]; N as usize]; N as usize],
    base_x: u8,
    base_y: u8,
    val: u8
    ) -> bool {

    for i in 0..NN {
        for j in 0..NN {
            if mat[(base_y + i) as usize][(base_x + j) as usize][0] == val {
                return false;
            }
        }
    }
    
    return true;
}

fn _kouho_kanri(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    ) {

    let mut nof: u16 = 0;
    
    loop {
        for i in 0..N { 
            for j in 0..N { 
                if _cell_init(mat, j, i) { nof = 0 } else { nof += 1 }    
                
                if nof > (N as u16 * N as u16 * 2) { return; }
                
                if mat[i as usize][j as usize][0] != 0 { 
                    let val: u8 = mat[i as usize][j as usize][0]; 
                     
                    _h_check(mat, j, i, val);
                    _v_check(mat, j, i, val);
                     
                    _block_check(
                        mat, j, i,
                        (j / NN * NN) as u8,
                        (i / NN * NN) as u8,
                        val);
                }
            }
        }
    }
}

fn _normal_backtracking(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    n: u16,
    ) {

    if n == N as u16 * N as u16 { _print_mat_2d(&mat); return; }

    let x = n as u8 % N;
    let y = n as u8 / N; 
    
    if mat[y as usize][x as usize][0] == 0 {
        for val in 1..=N {
            if _h_val_is_ok(mat, y, val) &&
                _v_val_is_ok(mat, x, val) &&
                    _block_val_is_ok(mat,
                        x / NN * NN,
                        y / NN * NN, val) {
                    
                mat[y as usize][x as usize][0] = val;      
                _normal_backtracking(mat, n + 1);
                mat[y as usize][x as usize][0] = 0;
            }
        }
    } else { _normal_backtracking(mat, n + 1); }    
}

fn _hybrid_backtracking(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    n: u16,
    ) {
    
    if n == N as u16 * N as u16 { _print_mat_2d(&mat); return; }

    let x = n as u8 % N;
    let y = n as u8 / N; 
    
    if mat[y as usize][x as usize][0] == 0 { 
        for val in 1..=N {
            if mat[y as usize][x as usize][val as usize] == 0 &&
                _h_val_is_ok(mat, y, val) && 
                _v_val_is_ok(mat, x, val) &&
                _block_val_is_ok( mat,
                    x / NN * NN,
                    y / NN * NN, val) {
                    
                mat[y as usize][x as usize][0] = val;      
                _hybrid_backtracking(mat, n + 1);
                mat[y as usize][x as usize][0] = 0;
            }
        }    
    } else { _hybrid_backtracking(mat, n + 1); }    
}

fn _hybrid_backtracking_v2(
    mat: &mut [[[u8; N as usize + 1]; N as usize]; N as usize],
    mask_stack: &mut Vec<[bool; N as usize]>,
    n: u16,
    ) {
    
    if n == N as u16 * N as u16 { _print_mat_2d(&mat); return; } 

    let x = n as u8 % N;
    let y = n as u8 / N;
 
    if mat[y as usize][x as usize][0] == 0 {  
        let base_x = x / NN * NN;
        let base_y = y / NN * NN;
        
        for val in 1..=N {
            if mat[y as usize][x as usize][val as usize] == 0 {
                if  _h_val_is_ok(mat, y, val) &&
                    _v_val_is_ok(mat, x, val) &&
                        _block_val_is_ok(mat, base_x, base_y, val) {
                 
                    mat[y as usize][x as usize][0] = val;
                    
                    /***** horizontal check *****/
                    _h_check_push_mask(mat, mask_stack, x, y, val);
                    
                    /***** vertical check *****/
                    _v_check_push_mask(mat, mask_stack, x, y, val);
                    
                    /*** call ***/
                    _hybrid_backtracking_v2(mat, &mut mask_stack.to_vec(), n + 1);

                    /***** vertical uncheck *****/ 
                    let v_mask: [bool; N as usize] = mask_stack.pop().unwrap();
                    _v_uncheck(mat, v_mask, x, val);
                    
                    /***** horizontal uncheck *****/ 
                    let h_mask: [bool; N as usize] = mask_stack.pop().unwrap();
                    _h_uncheck(mat, h_mask, y, val);
                    
                    mat[y as usize][x as usize][0] = 0;
                }
            }
        }
    } else { _hybrid_backtracking_v2(mat, mask_stack, n + 1); }    
}

fn _print_mat_2d(
    mat: &[[[u8; N as usize + 1]; N as usize]; N as usize]
    ) {

    for i in 0..N  {
        for j in 0..N {
            print!("{:<3}", mat[i as usize][j as usize][0]);
        }

        print!("\n");
    }

    print!("\n");
}

fn _print_mat_3d(
    mat: &[[[u8; N as usize + 1]; N as usize]; N as usize]
    ) {
    
    for i in 0..N {
        for j in 0..N {
            for k in 1..=N { 
                print!("{} ", mat[i as usize][j as usize][k as usize]);

                if k == 0 {
                    print!(" ");
                }
            }
            
            print!(" ");
        }
       
        print!("\n");
    }
    
    print!("\n");
}

// board size (N * N)
const N: u8 = 9;
const NN: u8 = 3;

fn main() {
    /***** Variables *****/
    let mut mat: [[[u8; N as usize + 1]; N as usize]; N as usize] =
        [[[0; N as usize + 1]; N as usize]; N as usize];

    let mut mask_stack: Vec<[bool; N as usize]> = Vec::new();
    
    /***** Manual initialize *****/
    {
        let mmat: [[u8; N as usize]; N as usize] =
        
        /***** 4x4 *****/
            // [
            //     [4, 0, 0, 0],
            //     [0, 0, 0, 0],
            //     [1, 0, 4, 0],
            //     [0, 0, 0, 2]
            // ];
            
            // [
            //      [4, 0, 0, 0],
            //      [0, 0, 0, 0],
            //      [0, 0, 4, 0],
            //      [0, 0, 0, 0]
            // ];

        /***** 9x9 *****/
            // [
            //     [1, 2, 8, 0, 6, 0, 0, 7, 9],
            //     [0, 0, 0, 0, 2, 7, 0, 0, 4],
            //     [4, 0, 7, 0, 0, 0, 1, 0, 0],
            //     
            //     [8, 0, 0, 2, 0, 0, 9, 0, 7],
            //     [0, 0, 4, 0, 0, 1, 5, 0, 8],
            //     [0, 9, 5, 0, 0, 8, 2, 3, 1],
            //     
            //     [0, 8, 0, 0, 0, 0, 0, 0, 6],
            //     [0, 0, 0, 8, 0, 0, 0, 5, 0],
            //     [5, 0, 0, 0, 1, 0, 0, 9, 0]
            // ];
            
            [
                [8, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 3, 6, 0, 0, 0, 0, 0],
                [0, 7, 0, 0, 9, 0, 2, 0, 0],
                
                [0, 5, 0, 0, 0, 7, 0, 0, 0],
                [0, 0, 0, 0, 4, 5, 7, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 3, 0],
                
                [0, 0, 1, 0, 0, 0, 0, 6, 8],
                [0, 0, 8, 5, 0, 0, 0, 1, 0],
                [0, 9, 0, 0, 0, 0, 4, 0, 0]
            ];

        /***** 16x16 *****/
            // [
            //     [0,  7,  0, 6,  0,  15, 11, 2,  0,  0,  0,  5,  0,  14, 0,  9],
            //     [0,  0,  15, 0,  0,  8,  0,  0,  0,  0,  7,  14, 0, 0,  5,  6],
            //     [5,  0, 11, 0,  6,  0,  12, 16, 9,  3,  1,  0,  4,  0,  2,  8],
            //     [12, 1,  0,  0,  14, 4,  5,  0,  0,  0, 0, 2,  0,  3,  0,  0],
            //     [3,  15, 14, 0,  9,  10, 0,  12, 5,  2,  0,  0,  0,  0,  11, 4],
            //     [16, 0,  0,  0,  0,  0,  0,  8,  11, 12, 0, 15, 3,  0,  0,  0],
            //     [9,  0,  0,  0,  15, 0, 3,  0,  1,  0,  0,  0,  2,  7,  0,  0],
            //     [6,  12, 0,  0,  2,  16, 1,  0,  0,  0,  0,  9,  14, 0,  8,  0],
            //     [15, 2,  0,  12, 0,  0,  10, 14, 7,  6,  5,  0,  0,  13, 0,  0],
            //     [0, 4,  0,  0,  0,  1,  2,  0,  0,  10, 0,  0,  9,  8,  3,  14],
            //     [1,  0,  0,  0,  11, 0,  9,  5,  0,  0,  0,  0,  15, 0,  0,  2],
            //     [0,  0,  6,  14, 0, 0,  4,  15, 0,  9,  11, 3,  1,  0,  7,  5],
            //     [7,  5,  4,  9,  12, 0,  16, 1,  0,  0,  0, 0, 8,  2,  0,  3],
            //     [0,  6,  0,  15, 0,  11, 0,  0,  16, 1,  0,  0,  0,  0,  0,  0],
            //     [0,  11, 1,  0,  0,  0,  0,  3,  8,  5,  0,  0,  6,  4,  0, 7],
            //     [8,  16, 0,  2,  5,  9,  14, 0,  3,  0,  6,  0, 0,  0,  15, 0]
            // ];

        for i in 0..N {
            for j in 0..N {
                mat[i as usize][j as usize][0] = 
                    mmat[i as usize][j as usize];
            }
        }
    }

    // /***** Auto initialize *****/
    // {
    //     let mut rng = rand::thread_rng();
    //     let mut n: u8 = 1;
    //     
    //     for i in 0..NN {
    //         for j in 0..NN {
    //             mat[(i * NN + rng.gen_range(0, NN)) as usize]
    //                 [(j * NN + rng.gen_range(0, NN)) as usize]
    //                 [0] = n;
    //             
    //             n += 1;
    //         }
    //     }
    // }

    /***** Print *****/
    println!("N = {}\n", N);
    println!("Start:");
    _print_mat_2d(&mat);
    // _print_mat_3d(&mat);

    /***** Solving *****/
    {
        // ***** Start *****
        let start = Instant::now();

        
        // _kouho_kanri(&mut mat);
        
        // _normal_backtracking(&mut mat, 0);    
        // _hybrid_backtracking(&mut mat, 0);  
        _hybrid_backtracking_v2(&mut mat, &mut mask_stack, 0); 

        
        let end = start.elapsed();

        println!("Time: {}.{}sec\n", end.as_secs(), end.subsec_nanos());
    }
 
    // _print_mat_2d(&mat);
    // _print_mat_3d(&mat);
}



