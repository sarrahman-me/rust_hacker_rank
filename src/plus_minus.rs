/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let mut positive_num = 0;
    let mut negative_num = 0;
    let mut zero_num = 0;

    for &n in arr {
        if n > 0 {
            positive_num += 1;
        } else if n < 0 {
            negative_num += 1;
        } else {
            zero_num += 1;
        }
    }

    let total: f64 = arr.len() as f64;

    println!("{}", positive_num as f64 / total);
    println!("{}", negative_num as f64 / total);
    println!("{}", zero_num as f64 / total);
}

#[test]
fn test_plus_minus() {
    let angka: [i32; 6] = [-4, 3, -9, 0, 4, 1];

    plus_minus(&angka)
}
