# Mini Search
A simple POC search engine to learn about Search Technologies.
https://mini-search-d9465.web.app/

## Description
A mini search engine specifically designed for searching programming documentation using existing technologies. Uses
- [tantivy](https://github.com/quickwit-oss/tantivy) for search,
- [voyager](https://github.com/mattsse/voyager) for crawling websites
- [axum](https://github.com/tokio-rs/axum) for APIs
- [vite](https://vite.dev/) and [React](https://react.dev) for UI
- [shuttle](https://shuttle.dev) for API deployment
- [firebase](https://firebase.google.com/) for UI deployment

## Getting Started

### Dependencies

* Rust 1.81
* Node 18 ( npm 9 )

### Structure
The codebase is set up as a mono repo with the following structure
- crates ( Modular building blocks which can be substituted easily )
	- crawler - The crawler component which visits websites
	- searcher - The component responsible for searching and returning results
	- config - Common config used throughout the project
- apps 
	- api - APIs built using Axum to serve search, analytics and crawling triggers
	- search-ui - React App for Search and analytics page
- index ( Search index generated by crawler & used by searcher which stores all sitedata )

### Setup - UI
- Navigate to the `search-ui` directory
`cd apps/search-ui`
- Install dependencies
`npm install`
- Run the app
`npm run dev`
_You may need to update the baseURL in `App.tsx` to point to your backend_

### Setup - Core
- Install dependencies
`cargo build`
- Install shuttle ( https://docs.shuttle.rs/getting-started/installation )
- Run the app
`shuttle run`

*Note: To test out an independent crates you can also navigate to the crate and run the corresponding examples*