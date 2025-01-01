import React from "react";
import { MouseEvent } from "react";
import { SearchResult } from "../types/Search";
import fetchSearchResults from "../api/fetchSearchResults";

const SearchPage = () => {
  const [searchText, setSearchText] = React.useState('');
  const [isLoading, setIsLoading] = React.useState(false);
  const [searchResults, setSearchResults] = React.useState<SearchResult[]>([]);
  const [searchDuration, setSearchDuration] = React.useState(0);
  const [isSearchCompleted, setIsSearchCompleted] = React.useState(false);

  const search = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    setIsLoading(true);
    setIsSearchCompleted(true);
    const {results, took_ms: duration} = await fetchSearchResults(searchText);
    setIsLoading(false);
    setSearchResults(results);
    setSearchDuration(duration);
  };

  return (
    <div className={!isSearchCompleted ? "search-centered" : ""}>
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
          <span>Took {searchDuration}ms</span>
          {searchResults.length === 0 && <p>No results found</p>}
          {searchResults.length > 0 && (
            <ul>
              {searchResults.map((result, index) => (
                <li key={index}>{result.title}</li>
              ))}
            </ul>
          )}
        </div>
      )}
      <div className="view-analytics-container">
        <a href='/analytics'>View Analytics</a>
      </div>
    </div>
  )
};

export default SearchPage;