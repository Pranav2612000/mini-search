import axios from "axios";

const fetchCrawledSites = async (domain?: string, limit?: number, offset?: number): Promise<{ urls: string[], total: number }> => {
  const response = await axios.get('api/crawled_urls', {
    params: {
      ...(domain && { domain }),
      ...(limit && { limit }),
      ...(offset && { offset }),
    }
  });
  const data = await response.data;
  return data;
}

export default fetchCrawledSites;