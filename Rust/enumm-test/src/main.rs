struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcase(u32), // without name field - u32 represent the episode number.
    Placeholder,  // no fields at all.
}

fn print_media(media: &Media) {
    println!("{:#?}", media)
}

impl Media {
    fn media_description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} - {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcase(id) => {
                format!("Podcast episode: {}", id)
            }
            Media::Placeholder => format!("Placeholder"),
        }
    }
}

fn main() {
    let bk = Media::Book {
        title: String::from("Martian"),
        author: String::from("Andy W"),
    };

    print_media(&bk);

    let mv = Media::Movie {
        title: String::from("Titanic"),
        director: String::from("James Cameron"),
    };

    print_media(&mv);

    let ab = Media::Audiobook {
        title: String::from("I am legand"),
    };

    print_media(&ab);

    println!("{}", bk.media_description());
    println!("{}", mv.media_description());
    println!("{}", ab.media_description());

    let pc = Media::Podcase(4);
    let pl = Media::Placeholder;

    // Catalog operations
    let mut catalog = Catalog::new();

    catalog.add(bk);
    catalog.add(mv);
    catalog.add(pc);
    catalog.add(pl);

    println!("Items in the catalog: {}", catalog.items.len());

    match catalog.items.get(8) {
        None => {
            println!("Nothing at that index");
        }
        Some(value) => {
            print_media(value);
        }
    }
}
