import { useEffect, useState } from "react";
import fetchCrawledSites from "../api/fetchCrawledSites";
import { useSearchParams } from "react-router-dom";

const DEFAULT_LIMIT = 10;
const DEFAULT_OFFSET = 0;

const CrawledSitesList = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [isLoading, setIsLoading] = useState(true);
  const [sites, setSites] = useState<string[]>([]);
  const [totalCount, setTotalCount] = useState(0);

  const domain = searchParams.get("domain");
  const limit = searchParams.get("limit");
  const offset = searchParams.get("offset");

  useEffect(() => {
    const getCrawledSites = async () => {
      setIsLoading(true);
      const result = await fetchCrawledSites(
        domain || undefined,
        limit ? Number(limit) : undefined,
        offset ? Number(offset) : undefined,
      );
      setSites(result.urls);
      setTotalCount(result.total);
      setIsLoading(false);
    };
    getCrawledSites();
  }, [domain, limit, offset]);
  return (
    <>
      <h1>Crawled Sites</h1>
      {(
        <div className="pagination-btn-container">
          {offset && offset != "0" && (
            <button className="prev-btn" onClick={() => {
              setSearchParams((prevParams) => {
                const offset = Number(prevParams.get("offset"));
                const limit = prevParams.get("limit") ? Number(prevParams.get("limit")) : DEFAULT_LIMIT;
                const newOffset = offset - limit < 0 ? 0 : offset - limit;

                prevParams.set("offset", newOffset.toString());
                return prevParams;
              })
            }}>
              Prev
            </button>
          )}
          {((Number(offset || 0) + Number(limit || DEFAULT_LIMIT)) < totalCount) && (
            <button className="next-btn" onClick={() => {
              setSearchParams((prevParams) => {
                const offset = prevParams.get("offset") ? Number(prevParams.get("offset")) : DEFAULT_OFFSET;
                const limit = prevParams.get("limit") ? Number(prevParams.get("limit")) : DEFAULT_LIMIT;

                prevParams.set("offset", (offset + limit).toString());
                return prevParams;
              })
            }}>
              Next
            </button>
          )}
        </div>
      )}
      {isLoading && <p>Loading...</p>}
      {!isLoading && sites.length > 0 && (
        <ul className="crawled-sites-list-container">
          {sites.map((site) => (
            <li>
              <a href={site} target="_blank">
                {site}
              </a>
            </li>
          ))}

        </ul>
      )}
      {!isLoading && sites.length > 0 &&
        <p className="crawled-sites-list-container">
          Showing{' '} 
            {offset}-{Number(offset || DEFAULT_OFFSET) + Number(limit || DEFAULT_LIMIT)}
          {' '}of {totalCount} results
        </p>
      }
      {!isLoading && sites.length === 0 && <p>No sites found</p>}
    </>
  )
};

export default CrawledSitesList;