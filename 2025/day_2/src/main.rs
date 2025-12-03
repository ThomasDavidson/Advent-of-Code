use library::input::{Day, InputType};

type Id = u64;

#[derive(Debug)]
struct GiftShop {
    product_id_ranges: Vec<ProductIdRange>,
}
impl GiftShop {
    fn parse(text: &str) -> Self {
        let product_id_ranges = text.split(',').map(ProductIdRange::parse).collect();

        Self { product_id_ranges }
    }
}

#[derive(Debug)]
struct ProductIdRange {
    start: Id,
    end: Id,
}
impl ProductIdRange {
    fn parse(text: &str) -> Self {
        let (start, end) = text.split_once('-').unwrap();

        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        Self { start, end }
    }
    fn get_invalid_ids(&self, is_valid: &dyn Fn(Id) -> bool) -> Vec<Id> {
        (self.start..=self.end)
            .filter(|arg0: &Id| !is_valid(*arg0))
            .collect()
    }
}

fn part_1_valid_id(number: Id) -> bool {
    let formated_number = number.to_string();
    let len = formated_number.len();

    // all odd lengths are valid?
    if len % 2 == 1 {
        return true;
    }

    let (first, second) = formated_number.split_at(len / 2);

    first != second
}
fn part_2_valid_id(number: Id) -> bool {
    let formated_number = number.to_string().chars().collect::<Vec<char>>();
    let len = formated_number.len();

    // iterate over factors of the length of the number
    for i in (1..=(len / 2)).filter(|i| len % i == 0) {
        let mut chunks = formated_number[..].chunks(i);

        let first_chunk = chunks.next().unwrap();

        if chunks.all(|chunk| chunk == first_chunk) {
            return false;
        }
    }
    true
}

struct Day1;
const DAY: Day1 = Day1;
impl Day<Id> for Day1 {
    fn part_1(&self, input: &str) -> Id {
        let mut part_1_answer = 0;

        let gift_shop = GiftShop::parse(input);

        for product_id_ranges in &gift_shop.product_id_ranges {
            part_1_answer += product_id_ranges
                .get_invalid_ids(&part_1_valid_id)
                .iter()
                .sum::<Id>();
        }

        part_1_answer
    }
    fn part_2(&self, input: &str) -> Id {
        let mut part_2_answer = 0;

        let gift_shop = GiftShop::parse(input);

        for product_id_ranges in &gift_shop.product_id_ranges {
            part_2_answer += product_id_ranges
                .get_invalid_ids(&part_2_valid_id)
                .iter()
                .sum::<Id>();
        }

        part_2_answer
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
