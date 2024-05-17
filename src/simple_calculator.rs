use std::io;

#[test]
fn calculator() {
    println!("Masukkan angka pertama :");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Gagal membaca input");
    let num1: f64 = input1.trim().parse().expect("Harusnya angka");

    println!("Masukkan angka kedua: ");
    let mut input2: String = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Gagal membaca input");
    let num2: f64 = input2.trim().parse().expect("Angka tidak valid");

    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} - {} = {}", num1, num2, num1 - num2);
    println!("{} x {} = {}", num1, num2, num1 * num2);
    println!("{} : {} = {}", num1, num2, num1 / num2);
}
