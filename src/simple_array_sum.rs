/*
 * Complete the 'simple_array_sum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

fn simple_array_sum(ar: &[i32]) -> i32 {
    let mut result: i32 = 0;

    for n in ar {
        result += n;
    }

    result
}

#[test]
fn test_sum() {
    let angka: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = simple_array_sum(&angka);

    println!("{}", result);
}
