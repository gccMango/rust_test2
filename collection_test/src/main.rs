use std::collections::HashMap;


fn main() {
    // let days= ["sunday","monday","tuseday","wednesday","thursday","friday","saturday"];

    // let bytes = [0;5];

    // let first = days[0];
    // let second = days[1];

    let three_nums = vec![15,3,46];
    println!("initial vector: {:?}",three_nums);

    let zeros = vec![0;5];
    println!("Zeros: {:?}",zeros);

    let mut fruit = Vec:: new();
    
    // push values onto end of vector, type changes from generic 'T' to String

    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("cherry");
    println!("Fruits: {:?}",fruit);

    // pop off value at end of vector
    // call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    let mut index_vec = vec![15,3,46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);

    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert("roman history".to_string(), "very accurate".to_string());
    reviews.insert("cooking with roro".to_string(), "sweet recipes".to_string());
    reviews.insert("programming in rust".to_string(),"Great example".to_string());

    let book: &str =  "programming in rust";
    println!("\nReview for \'{}\' : {:?}", book, reviews.get(book));

    let obsolete: &str = "roman history";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    println!("\nReview for \'{}\' : {:?}", obsolete,reviews.get(obsolete));
    





}
