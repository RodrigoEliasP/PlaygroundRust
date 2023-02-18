static VEGETABLES: [&'static str; 3] = ["Asparagus", "carrots", "potatoes"];

pub fn print_all_vegetables(){
    let all_vegetables = VEGETABLES
        .join(" and ")
        .replacen(" and ", ", ", VEGETABLES.len() - 2);
    println!("{}", all_vegetables);
}