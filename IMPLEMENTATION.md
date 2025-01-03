# Implementation
This document contains some implementation details, design decisions and approaches for next steps

## Components

### Crawler
Looked into various crawlers in various languages for e.g. 
- [scrapy](https://scrapy.org/)
- [spider](https://github.com/spider-rs/spider) [ A very close second ]
- [colly](https://github.com/gocolly/colly)

But finally decided to use [voyager](https://github.com/mattsse/voyager) because
- Rust based ( I wanted to keep the whole project in mono linguistic )
- Easily programmable stateful crawlers ( Although I'm not using this feature, it's a good to have to make it extensible )
- Support for customizing the crawl function
- Small and easy to understand codebase, making it easy to modify it according to use ( I also gave building my own crawler a thought - and this seemed like a good compromise )

### Index
Decided to use [tantivy](https://github.com/quickwit-oss/tantivy) because
- Found benchmarks where tantivy was faster for text data
- low memory footprint
- easy integration with rust and lots of resources available

## Challenges

### Prevent same site from being indexed multiple times
- Solved this by using `url` as an index to ensure each page is indexed only once
- Also ensured that the same url with `query` and `hash` params is treated as the same page to prevent unnecessary re-indexing

### Problem: Pages are continuously updating, how do we rescrape data when a site updates
Solution: Use `scraped_at` to track when a site was last indexed. Re-index the sites when this duration is older than a day ( configurable )

### Problem: Search latency increases and is high ( >50ms ) for large indexes
Solution: < WIP >. Not able to find a solution yet

### Problem: How do we deploy the large index ( may be in a distributed fashion )
Solution: < WIP >. Not able to find a solution, especially one which works for a free tier deployment.

### Problem: Multiple pages may have similar content, but a page is the main page and should be first in rankings
Solution: Created a custom field `headings` to store all data in `h1, h2, h3` tags and used this in search. This improved the results by ensuring that pages which have the matching text in heading are prioritized.

### Problem: Rankings not accurate. Equal relevance given to content, title and headings
Solution: Made use of `set_field_boost` to give more priority to `title` and `headings` for better result

### Problem: User text is not always an exact keyword match ( + can contain typos )
Solution: Made use of Skim Fuzzy matcher for fuzzy match. Using the indexes returned by it for snippet generation

### Problem: Results are not developer documenation oriented
Solution: < WIP >. The current crawling, extracing and searching logic is generic search based. I'll need to add some custom logic to make it easy to search developer documentation

## Questions
### How were rankings optimized to achieve high relevancy in the search results
- Added custom fields for better search results e.g `headings`
- Added boost for fields - `title` - 3x, `heading` 2x over generic search
- Custom snippet generation with fuzzy matching

### How would this work with proxy use?
#### Approach 1:
- Update the `voyager` code base to add first class support for proxies – This is easy as the codebase is tiny and easy to understand.

#### Approach 2:
- Modify the `crawl` function to make the calls through the proxy.
- For reference -  https://github.com/mattsse/voyager?tab=readme-ov-file#inject-async-calls