use crate::page::{Element, Page, Response};

/// This is the analogue to the main() function of the game. The game will be defined here.
pub fn game() -> Result<Response, ()> {
    // this is just a demo page
    let mut page = Page::new();
    page.add_element(
        Element::Paragraph(
            "There's a button down there."
                .to_string()
        )
    );   
    page.add_element(
        Element::Link(
            "It doesn't do anything."
                .to_string()
        )
    );   
    page.add_element(
        Element::Button(
            "No really, nothing happens when you click this."
                .to_string()
        )
    );
    page.add_element(
        Element::Input(
            "Surprise textbox!".to_string(),
            "This also does nothing...".to_string()
        )
    );


    page.get_response()
}