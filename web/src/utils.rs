pub fn change_title(new_title: &str) {
    let head = gloo_utils::head();
    let title_element = head
        .get_elements_by_tag_name("title")
        .item(0)
        .or_else(|| {
            let element = gloo_utils::document()
                .create_element("title")
                .ok()?;
                
            head.append_child(&element).ok()?;
            Some(element)
        });
        
    if let Some(element) = title_element {
        element.set_text_content(Some(new_title));
    }
}
