fn many_outputs(option: &str) -> usize {
    if option == "one" {
        1
    } else if option == "two" {
        2
    } else {
        0
    }
}


fn main() {

    println!("option: {}", many_outputs("one"));

}




