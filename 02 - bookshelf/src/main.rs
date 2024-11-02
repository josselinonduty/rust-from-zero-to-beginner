use core::fmt;
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
struct Book {
    name: String,
    author: Option<String>,
    release_year: Option<u32>,
}

impl Book {
    fn new(name: String, author: Option<String>, release_year: Option<u32>) -> Self {
        Self {
            name,
            author,
            release_year,
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(author) = &self.author {
            if let Some(release_year) = &self.release_year {
                write!(f, "{}, {} ({})", self.name, author, release_year)
            } else {
                write!(f, "{}, {}", self.name, author)
            }
        } else if let Some(release_year) = &self.release_year {
            write!(f, "{}, {}", self.name, release_year)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[derive(Debug)]
struct Bookshelf {
    rows: u32,
    cols: u32,
    books: Vec<Book>,
}

impl Bookshelf {
    fn new(rows: u32, cols: u32) -> Self {
        Self {
            rows,
            cols,
            books: vec![],
        }
    }

    fn add(&mut self, book: Book) -> Result<(), String> {
        if self.count() >= self.rows * self.cols {
            return Err("Bookshelf is full".to_string());
        }

        self.books.push(book);
        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<(), String> {
        for (book_id, book) in self.books.iter().enumerate() {
            if book.name == name {
                self.books.remove(book_id);
                return Ok(());
            }
        }

        Err(format!("Book \"{name}\" does not exist.").to_string())
    }

    fn consult(&self, name: &str) -> Result<&Book, String> {
        for book in self.books.iter() {
            if book.name == name {
                return Ok(book);
            }
        }

        Err(format!(r#"Book "{name}" does not exist."#).to_string())
    }

    fn count(&self) -> u32 {
        self.books.len() as u32
    }

    fn burn(&mut self) {
        self.books.clear();
    }
}

impl fmt::Display for Bookshelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Occupation: {:.2}% ({}/{})",
            self.count() as f32 / (self.cols * self.rows) as f32 * 100.0,
            self.count(),
            self.cols * self.rows
        )
    }
}

fn display_menu() {
    println!("-----------------");
    println!("Book Manager");
    println!("(1) Add a book");
    println!("(2) Remove a book");
    println!("(3) Consult a book");
    println!("(4) Display the bookshelf");
    println!("(5) Burn the bookshelf down");
    println!("(q) Quit");
    println!("(h) Help");
    println!("-----------------");
}

fn main() {
    let mut bookshelf = Bookshelf::new(4, 32);

    display_menu();

    loop {
        print!("choice>> ");
        stdout().flush().expect("Error: Could not flush stdout.");

        let mut choice = String::new();
        stdin()
            .read_line(&mut choice)
            .expect("Error: Could not stdin.");

        choice = choice.trim().to_string();

        match &choice[..] {
            "q" | "quit" | "e" | "exit" => break,
            "h" | "help" => {
                display_menu();
                continue;
            }
            _ => (),
        }

        let choice: u8 = match choice.parse() {
            Err(_) => continue,
            Ok(input) => input,
        };

        match choice {
            1 => {
                print!("name> ");
                stdout().flush().expect("Error: Could not flush stdout.");

                let mut name = String::new();
                stdin()
                    .read_line(&mut name)
                    .expect("Error: Could not read book name.");
                let name = name.trim();

                print!("author> ");
                stdout().flush().expect("Error: Could not flush stdout.");

                let mut author = String::new();
                stdin()
                    .read_line(&mut author)
                    .expect("Error: Could not read book author.");
                let author = author.trim();

                print!("release_year> ");
                stdout().flush().expect("Error: Could not flush stdout.");

                let mut release_year = String::new();
                stdin()
                    .read_line(&mut release_year)
                    .expect("Error: Could not read book release_year.");
                let release_year = release_year.trim();

                let book = Book::new(
                    name.to_string(),
                    if author == "" {
                        None
                    } else {
                        Some(author.to_string())
                    },
                    if release_year == "" {
                        None
                    } else {
                        let release_year: u32 = release_year
                            .parse()
                            .expect("Error: Could not parse release_year.");
                        Some(release_year)
                    },
                );
                match bookshelf.add(book) {
                    Ok(_) => (),
                    Err(message) => {
                        println!("{message}");
                        break;
                    }
                };
            }
            2 => {
                print!("book> ");
                stdout().flush().expect("Error: Could not flush stdout.");

                let mut name = String::new();
                stdin()
                    .read_line(&mut name)
                    .expect("Error: Could not read book name.");

                match bookshelf.remove(&name.trim()) {
                    Err(message) => println!("{message}"),
                    Ok(_) => (),
                };
            }
            3 => {
                print!("book> ");
                stdout().flush().expect("Error: Could not flush stdout.");

                let mut name = String::new();
                stdin()
                    .read_line(&mut name)
                    .expect("Error: Could not read book name.");

                match bookshelf.consult(&name.trim()) {
                    Ok(book) => {
                        println!("Book: {book}");
                    }
                    Err(message) => println!("{message}"),
                };
            }
            4 => {
                println!("{bookshelf}");

                for book in bookshelf.books.iter() {
                    println!("- {book}");
                }
            }
            5 => {
                bookshelf.burn();
                println!("Bookshelf burned down successfully.");
            }
            i => {
                println!("Warning: {i} is not a valid option. Please retry.");
            }
        }
    }
}
