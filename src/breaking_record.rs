/*
 * Complete the 'breaking_record' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breaking_record(scores: &[i32]) -> Vec<i32> {
    let mut min_record: i32 = scores[0];
    let mut max_record: i32 = scores[0];
    let mut min_break: i32 = 0;
    let mut max_break: i32 = 0;

    for n in 1..scores.len() {
        if scores[n] < min_record {
            min_record = scores[n];
            min_break += 1;
        } else if scores[n] > max_record {
            max_record = scores[n];
            max_break += 1;
        }
    }

    vec![max_break, min_break]
}

#[test]
fn test_breaking_record() {
    // let score: [i32; 10] = [3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
    let score2 = [10, 5, 20, 20, 4, 5, 2, 25, 1];

    let result = breaking_record(&score2);

    println!("{:?}", result)
}
