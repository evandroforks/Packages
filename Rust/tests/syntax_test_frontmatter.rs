#! SYNTAX TEST "Rust.sublime-syntax"
---
#! <- meta.frontmatter.rust punctuation.section.frontmatter.begin.rust
#!^ meta.frontmatter.rust punctuation.section.frontmatter.begin.rust
#! ^ meta.frontmatter.rust - punctuation
[section]
#! <- meta.frontmatter.rust source.toml.embedded.rust source.toml punctuation.definition.table.begin.toml
key = "value"
#! <- meta.frontmatter.rust source.toml.embedded.rust source.toml meta.tag.key.toml entity.name.tag.toml
---
#! <- meta.frontmatter.rust punctuation.section.frontmatter.end.rust
#!^ meta.frontmatter.rust punctuation.section.frontmatter.end.rust
#! ^ meta.frontmatter.rust - punctuation

fn main() {}
#! <- meta.function.rust keyword.declaration.function.rust
#!^^^^^^^^^^ meta.function.rust
