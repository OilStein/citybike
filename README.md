
# Helsinki citybike application
Solita dev academy pre-assignment

## Installation
There are installation guides in both frontend and backend directories.

## Done features
### Backend
- Imports data to embedded Surrealdb database
- Validates data when reading CSV
- Pagination in the journey and station queries
- Supports ordering in querying -> Not implemented to frontend
- Single journey and station endpoints
- Calculates the total number of leaving and starting journeys in querying a single station.

### Frontend
- User can surf in home, journey list, station list, and single station pages.
- User can change pages in both views

## Known issues
### Performance
- With a larger dataset journey, queries are very slow. -> Added timeout to query.
  - Cause of the large table
  - Should be fixable if using configuring to TiKV instead of RocksDB.
- Importing stations and three months journey dataset to the database takes ~50min
  - Importing stations and one journey set takes ~10min  

## Post Mortem
Pre-assignment was fun and intriguing to make. I wish that I had more time for working on this project before the deadline.
The real-life application aspect made working more meaningful. 
Naming parameters and functions were painful. Writing tests is one of my weaknesses.

## Used technologies
### Frontend
- [https://nextjs.org/](NextJS) - UI React Framework 
- [https://tailwindcss.com/](TailwindCSS) - Styling
- [https://www.typescriptlang.org/](TypeScript) - Syntax
- [https://ui.shadcn.com/docs](shadcn-ui) - Reusable ui components

### Backend - REST API
- [https://www.rust-lang.org/](Rust) - Core 
- [https://surrealdb.com/docs](SurrealDB) - Database 
- [https://actix.rs/](Actic) - Web Framework

### Testing
- [https://www.cypress.io/](Cypress) - (E2E) - In Progress
- [https://jestjs.io/](Jest) - Unit Testing 
- [https://doc.rust-lang.org/book/ch11-01-writing-tests.html](Rust) - Unit Testing

### Tools
- [https://neovim.io/](Neovim) with [https://nvchad.com/](NvChad) config
- [https://learn.microsoft.com/en-us/windows/wsl/install](WSL2) with Ubuntu distro
- [https://github.com/jesseduffield/lazygit](Lazygit) as git tool.
- [https://code.visualstudio.com/](VS Code) when neovim rust-analyzer stopped working...


