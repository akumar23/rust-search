use std::io;
use std::fs::{File, self};
use xml::reader::{XmlEvent, EventReader};

fn read_xml(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();
    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
        }
    }

    Ok(content)
}

fn main() -> io::Result<()> {
    //let file_path = "docs.gl/gl4/glClear.xhtml";
    let dir_path = "docs.gl/gl4";
    //println!("{content}", content=read_xml(file_path).expect("TODO"));
    let dir = fs::read_dir(dir_path)?;
    for file in dir {
        println!("{file:?}", file=file?.path());
    }
    Ok(())
}
