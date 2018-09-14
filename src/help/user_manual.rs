use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ord;

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

struct UserManual<StoreType> {
    storage: StoreType,
}

struct MetaPage<NameType, ImportanceType>
    where NameType: Hash + Eq, ImportanceType: Ord {
    name: NameType,
    importance: ImportanceType,
    indexed: bool,
}

struct Page<PageNameType, PageImportanceType, TextType>
    where PageNameType: Hash + Eq, PageImportanceType: Ord {
    meta: MetaPage<PageNameType, PageImportanceType>,
    paragraphs: Vec<TextType>
}



