import { useEffect, useState } from "react";
import fetchCrawledSites from "../api/fetchCrawledSites";
import { useSearchParams } from "react-router-dom";

const CrawledSitesList = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [isLoading, setIsLoading] = useState(true);
  const [sites, setSites] = useState<string[]>([]);

  useEffect(() => {
    const getCrawledSites = async () => {
      const domain = searchParams.get("domain");
      const limit = searchParams.get("limit");
      const offset = searchParams.get("offset");
      const fetchedSites = await fetchCrawledSites(
        domain || undefined,
        limit ? Number(limit) : undefined,
        offset ? Number(offset) : undefined,
      );
      setSites(fetchedSites);
      setIsLoading(false);
    };
    getCrawledSites();
  }, [searchParams]);
  return (
    <>
      <h1>Crawled Sites</h1>
      {isLoading && <p>Loading...</p>}
      {!isLoading && sites.length > 0 && (
        <ul>
          {sites.map((site) => (
            <li>{site}</li>
          ))}

        </ul>
      )}
      {!isLoading && sites.length === 0 && <p>No sites found</p>}
      {!isLoading && (
        <>
          {searchParams.get("offset") && (
            <button onClick={() => {
              setSearchParams((prevParams) => {
                const offset = Number(prevParams.get("offset"));
                const limit = prevParams.get("limit") ? Number(prevParams.get("limit")) : 10;
                const newOffset = offset - limit < 0 ? 0 : offset - limit;

                prevParams.set("offset", newOffset.toString());
                return prevParams;
              })
            }}>
              Prev
            </button>
          )}
          <button onClick={() => {
            setSearchParams((prevParams) => {
              const offset = prevParams.get("offset") ? Number(prevParams.get("offset")) : 0;
              const limit = prevParams.get("limit") ? Number(prevParams.get("limit")) : 10;

              prevParams.set("offset", (offset + limit).toString());
              return prevParams;
            })
          }}>
            Next
          </button>
        </>
      )}
    </>
  )
};

export default CrawledSitesList;