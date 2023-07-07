use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {

         "ᱟᱨᱮ" => "Err",
        "ᱴᱷᱤᱠᱜᱮᱭᱟ" => "Ok",
        "ᱵᱟᱵᱮᱨ" => "String",
        "ᱦᱮᱱᱟᱠᱥᱟ" => "HashMap",
        "ᱟᱩᱱᱠᱟᱜᱮ" => "Default",
        "ᱜᱟᱞᱛᱤ" => "Error",
        "ᱠᱩᱥᱤ" => "Option",
        "ᱴᱷᱩᱨᱟᱜᱟᱱ" => "Some",
        "ᱵᱟᱱᱜ" => "None",
        "ᱚᱨᱡᱚ" => "Result",
        "ᱤᱱᱜ" => "Self",
        "ᱚᱞ" => "println",
        "ᱪᱦᱟᱯᱟ" => "print",
        "ᱨᱟᱹᱯᱩᱫ" => "break",
        "ᱟᱩᱱᱩᱢ" => "async",
        "ᱛᱟᱹᱱᱜᱤ" => "await",
        "ᱞᱚᱯ" => "loop",
        "ᱟᱪᱚᱜ" => "move",
        "ᱵᱮᱱᱟᱭ" => "crate",
        "ᱪᱚᱫ_ᱵᱟᱱᱜᱴᱮᱭᱟᱞᱮᱱᱟ" => "unreachable_code",
        "ᱞᱮᱠᱟ" => "as",
        "ᱞᱟᱜᱚᱣᱨᱮ" => "const",
        "ᱥᱟᱝᱟ" => "trait",
        "ᱵᱚᱛᱚᱨᱜᱮ" => "unsafe",
        "ᱵᱷᱤᱛᱟᱹᱨ" => "in",
        "ᱠᱷᱟᱡ" => "from",
        "ᱥᱟᱨ" => "dyn",
        "ᱵᱟᱱᱜᱛᱚᱞ" => "unwrap",
        "ᱟᱩᱱᱠᱟᱜᱮᱮ" => "default",
        "ᱞᱮᱠᱟ_ᱨᱮᱝ" => "as_ref",
        "ᱤᱚ" => "io",
        "ᱱᱟᱷᱤᱞ" => "extern",
        "ᱜᱟᱞᱛᱤᱮ" => "false",
        "ᱠᱟᱹᱡᱤ" => "fn",
        "ᱵᱮᱥ" => "super",
        "ᱵᱚᱞᱚ" => "insert",
        "ᱱᱟᱢ" => "get",
        "ᱦᱩᱠᱩᱢ_ᱮᱢ" => "allow",
        "ᱵᱩᱛᱚᱨ" | "ᱞᱟᱭ" | "ᱜᱤᱛᱤ" => "panic",
        "ᱦᱟᱹᱴᱤᱧ" => "mod",
        "ᱡᱤᱞ" => "mut",
        "ᱱᱟᱣᱟ" => "new",
        "ᱟᱠᱟᱨᱮ" => "where",
        "ᱞᱟᱹᱜᱤᱫ" => "for",
        "ᱱᱟᱢᱢᱮ_ᱟᱨ_ᱟᱫᱤᱨᱢᱮ" => "get_or_insert_with",
        "ᱟᱥᱚᱞ" => "main",
        "ᱦᱚᱲ" => "pub",
        "ᱪᱮᱫ" => None?,
        "ᱨᱩᱟᱨ" => "return",
        "ᱮᱥᱮᱨ" => "impl",
        "ᱟᱨᱩ" => "ref",
        "ᱡᱩᱨᱚᱣᱫ" => "match",
        "ᱡᱩᱫᱤ" => "if",
        "ᱮᱨᱨ" => "else",
        "ᱤᱱᱜᱛᱮ" => "self",
        "ᱟᱹᱞᱟᱹᱭ" => "let",
        "ᱠᱟᱛᱤᱪ" => "static",
        "ᱟᱩᱨᱟ" => "struct",
        "ᱦᱩᱫᱤᱪ" => "expect",
        "ᱴᱤᱱᱦᱟᱨᱮ" => "while",
        "ᱞᱟᱭᱟᱣ" => "use",
        "ᱟᱢ" => "into",
        "ᱥᱟᱹᱨᱮ" => "true",
        "ᱞᱮᱠᱷᱟ" => "enum",
        "ᱜᱟᱫᱮᱞ" => "Group",
        "ᱩᱯᱨᱩᱢ" => "Ident",
        "ᱯᱚᱭᱥᱟᱟᱛᱩᱜ" => "TokenStream",
        "ᱯᱚᱭᱥᱟᱫᱟᱨᱤ" => "TokenTree",
        "ᱛᱮᱫ_ᱵᱟᱵᱮᱨ" => "to_string",
        "ᱞᱮᱠᱟ_ᱵᱟᱨ" => "as_str",
        "ᱫᱚᱦᱚ" => "span",
        "ᱥᱟᱡᱚᱣ" => "Vec",
        "ᱟᱛᱩᱜ" => "stream",
        "ᱫᱷᱟᱠᱞᱟᱣ" => "push",
        "ᱯᱟᱥᱱᱟᱚᱣ" => "extend",
        "ᱱᱩᱱᱫᱤᱜᱮ" => "delimiter",
        "ᱢᱩᱪᱟᱹᱫᱽ" => "Punct",
        "ᱵᱚᱱᱫᱟ" => "Literal",
        "ᱮᱞ" => "num",
        "ᱥᱮᱞᱮᱫ" => "sum",
        "ᱨᱮᱦᱮᱫ" => "root",
        "ᱥᱛᱫ" => "std",
        "ᱫᱚᱞ" => "rand",
        "ᱩᱨᱟᱹᱞ" => "guess",
        "ᱪᱦᱟᱵᱟ" => "cmp",
        "ᱪᱩᱠᱩ_ᱮᱞ" => "secret_number",
        "ᱱᱟᱦᱤᱞ" => "stdin",
        "ᱯᱟᱲᱦᱟᱣ_ᱫᱟᱠ" => "read_line",
        "ᱥᱩᱛᱚᱢ_ᱨᱱᱜ" => "thread_rng",
        "ᱨᱱᱜ" => "Rng",
        "ᱦᱚᱦᱚ" => "Ordering",
        "ᱠᱚᱢ" => "Less",
        "ᱞᱟᱛᱩ" => "Greater",
        "ᱥᱩᱢᱟᱱ" => "Equal",
        "ᱜᱮᱱ_ᱡᱷᱤᱞ" => "gen_range",
        "ᱞᱮᱛᱟᱲ" => "continue",
        "ᱪᱦᱟᱯᱟ_ᱯᱮᱠᱩᱱᱟ" => "print_triangle",
        "ᱢᱮᱫ_ᱞᱟᱫ" => "rows",
        "ᱦᱚᱭᱚ" => "trim",
        "ᱯᱟᱨᱠᱚᱢ" => "parse",
        "ᱵᱚᱪᱷᱟᱨ" => "year",
        "ᱱᱮᱭᱟ_ᱫᱟᱱ_ᱵᱚᱪᱷᱟᱨ" => "is_leap_year",
        "ᱮᱛᱟᱦᱚᱵ" => "start",
        "ᱢᱩᱪᱟᱹᱫ" => "end",
        "ᱫᱚᱵᱚᱲ_ᱧᱟᱢ" => "product",
        "ᱫᱟᱠ" => "line",
        "ᱡᱩᱛ" => "debug",
        "ᱨᱮᱜᱮᱽ" => "regex",
        "ᱱᱚᱨᱢᱟᱞ_ᱤᱱᱯᱩᱛ" => "normalized_input",
        "ᱱᱚᱨᱢᱞ_ᱤᱱᱯᱩᱛ" => "normalize_input",
        "ᱟᱭᱞᱛᱚᱣᱛᱮ_ᱤᱱᱯᱩᱛ" => "reversed_input",
         "ᱤᱱᱯᱩᱛ" => "input",
          "ᱪᱦᱮᱫᱢᱮᱢᱮᱱᱫᱟ" => "response",
          "ᱝᱚᱨᱢᱟᱛ" => "format",
         "ᱢᱩᱪᱟᱹᱫ_ᱥᱮ" => "ends_with",
         "ᱟᱭᱞᱛᱚᱣᱛᱮ_ᱨᱚᱟᱨ" => "reverse_pronouns",
         "ᱱᱟᱠᱥᱟ" => "map",
         "ᱟᱚᱪᱦᱚ_ᱯᱩᱱᱚᱛ" => "replace_phrases",
        "ᱵᱟᱨ" => "str",
        "ᱟᱚᱩᱪᱷᱚᱜᱮ" => "replacement",
        "ᱯᱨᱮᱡᱩᱛ" => "prefix",
        "ᱡᱚᱛᱚ_ᱟᱚᱩᱪᱦᱚ" => "replace_all",
        "ᱠᱩᱞᱥᱤ" => "choices",
        "ᱵᱟᱪᱷᱟᱣ" => "choose",
        "ᱞᱟᱹᱱᱟᱹᱭ" => "responses",
        "ᱟᱹᱭᱟᱹᱛ" => "sentences",
        "ᱡᱮᱥᱟ" => "join",
        "ᱝᱢᱛ" => "fmt",
        "ᱥᱟᱛᱟᱢ" => "subject",
        "ᱞᱟᱵᱮᱞ" => "label",
        "ᱠᱟᱛᱷᱟ" => "sentence",
        "ᱥᱮᱱᱫᱨᱟ_ᱤᱛᱮᱨ" => "find_iter",
        "ᱡᱟᱨᱣᱟ" => "collect",
        "ᱯᱟᱴᱮᱨᱱ" => "patterns",
        "ᱯᱩᱨᱚᱣ_ᱯᱟᱴᱮᱨᱱ" => "full_pattern",
        "ᱯᱮᱨᱮᱷ_ᱯᱩᱱᱚᱛ" => "filler_phrases",
        "ᱛᱮᱨᱟᱱᱜ" => "target",
        "ᱱᱟᱢ_ᱞᱟᱹᱱᱟᱹᱭ_ᱛᱮᱫ" => "get_response_to",
        "ᱡᱚᱱᱚᱢ_ᱦᱟᱢᱟᱞ_ᱨᱚᱨ" => "gen_weighted_bool",
        "ᱛᱮᱫ_ᱠᱟᱛᱤᱪ" => "to_lowercase",
 
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}


fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn bonga(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
