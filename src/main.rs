// make csv crate available
use csv;
// import the print_hello function from the hello_print crate
use hello_print::print_hello;

fn main() {
    print_hello("from");

    let mut rdr = csv::Reader::from_reader(std::io::stdin());
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.
        println!("{:?}", record);
    }
}
