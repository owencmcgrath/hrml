<h2 align="center"> Home Row Markup Language</h1>
  
<h3 align="center"> A lightweight, Rust-powered, Markup language where your fingers never have to leave the homerow. </h3>

<p align="center"> I am a frequent Markdown user, with all of my notes being taken in Obsidian, but I found myself slowing down when typing `#` for headings, or `[]()` for URL's. My solution to this was to make my own Markup language where you never have to leave the home row. </p>

<h3 align="center"> Syntax </h4>

<div align="center">

| Feature | Syntax | Output |
|---------|--------|--------|
| Heading | `jf Title` | `<h1>Title</h1>` |
| Bold | `js text sj` | `<strong>text</strong>` |
| Italic | `jd text dj` | `<em>text</em>` |
| Underline | `ju text uj` | `<u>text</u>` |
| List Item | `ja Item` | `<li>Item</li>` |
| Ordered List Item | `jl Item` | `<li>Item</li>` |
| Link | `jg text gh url hg` | `<a href="url">text</a>` |
| Image | `jh alt gh url hj` | `<img src="url" alt="alt"/>` |
| Code Block | `jkd lang code dkj` | `<pre><code class="language-lang">code</code></pre>` |
| Horizontal Rule | `js` | `<hr>` |

</div>

<h3 align = "center"> Demo </h3>



https://github.com/user-attachments/assets/d6626b35-5a52-4ea9-8148-bc5f7e817ad5



<h3> Roadmap </h3>

- [ ] Add more code block functionality
- [ ] Add this as an Obsidian extension
- [ ] Literally refactor the whole codebase, lol
      
