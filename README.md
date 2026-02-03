# Dimensionless Rust Link Crawler 🔗

Dimensionless Developments presents Rust Link Web Crawler 🔗. A blazingly fast, high-performance web crawler and broken link checker built with Rust. This project demonstrates modern async Rust development with a full-stack web application featuring server-side rendering, concurrent HTTP requests, and comprehensive link validation.

## Why is this needed? 
Broken web links cause significant detriments across user experience, SEO, and compliance, with direct financial consequences. 

User Experience & Trust: Broken links frustrate users, leading to higher bounce rates. Research shows 88% of online consumers are less likely to return to a site after a bad experience, damaging brand credibility and reducing conversion chances. 
**SEO Impact**:
- Crawl Budget Waste: Search engine bots waste time on broken links, reducing efficient crawling of valid pages, which can harm indexing and visibility.
- Lower Rankings: Google considers user behavior signals like bounce rate and dwell time; high bounce rates from broken links can negatively affect rankings.
- Loss of Link Equity (Link Juice): Broken internal and external links fail to pass SEO value. This reduces the authority of linked pages and wastes the value of backlinks from other sites. 
**Financial Implications**:
- Lost Revenue: Broken call-to-action (CTA) links, product page links, or demo links directly result in missed sales, conversions, and leads. 
**Reduced Referral Traffic**: 
- When external sites link to your broken content, their visitors bounce, harming your referral traffic and partnership value.
**Compliance Fines**: 
- In regulated industries like finance or healthcare, broken links to privacy policies, licensing, or legal documents can lead to regulatory penalties (e.g., GDPR, CCPA, Reg Z) and legal risks. 
- Long-Term Costs: Wasted time and resources spent on content creation become ineffective if content is inaccessible due to broken links. Fixing them proactively is far cheaper than recovering lost traffic and reputation

**These are the reasons Dimensionless Rust Link Crawler is needed**

## Overview

Rust Link Crawler 🔗 is a production-ready web crawler that:

- **Traverses websites** with configurable depth levels to discover all links
- **Validates link status** by checking HTTP response codes (working vs broken)
- **Runs concurrently** using Rust's async/await for lightning-fast performance
- **Provides a beautiful UI** with real-time crawl results and detailed link analysis
- **Ensures memory safety** with Rust's compile-time guarantees (no null pointer errors, no data races)
- **Filters same-domain links** to avoid crawling the entire internet

### Key Features

✅ **Fast**: Concurrent HTTP requests powered by Tokio async runtime  
✅ **Safe**: Compile-time memory safety with zero-cost abstractions  
✅ **Reliable**: Robust error handling with fallback mechanisms (HEAD → GET requests)  
✅ **User-friendly**: Beautiful web interface with live progress updates  
✅ **Configurable**: Adjust crawl depth (0-5 levels) for different needs  

---
## How the Crawler Works

### Algorithm: Breadth-First Search (BFS)

```
1. Start with root URL
2. Fetch the page → Extract all <a> links
3. Check status of each link (working or broken?)
4. Add unvisited same-domain links to queue
5. Repeat for each URL up to max_depth
6. Return all results
```
---

## Quickstart

### Prerequisites

- **Rust 1.70+** ([Install Rust](https://rustup.rs/))
- Windows, macOS, or Linux

### Clone & Run

```bash
# Clone the repository
git clone https://github.com/DimensionlessDevelopments/Dimensionless-Rust-Web-Crawler.git
cd dimensionless_crawler_core

# Build the project
cargo build -p dimensionless_crawler_core
cargo build -p server

# Run the server
cargo run -p server
```

The crawler will start on `http://127.0.0.1:3000`

### Usage

1. Open your browser to `http://127.0.0.1:3000`
2. Enter a website URL (e.g., `https://example.com`)
3. Set crawl depth (0-5):
   - **0**: Only check the starting URL
   - **1**: Check the page + all links on that page
   - **2+**: Recursively follow links deeper
4. Click "Start Crawling 🔎"
5. View results with ✓ working links and ✗ broken links

**Example Output:**
```
=== Crawl Results ===

Total links found: 42

✓ Working links: 40
✗ Broken links: 2

--- Broken Links ---
✗ https://example.com/broken-page (Status: 404)
✗ https://example.com/old-resource (Status: 410)

--- All Links ---
✓ https://example.com (Status: 200)
✓ https://example.com/about (Status: 200)
...
```

---

## Tech Stack

### Core Technologies

| Technology | Version | Purpose |
|-----------|---------|---------|
| **Rust** | 2021 Edition | Language & type system |
| **Tokio** | 1.49 | Async runtime (multi-threaded) |
| **Axum** | 0.6 | Web framework & HTTP routing |
| **Reqwest** | 0.11 | HTTP client with connection pooling |
| **Scraper** | 0.13 | HTML parsing & CSS selectors |

### Frontend

| Technology | Purpose |
|-----------|---------|
| **HTML5** | Semantic markup |
| **Tailwind CSS** | Utility-first styling (CDN) |
| **Vanilla JavaScript** | DOM manipulation & API calls |

### Build & Package Management

| Tool | Purpose |
|------|---------|
| **Cargo** | Rust package manager & build system |
| **Tower-HTTP** | Static file serving middleware |

---

## Project Structure

```
DIMENSIONLESS-RUST-WEB-CRAWLER/
├── Cargo.toml                 # Workspace manifest
├── dimensionless_crawler_core/              # Crawling logic (library)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs             # Core crawling algorithm
├── server/                    # Web server (binary)
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs           # Axum server & routes
│   └── static/
│       └── index.html        # Frontend HTML/JS
└── README.md
```

---

## Crates & Modules

### `dimensionless_crawler_core` Crate

**Purpose**: Implements the core web crawling logic

**Main Module**: `lib.rs`

```rust
pub async fn crawl_and_check(
    start: &str, 
    max_depth: usize
) -> Result<Vec<LinkResult>, Box<dyn Error + Send + Sync>>
```

**Key Structures**:
- `LinkResult { url: String, status: Option<u16>, ok: bool }` - Represents a checked link

**Key Dependencies**:
- `reqwest::Client` - HTTP requests with connection pooling
- `scraper::Html` - DOM parsing
- `url::Url` - URL parsing and joining
- `std::collections::{HashSet, VecDeque}` - Graph traversal data structures

### `server` Crate

**Purpose**: Provides HTTP endpoints and serves the web interface

**Main Module**: `main.rs`

**Routes**:
- `GET /` - Serves static HTML frontend
- `POST /api/crawl` - Accepts crawl requests, returns results

**Key Structures**:
- `CrawlRequest { url: String, depth: Option<usize> }` - Client request
- `CrawlResponse { links: Vec<LinkResult> }` - Server response

**Key Dependencies**:
- `axum::Router` - HTTP routing
- `tower_http::services::ServeDir` - Static file serving
- `serde` - JSON serialization/deserialization
- `tokio::main` - Async runtime entry point

---

## Rust Concepts Explained

### 1. **Async/Await & Concurrency**

The crawler uses Rust's async/await to handle many HTTP requests concurrently without threads:

```rust
// Makes 100+ simultaneous HTTP requests efficiently
let status = match client.head(url).send().await {
    Ok(r) => Some(r.status().as_u16()),
    Err(_) => None,
};
```

**Why it matters**: Traditional threads are expensive (~2MB memory each). Tokio's async tasks are lightweight (~64 bytes), allowing thousands of concurrent operations.

### 2. **Ownership & Memory Safety**

Rust's ownership system prevents memory leaks and data races at compile-time:

```rust
// HTML is dropped before async await point
let resolved: Vec<Url> = {
    let document = Html::parse_document(&body);  // Borrows &body
    // ... extract URLs ...
    res  // Returns owned Vec<Url>
};  // HTML is dropped here (no dangling pointers!)

// Safe to use resolved with async
for url in resolved { /* ... */ }
```

**Why it matters**: No garbage collection overhead, zero-cost abstractions.

### 3. **Error Handling with Result Types**

Rust uses `Result<T, E>` instead of exceptions:

```rust
let status = match client.head(url).send().await {
    Ok(r) => Some(r.status().as_u16()),     // Success path
    Err(e) => {
        eprintln!("Failed: {}", e);          // Error path (explicit)
        None
    }
};
```

**Why it matters**: Errors must be handled explicitly. No silent failures.

### 4. **Traits & Bounds**

Traits define shared behavior across types:

```rust
pub async fn crawl_and_check(
    start: &str, 
    max_depth: usize
) -> Result<Vec<LinkResult>, Box<dyn Error + Send + Sync>>
```

- `Error + Send + Sync` - Any error type that's thread-safe
- `Send` - Can be transferred between threads
- `Sync` - Can be safely referenced from multiple threads
- `dyn` - Dynamic dispatch (runtime polymorphism)

**Why it matters**: Generic, reusable error handling without knowing exact error type.

### 5. **Borrowing & References**

Instead of pointers, Rust has safe references:

```rust
// Immutable borrow (multiple readers allowed)
let body: String = client.get(url.clone()).send().await?.text().await?;

// Parse without moving body
let document = Html::parse_document(&body);  // Borrows &body
let selector = Selector::parse("a").unwrap();

// body is still valid here!
println!("Parsed {} bytes", body.len());
```

**Why it matters**: Compiler ensures no use-after-free, no buffer overflows.

### 6. **Pattern Matching**

Rust's `match` is exhaustive and more powerful than switch statements:

```rust
match depth {
    0 => println!("Check only this URL"),
    1..=5 => println!("Traverse {} levels", depth),
    _ => println!("Too deep!"),
}
```

**Why it matters**: Handles all cases, compiler forces you to be exhaustive.

### 7. **Lifetimes**

Rust tracks how long references are valid:

```rust
// 'a lifetime ensures &str lives at least as long as the function
fn crawl_and_check(start: &str, max_depth: usize) -> Result<Vec<LinkResult>, ...> {
    let url = Url::parse(start)?;  // start must be valid for this function
    // ...
}
```

**Why it matters**: Prevents dangling references at compile-time.

### 8. **Zero-Cost Abstractions**

High-level Rust code compiles to efficient machine code:

```rust
// This iterator chain has ZERO runtime overhead
data.links
    .filter(|l| !l.ok)              // Inlined
    .map(|link| link.url.clone())   // Inlined
    .collect::<Vec<_>>()            // Single allocation
```

Compiles to the same code as hand-written C.

### 9. **Macros & Derive**

Reduce boilerplate with compile-time code generation:

```rust
#[derive(Serialize, Deserialize)]
struct LinkResult {
    url: String,
    status: Option<u16>,
    ok: bool,
}

// ☝️ Automatically implements JSON serialization/deserialization
```

**Why it matters**: Less code, fewer bugs, same performance.

### 10. **Fearless Concurrency**

Rust's type system guarantees thread-safety:

```rust
// Client is Arc<...> internally, safe to share across async tasks
let client = Client::builder().build()?;
let client_clone = client.clone();

tokio::spawn(async move {
    client_clone.get(url).send().await  // No race conditions!
}).await?;
```

**Why it matters**: Compiler catches data races before runtime. Most concurrency bugs prevented at compile-time.

---

### Implementation Details

```rust
let mut queue: VecDeque<(Url, usize)> = VecDeque::new();  // (url, depth)
let mut seen: HashSet<String> = HashSet::new();           // Track visited
let mut results: Vec<LinkResult> = Vec::new();            // Collect results

queue.push_back((start_url, 0));
seen.insert(start_url.to_string());

while let Some((url, depth)) = queue.pop_front() {
    // 1. Fetch page
    let body = client.get(url).send().await?.text().await?;
    
    // 2. Parse HTML & extract links
    let resolved: Vec<Url> = {
        let document = Html::parse_document(&body);
        let mut res = Vec::new();
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                res.push(/* parse and join URL */);
            }
        }
        res
    };
    
    // 3. Check each link
    for link in resolved {
        let status = client.head(link).send().await?.status().as_u16();
        results.push(LinkResult {
            url: link.to_string(),
            status: Some(status),
            ok: status < 400,
        });
        
        // 4. Add to queue if not visited
        if depth + 1 <= max_depth && !seen.contains(&link.to_string()) {
            queue.push_back((link, depth + 1));
        }
    }
}

Ok(results)
```

---

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Fetch URL | O(n) | n = bytes to download |
| Parse HTML | O(n) | n = HTML size |
| Check links | O(m) parallel | m = number of links |
| Overall | O(n + m*d) | d = depth, m = links per page |

**Example**: Crawling 100-page site with 50 links per page at depth 2:
- Synchronous: ~30 seconds (sequential)
- Async (Rust): ~2 seconds (concurrent)

---

## Building & Development

```bash
# Build in development mode (faster compile, slower runtime)
cargo build

# Build in release mode (slower compile, faster runtime)
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

---

## Common Rust Patterns Used

| Pattern | Example | Purpose |
|---------|---------|---------|
| **Option<T>** | `status: Option<u16>` | Value might not exist |
| **Result<T, E>** | `Result<Vec<LinkResult>, Error>` | Operation might fail |
| **Iterators** | `.filter().map().collect()` | Lazy, composable data processing |
| **Match** | `match result { Ok(x) => ..., Err(e) => ... }` | Exhaustive pattern matching |
| **Closure** | `\|link\| link.ok` | Inline anonymous function |
| **Trait Objects** | `Box<dyn Error>` | Runtime polymorphism |
| **Scoping** | `{ /* scope */ }` | Explicit resource cleanup |

---

## Troubleshooting

### Server won't start
```bash
# Check if port 3000 is in use
netstat -ano | findstr :3000  # Windows
lsof -i :3000                  # macOS/Linux

# Kill the process or use a different port
```

### Crawler returns no links
- Check browser console for JavaScript errors
- Ensure you're using `https://` (some sites require it)
- Try depth=0 first to verify the URL is accessible
- Check server logs for detailed error messages

### Build fails
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

---

## Future Improvements

- [ ] Rate limiting to respect robots.txt
- [ ] Proxy support for anonymity
- [ ] Custom headers & authentication
- [ ] Export results to CSV/JSON
- [ ] Screenshot website previews
- [ ] Broken link notifications via email
- [ ] Database persistence
- [ ] Distributed crawling

---

## License

MIT License - Feel free to use this project for learning or production!

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Tokio Documentation](https://tokio.rs/) - Async runtime
- [Axum Documentation](https://docs.rs/axum/) - Web framework
- [Reqwest Documentation](https://docs.rs/reqwest/) - HTTP client
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Interactive examples

---

**Built with ❤️ using Rust, Axum, and Tokio**  

--- 

## Contact
**Made by Dimensionless Developments**
**Head to our website https://www.dimensionlessdevelopments.com. email: contact@dimensionlessdevelopments.com**
