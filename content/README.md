# Content Management

This directory contains markdown files for the blog and documentation pages.

## Structure

```
content/
├── blog/
│   └── {slug}.md          # Blog posts at /blog/{slug}
└── docs/
    └── {folder}/
        └── {slug}.md      # Docs at /docs/{folder}/{slug}
```

## Markdown Format

All markdown files must include YAML frontmatter at the top:

### Blog Posts

```markdown
---
title: Your Post Title
description: A brief description for SEO
date: 2025-12-19
author: Author Name
tags:
  - tag1
  - tag2
---

# Your content here

Write your blog post content using standard Markdown syntax.
```

### Documentation Pages

```markdown
---
title: Your Doc Title
description: A brief description for SEO
---

# Your content here

Write your documentation using standard Markdown syntax.
```

## Supported Markdown Features

The markdown parser (`pulldown-cmark`) supports:

- ✅ Headings (`#`, `##`, `###`, etc.)
- ✅ **Bold** and *italic* text
- ✅ Links `[text](url)`
- ✅ Images `![alt](url)`
- ✅ Lists (ordered and unordered)
- ✅ Code blocks with syntax highlighting
- ✅ Inline `code`
- ✅ Blockquotes
- ✅ Tables
- ✅ ~~Strikethrough~~
- ✅ Task lists `- [ ]` and `- [x]`
- ✅ Smart punctuation (curly quotes, em dashes, etc.)

## URL Structure

- Blog posts: `/blog/{slug}`
  - Example: `content/blog/welcome.md` → `/blog/welcome`
  
- Documentation: `/docs/{folder}/{slug}`
  - Example: `content/docs/getting-started/quickstart.md` → `/docs/getting-started/quickstart`
  
- Documentation indexes: `/docs` or `/docs/{folder}`
  - Example: `content/docs/index.md` → `/docs`
  - Example: `content/docs/getting-started/index.md` → `/docs/getting-started`

## Adding New Content

### Add a Blog Post

1. Create a new `.md` file in `content/blog/`
2. Add the required frontmatter (title, description, date, author, tags)
3. Write your content
4. The post will be available at `/blog/{filename-without-md}`

### Add a Documentation Page

1. Create or choose a folder in `content/docs/`
2. Create a new `.md` file in that folder
3. Add the required frontmatter (title, description)
4. Write your content
5. The page will be available at `/docs/{folder}/{filename-without-md}`

### Add a Documentation Index Page

You can create index pages for documentation sections:

1. **Root index**: Create `content/docs/index.md` → Available at `/docs`
2. **Folder index**: Create `content/docs/{folder}/index.md` → Available at `/docs/{folder}`

These work great for overview/landing pages for documentation sections.

## Security

The markdown parser includes path traversal protection. Any path containing `..` or starting with `/` will be rejected with a 400 Bad Request error.

## Styling

Content pages are styled with `/static/css/content.css`, which provides:

- Responsive typography
- Code syntax highlighting
- Table styling
- Blockquote styling
- Image handling
- And more...

The styles are optimized for readability and work well on all screen sizes.

