use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize)]
struct Paragraph{
    name: String
}


#[derive(Serialize,Deserialize)]
struct Article {
    article:String,
    author:String,
    paragraph:Vec<Paragraph>
}

fn main(){

    let article:Article = Article{
        article:String::from("How to write JSON in Rust"),
        author:String::from("Vishal M Saroj"),
        paragraph: vec![
            Paragraph{
                name:String::from("Navbar of the Paragraph")
            },
            Paragraph {
                name:String::from("Main body of the Paragraph")
            },
            Paragraph {
                name:String::from("Footer of the paragraph")
            },
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("Here is the  JSON data  : {}",json);
}