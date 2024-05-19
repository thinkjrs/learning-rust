fn bigger_sum<'a, T>(first: &'a Vec<T>, second: &'a Vec<T>) -> &'a Vec<T>
where
    T: 'a + std::iter::Sum<&'a T> + std::cmp::PartialOrd + std::fmt::Display + Clone,
{
    // find sums of each
    let sum_first: T = first.iter().sum();
    let sum_second: T = second.iter().sum();
    // return vector with larger sum
    if sum_first > sum_second {
        println!("The first vector has a larger sum: {}", sum_first);
        first
    } else {
        println!("The second vector has a larger sum: {}", sum_second);
        second
    }
}
struct PintGlass<T>
where
    T: std::cmp::PartialOrd,
{
    beer: BeerType,
    price: T,
    is_empty: bool,
}

impl<T> PintGlass<T>
where
    T: std::cmp::PartialOrd,
{
    fn set_to_empty(&mut self) {
        self.is_empty = true;
    }
}

pub trait Display {
    fn print(&self);
}
impl<T> Display for PintGlass<T>
where
    T: std::cmp::PartialOrd + std::fmt::Display,
{
    fn print(&self) {
        println!(
            "{}",
            format!(
                "Beer {:?}, price {}, is empty? {}",
                self.beer, self.price, self.is_empty
            )
        );
    }
}
#[derive(Debug)]
enum BeerType {
    IPA,
    Kolsch,
    Lager,
    Stout,
    Sour,
}
fn use_bigger_sum() {
    let first: Vec<u16> = vec![1, 2, 3, 4];
    let second: Vec<u16> = vec![1, 2, 3, 4, 5];

    bigger_sum(&first, &second);

    {
        let third: Vec<u16> = vec![2, 3, 4, 5];
        bigger_sum(&second, &third);
    }
    {
        let third: Vec<usize> = vec![2, 3, 4, 5];
        let forth: Vec<usize> = vec![2, 3, 4, 5];
        bigger_sum(&third, &forth);
    }
}
fn main() {
    let first_pint = PintGlass {
        beer: BeerType::IPA,
        price: 5,
        is_empty: true,
    };

    let second_pint = PintGlass {
        beer: BeerType::Stout,
        price: 6,
        is_empty: true,
    };

    // there's a deal with the third pint that the restaurant pays the customer
    // 1 unit of currency
    let third_pint = PintGlass {
        beer: BeerType::Kolsch,
        price: -1,
        is_empty: true,
    };

    // then because the customer is drunk they double charge them
    // let's call this establishment "VC Bar"
    let mut forth_pint = PintGlass {
        beer: BeerType::Lager,
        price: 12,
        is_empty: false,
    };

    forth_pint.set_to_empty();

    // though shady, the business model obviously works
    let fifth_pint = PintGlass {
        beer: BeerType::IPA,
        price: 12,
        is_empty: false,
    };

    let pints = vec![first_pint, second_pint, third_pint, forth_pint, fifth_pint];
    let mut total_sales: i32 = 0;
    for pint in pints.iter() {
        total_sales += pint.price;
    }
    println!(
        "The customer has paid {} to get black out drunk",
        total_sales
    );

    pints[4].print();
}
