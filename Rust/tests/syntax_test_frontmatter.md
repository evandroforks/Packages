#! SYNTAX TEST "Packages/Markdown/Markdown.sublime-syntax"

1. shebang

   ```rust
   #!/usr/bin/env cargo
   #! <- markup.list.numbered.markdown markup.raw.code-fence.rust.markdown-gfm source.rust comment.line.shebang.rust punctuation.definition.comment.rust
   #!^^^^^^^^^^^^^^^^^^^ markup.list.numbered.markdown markup.raw.code-fence.rust.markdown-gfm source.rust comment.line.shebang.rust - punctuation
   #!             ^^^^^ constant.language.shebang.rust
   ```

2. frontmatter

   ```rust
   ---
   #! <- markup.list.numbered.markdown markup.raw.code-fence.rust.markdown-gfm source.rust meta.frontmatter.rust punctuation.section.frontmatter.begin.rust
   [section]
   #! <- markup.list.numbered.markdown markup.raw.code-fence.rust.markdown-gfm source.rust meta.frontmatter.rust source.toml.embedded.rust source.toml punctuation.definition.table.begin.toml
   key = "value"
   #! <- markup.list.numbered.markdown markup.raw.code-fence.rust.markdown-gfm source.rust meta.frontmatter.rust source.toml.embedded.rust source.toml meta.tag.key.toml entity.name.tag.toml
   ---
   #! <- markup.list.numbered.markdown markup.raw.code-fence.rust.markdown-gfm source.rust meta.frontmatter.rust punctuation.section.frontmatter.end.rust
   
   fn main() {}
   #! <- markup.list.numbered.markdown markup.raw.code-fence.rust.markdown-gfm source.rust meta.function.rust keyword.declaration.function.rust
   #!^^^^^^^^^^ source.rust meta.function.rust
   ```
