import axios from "axios";
import { SearchResult } from "../types/Search";

const fetchSearchResults = async (query: string): Promise<{ results: SearchResult[], took_ms: number}> => {
  const response = await axios.get(`api/search?q=${query}`);
  const data = await response.data;
  return data;
}

export default fetchSearchResults;