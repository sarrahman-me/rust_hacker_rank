/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

fn a_very_big_sum(ar: &[i64]) -> i64 {
    let mut result: i64 = 0;

    for n in ar {
        result += n;
    }

    result
}

#[test]
fn test_a_very_big_sum() {
    let big_num: [i64; 5] = [1000000001, 1000000002, 1000000003, 1000000004, 1000000005];

    let result: i64 = a_very_big_sum(&big_num);

    println!("{}", result)
}
