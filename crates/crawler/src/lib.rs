mod doc_extractor;
mod extracted_content;
mod doc_collector;

use std::{sync::{Arc, Mutex}, time::Duration};

use doc_collector::DocCollector;
use tantivy::query::QueryParser;
use voyager::{Collector, CrawlerConfig, RequestDelay};
use futures::StreamExt;

const SITES: [&str; 166] = [
  "angular.io",
  "api.drupal.org",
  "api.haxe.org",
  "api.qunitjs.com",
  "babeljs.io",
  "backbonejs.org",
  "bazel.build",
  "bluebirdjs.com",
  "bower.io",
  "cfdocs.org",
  "clojure.org",
  "clojuredocs.org",
  "codecept.io",
  "codeception.com",
  "codeigniter.com",
  "coffeescript.org",
  "cran.r-project.org",
  "crystal-lang.org",
  "forum.crystal-lang.org",
  "css-tricks.com",
  "dart.dev",
  "dev.mysql.com",
  "developer.apple.com",
  "developer.mozilla.org",
  "developer.wordpress.org",
  "doc.deno.land",
  "doc.rust-lang.org",
  "docs.astro.build",
  "docs.aws.amazon.com",
  "docs.brew.sh",
  "docs.chef.io",
  "docs.cypress.io",
  "docs.influxdata.com",
  "docs.julialang.org",
  "docs.microsoft.com",
  "docs.npmjs.com",
  "docs.oracle.com",
  "docs.phalconphp.com",
  "docs.python.org",
  "docs.rs",
  "docs.ruby-lang.org",
  "docs.saltproject.io",
  "docs.wagtail.org",
  "doctrine-project.org",
  "docwiki.embarcadero.com",
  "eigen.tuxfamily.org",
  "elixir-lang.org",
  "elm-lang.org",
  "en.cppreference.com",
  "enzymejs.github.io",
  "erights.org",
  "erlang.org",
  "esbuild.github.io",
  "eslint.org",
  "expressjs.com",
  "fastapi.tiangolo.com",
  "flow.org",
  "fortran90.org",
  "fsharp.org",
  "getbootstrap.com",
  "getcomposer.org",
  "git-scm.com",
  "gnu.org",
  "gnucobol.sourceforge.io",
  "go.dev",
  "golang.org",
  "graphite.readthedocs.io",
  "groovy-lang.org",
  "gruntjs.com",
  "handlebarsjs.com",
  "haskell.org",
  "hex.pm",
  "hexdocs.pm",
  "httpd.apache.org",
  "i3wm.org",
  "jasmine.github.io",
  "javascript.info",
  "jekyllrb.com",
  "jsdoc.app",
  "julialang.org",
  "knockoutjs.com",
  "kotlinlang.org",
  "laravel.com",
  "latexref.xyz",
  "learn.microsoft.com",
  "lesscss.org",
  "love2d.org",
  "lua.org",
  "man7.org",
  "mariadb.com",
  "mochajs.org",
  "modernizr.com",
  "momentjs.com",
  "mongoosejs.com",
  "next.router.vuejs.org",
  "next.vuex.vuejs.org",
  "nginx.org",
  "nim-lang.org",
  "nixos.org",
  "nodejs.org",
  "npmjs.com",
  "ocaml.org",
  "odin-lang.org",
  "openjdk.java.net",
  "opentsdb.net",
  "perldoc.perl.org",
  "php.net",
  "playwright.dev",
  "pointclouds.org",
  "postgresql.org",
  "prettier.io",
  "pugjs.org",
  "pydata.org",
  "pytorch.org",
  "qt.io",
  "r-project.org",
  "react-bootstrap.github.io",
  "reactivex.io",
  "reactjs.org",
  "reactnative.dev",
  "reactrouterdotcom.fly.dev",
  "readthedocs.io",
  "readthedocs.org",
  "redis.io",
  "redux.js.org",
  "requirejs.org",
  "rethinkdb.com",
  "ruby-doc.org",
  "ruby-lang.org",
  "rust-lang.org",
  "rxjs.dev",
  "sass-lang.com",
  "scala-lang.org",
  "scikit-image.org",
  "scikit-learn.org",
  "spring.io",
  "sqlite.org",
  "stdlib.ponylang.io",
  "superuser.com",
  "svelte.dev",
  "swift.org",
  "tailwindcss.com",
  "twig.symfony.com",
  "typescriptlang.org",
  "underscorejs.org",
  "vitejs.dev",
  "vitest.dev",
  "vuejs.org",
  "vueuse.org",
  "webpack.js.org",
  "wiki.archlinux.org",
  "www.chaijs.com",
  "www.electronjs.org",
  "www.gnu.org",
  "www.hammerspoon.org",
  "www.khronos.org",
  "www.lua.org",
  "www.php.net/manual/en/",
  "www.pygame.org",
  "www.rubydoc.info",
  "www.statsmodels.org",
  "www.tcl.tk",
  "www.terraform.io",
  "www.vagrantup.com",
  "www.yiiframework.com",
  "yarnpkg.com"
];

pub async fn start_indexing() {
    println!("Starting indexing...");
    let mut schema_builder = tantivy::schema::Schema::builder();
    schema_builder.add_text_field("title", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("content", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("url", tantivy::schema::STORED | tantivy::schema::TEXT);
    schema_builder.add_text_field("domain", tantivy::schema::STORED);
    schema_builder.add_text_field("headings", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("code_blocks", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("api_items", tantivy::schema::TEXT | tantivy::schema::STORED);
    let schema = schema_builder.build();

    let index_dir = tantivy::directory::MmapDirectory::open("./index").unwrap();
    let index = if tantivy::Index::exists(&index_dir).unwrap() {
      tantivy::Index::open(index_dir).unwrap()
    } else {
      tantivy::Index::create_in_dir("./index", schema.clone()).unwrap()
    };
    let index_writer = index.writer(50_000_000).unwrap();
    let index_reader = index.reader().unwrap();

    let config = CrawlerConfig::default()
      .allow_domains_with_delay(
        SITES.iter().map(|site| {
          (site.to_string(), RequestDelay::Random { min: Duration::from_millis(500), max: Duration::from_millis(2000) })
        })
      )
      .respect_robots_txt()
      .max_concurrent_requests(10);

    let mut doc_collector = DocCollector {
      index_writer: Arc::new(Mutex::new(index_writer)),
      index_reader: Arc::new(index_reader),
      url_query_parser: Arc::new(
        QueryParser::for_index(&index, vec![schema.get_field("url").unwrap()])
      ),
      schema,
      extractors: Arc::new(dashmap::DashMap::new()),
      counter: Arc::new(dashmap::DashMap::new()),
    };

    let mut collector = Collector::new(doc_collector.clone(), config);

    for curr_chunk_sites in SITES.chunks(1) {
      println!("Indexing sites: {:?}", curr_chunk_sites);
      for site in curr_chunk_sites {
        println!("Site: {}", site);
        collector.crawler_mut().visit_with_state(
          &format!("https://{}", site),
          ()
        );
      }
      println!("Visiting sites...");

      while let Some(output) = collector.next().await {
        if let Ok(post) = output {
          // println!("{:?}", post.headings);
        }
      };

      doc_collector.commit();
      println!("Completed indexing sites: {:?}", curr_chunk_sites);
    }

    println!("Indexing complete!");
}
