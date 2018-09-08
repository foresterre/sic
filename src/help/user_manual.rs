use std::collections::HashMap;

// Experimental page structure:
//
//  - user_manual:
//    - index:
//      - [p.page.name for p in user_manual.pages] # implicit
//    - pages:
//      - page:
//        - name: "example"
//        - importance: 1 # :optional, default = 1
//        - paragraphs:
//          - paragraph:
//            - name: "goal"
//            - text: "The goal of the example paragraph is to have an example of a goal paragraph."
//          - paragraph:
//            - name: "syntax"
//            - text: "(example: '<text>')"
//          - paragraph:
//            - name: "see also"
//            - text: "https://example.com"
//

struct UserManual {
    // name => page :todo
    help_pages: HashMap<String, String>
}

trait Index<T> {
    fn index() -> T;
}

impl UserManual {

}

struct Page {
    name: str,
    importance: u32, // pages are sorted by importance

}

