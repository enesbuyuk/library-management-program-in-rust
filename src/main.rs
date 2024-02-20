use std::io;
enum Publication {
    Book(Book),
    Magazine(Magazine),
}
struct Book {
    title: String,
    author: String,
    page_count: u32,
}
struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}
fn main() {
    let default_book = Book {
        title: "Martin Eden".to_string(),
        author: "Jack London".to_string(),
        page_count: 480,
    };
    let default_magazine = Magazine {
        title: "Kafka".to_string(),
        issue: 10,
        topic: "Literature".to_string(),
    };
    let mut publications = vec![Publication::Book(default_book), Publication::Magazine(default_magazine)];
    show_menu(&mut publications);
}
fn show_menu(publications: &mut Vec<Publication>) {
    loop {
        println!("*** MENU ***\n1) List Publications\n2) Add Book\n3) Add Magazine\n4) Quit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Reading error!");
        match input.trim() {
            "1" => { list_publications(publications) }
            "2" => { add_book(publications) }
            "3" => { add_magazine(publications) }
            "4" => {
                println!("Quitting program.");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
fn list_publications(publications: &mut Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(ref book) => {
                println!(
                    "Book: {}, Author: {}, Pages: {}",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(ref magazine) => {
                println!(
                    "Magazine: {}, Issue: {}, Topic: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}
fn add_book(mut publications: &mut Vec<Publication>) {
    println!("Enter the title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Reading error!");

    println!("Enter the author:");
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Reading error!");

    println!("Enter the page count:");
    let mut page_count_str = String::new();
    io::stdin().read_line(&mut page_count_str).expect("Reading error!");

    let page_count: u32 = page_count_str.trim().parse().expect("Invalid input for page count!");
    let book = Book {
        title: title.trim().to_string(),
        author: author.trim().to_string(),
        page_count,
    };
    publications.push(Publication::Book(book));
    show_menu(&mut publications);
}
fn add_magazine(mut publications: &mut Vec<Publication>) {
    println!("Enter the title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Reading error!");

    println!("Enter the issue:");
    let mut issue_str = String::new();
    io::stdin().read_line(&mut issue_str).expect("Reading error!");

    println!("Enter the topic:");
    let mut topic = String::new();
    io::stdin().read_line(&mut topic).expect("Reading error!");

    let issue: u32 = issue_str.trim().parse().expect("Invalid input for issue!");
    let magazine = Magazine {
        title: title.trim().to_string(),
        issue: issue,
        topic: topic.trim().to_string(),
    };
    publications.push(Publication::Magazine(magazine));
    show_menu(&mut publications);
}