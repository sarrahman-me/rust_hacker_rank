/*
 * Complete the 'compare_triplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut bob: i32 = 0;
    let mut alice: i32 = 0;

    for n in 0..a.len() {
        if a[n] > b[n] {
            alice += 1;
        } else if a[n] < b[n] {
            bob += 1;
        }
    }

    result.push(alice);
    result.push(bob);

    result
}

#[test]
fn test_compare_triplets() {
    let alice_point: [i32; 3] = [1, 2, 3];
    let bob_point: [i32; 3] = [3, 2, 1];

    let result: Vec<i32> = compare_triplets(&alice_point, &bob_point);

    println!("{:?}", result)
}
