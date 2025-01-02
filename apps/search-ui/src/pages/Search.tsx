import React, { useEffect } from "react";
import { MouseEvent } from "react";
import { SearchResult } from "../types/Search";
import fetchSearchResults from "../api/fetchSearchResults";
import { useSearchParams } from "react-router-dom";

const SearchPage = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [searchText, setSearchText] = React.useState('');
  const [isLoading, setIsLoading] = React.useState(false);
  const [searchResults, setSearchResults] = React.useState<SearchResult[]>([]);
  const [searchDuration, setSearchDuration] = React.useState(0);
  const searchQuery = searchParams.get('q') || "";
  const isSearchCompleted = Boolean(searchQuery)

  const search = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    setSearchParams({ q: searchText });
  };

  useEffect(() => {
    if (!searchQuery) {
      return;
    }

    const populateSearchResults = async () => {
      setIsLoading(true);
      const {results, took_ms: duration} = await fetchSearchResults(searchQuery);
      setIsLoading(false);
      setSearchResults(results);
      setSearchDuration(duration);
    };
    populateSearchResults();
  }, [searchQuery]);

  return (
    <div className={`search-page ${!isSearchCompleted ? "search-centered" : ""}`}>
      <h1>Mini Search</h1>

      <form>
        <div className={"search-container"}>
          <input
            className="search-input"
            type="text"
            placeholder="Search..."
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
          />
          <button onClick={search}>Search</button>
        </div>
      </form>
      {isLoading && <p>Loading...</p>}
      {isSearchCompleted && !isLoading && (
        <div className="search-results">
          {searchResults.length === 0 && <p>No results found</p>}
          {searchResults.length > 0 && (
            <ul>
              {searchResults.map((result, index) => (
                <li key={index} className="search-result-entry">
                  <div>
                    <a href={result.url}>
                      {result.title}
                    </a>
                  </div>
                </li>
              ))}
            </ul>
          )}
          <span>Took {searchDuration}ms</span>
        </div>
      )}
      <div className="view-analytics-container">
        <a href='/analytics'>View Analytics</a>
      </div>
    </div>
  )
};

export default SearchPage;