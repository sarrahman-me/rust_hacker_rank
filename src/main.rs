use std::io;

enum Order {
    Pizza,
    Drink,
    Burger,
}

struct OrderInfo {
    order_id: usize,
    customer_name: String,
    order: Order,
}

fn create_order(customer_name: String, menu: Order, list: &mut Vec<OrderInfo>) {
    let order_id: usize = list.len() + 1;

    let pesanan: OrderInfo = OrderInfo {
        customer_name,
        order: menu,
        order_id,
    };

    list.push(pesanan);
}

fn show_order(list: &Vec<OrderInfo>) {
    println!("Daftar pesanan :");

    for o in list {
        let menu: &str = match o.order {
            Order::Burger => "Burger",
            Order::Drink => "Drink",
            Order::Pizza => "Pizza",
        };
        println!(
            "ID {}, pemesan {} dengan menu {}",
            o.order_id, o.customer_name, menu
        )
    }
}

fn pertanyaan_pesanan() -> Result<(String, Order), String> {
    println!("Masukkan nama anda :");
    let mut customer_name: String = String::new();
    io::stdin()
        .read_line(&mut customer_name)
        .expect("Invalid input");

    println!("Kamu ingin pesan apa {} :", &customer_name);
    println!("1.Pizza");
    println!("2.Burger");
    println!("3.Drink");

    let mut input_order: String = String::new();
    io::stdin()
        .read_line(&mut input_order)
        .expect("Invalid input");
    let angka: u8 = input_order.trim().parse().expect("Angka tidak valid");

    let menu = match angka {
        1 => Order::Pizza,
        2 => Order::Burger,
        3 => Order::Drink,
        _ => return Err(String::from("Pesanan tidak ada")),
    };

    Ok((customer_name, menu))
}

fn main() {
    let mut list_order: Vec<OrderInfo> = Vec::new();

    loop {
        println!("----------------");
        println!("Pilih nomor : ");
        println!("1.Buat pesanan");
        println!("2.Lihat pesanan");
        println!("3.keluar");

        // mendapatkan pilihan
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        let angka: u8 = input.trim().parse().expect("Angka tidak valid");

        match angka {
            1 => match pertanyaan_pesanan() {
                Ok((customer_name, menu)) => create_order(customer_name, menu, &mut list_order),
                Err(e) => println!("{}", e),
            },
            2 => show_order(&list_order),
            3 => {
                println!("selamat datang kembali");
                break;
            }
            _ => return println!("invalid"),
        }
    }
}
