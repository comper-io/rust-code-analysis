use tree_sitter::Language;

mk_langs!(
    // 1) Name for enum
    // 2) tree-sitter function to call to get a Language
    (Kotlin, tree_sitter_kotlin_codanna),
    (Java, tree_sitter_java),
    (Rust, tree_sitter_rust),
    (Cpp, tree_sitter_cpp),
    (Python, tree_sitter_python),
    (Tsx, tree_sitter_tsx),
    (Typescript, tree_sitter_typescript),
    (Ccomment, tree_sitter_ccomment),
    (Preproc, tree_sitter_preproc),
    (Mozjs, tree_sitter_mozjs),
    (Javascript, tree_sitter_javascript),
    (Perl, tree_sitter_perl),
    (Html, tree_sitter_html),
    (Css, tree_sitter_css),
    (Php, tree_sitter_php),
    (Csharp, tree_sitter_c_sharp)
    // Note: Vue support temporarily disabled due to tree-sitter version incompatibility
    // tree-sitter-vue 0.0.3 uses tree-sitter 0.20, but this project uses tree-sitter 0.26
    // (Vue, tree_sitter_vue)
);
