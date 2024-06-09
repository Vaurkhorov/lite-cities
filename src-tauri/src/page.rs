use serde_json::json;
use serde::{Deserialize, Serialize};

pub enum Element {
    Paragraph(String),
    Button(String),
    Link(String),
    Input(String, String),
}

pub struct Page {
    // This needs to be Vec<ID, Element>, where ID will become the id attribute of the html element.
    elements: Vec<Element>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn get_response(self) -> Result<Response, ()> {
        let queue = self.get_queue()?;
        Ok(Response {
            queue,
        })
    }
    
    fn get_queue(self) -> Result<Vec<Command>, ()> {
        let mut queue: Vec<Command> = Vec::new();

        for element in self.elements {
            let command = match element {
                Element::Paragraph(content) => Command {
                    command: "staticElement".to_string(),
                    id: "testElement".to_string(),
                    element: "p".to_string(),
                    content,
                    event: None,
                },
                Element::Button(content) => Command {
                    command: "interactiveElement".to_string(),
                    id: "testElement".to_string(),
                    element: "button".to_string(),
                    content,
                    event: Some("click".to_string()),
                },
                Element::Link(content) => Command {
                    command: "interactiveElement".to_string(),
                    id: "testElement".to_string(),
                    element: "a".to_string(),
                    content,
                    event: Some("click".to_string()),
                },
                Element::Input(prompt, button_text) => Command {
                    command: "inputElement".to_string(),
                    id: "testElement".to_string(),
                    element: "input".to_string(),
                    content: json!({
                        "prompt": prompt,
                        "button_text": button_text
                    }).to_string(),
                    event: Some("click".to_string()),
                },
            };
            queue.push(command);
        }

        Ok(queue)
    }
}


#[derive(Serialize, Deserialize)]
pub struct Response {
    queue: Vec<Command>
}

#[derive(Serialize, Deserialize)]
struct Command {
    command: String,
    id: String,
    element: String,
    content: String,
    event: Option<String>,
}