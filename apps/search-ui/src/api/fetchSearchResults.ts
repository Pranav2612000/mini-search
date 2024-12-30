import { SearchResult } from "../types/Search";

const fetchSearchResults = async (query: string): Promise<{ results: SearchResult[], took_ms: number}> => {
  const response = await fetch(`http://localhost:3000/api/search?q=${query}`);
  const data = await response.json();
  return data;
}

export default fetchSearchResults;