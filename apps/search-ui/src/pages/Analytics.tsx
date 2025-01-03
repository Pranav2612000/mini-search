import { useEffect, useState } from "react";
import fetchAnalytics from "../api/fetchAnalytics";
import { AnalyticsEntry } from "../types/AnalyticsEntry";

const Analytics = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [analyticsData, setAnalyticsData] = useState<AnalyticsEntry[]>([]);

  useEffect(() => {
    const getAnalyticsData = async () => {
      const result = await fetchAnalytics();
      setAnalyticsData(result);
      setIsLoading(false);
    }

    getAnalyticsData();
  }, []);
  return (
    <>
      <h1>Analytics</h1>
      <a href="/">Back to search</a>
      {isLoading && <p>Loading...</p>}
      {!isLoading && analyticsData.length > 0 && (
        <table>
          <tr>
            <th className="domain-cell">Domain</th>
            <th>No of pages scraped</th>
          </tr>
          {analyticsData.map((entry) => (
            <>
              <tr>
                <td className="domain-cell">
                  <a href={`/analytics/crawled_sites?domain=${entry.domain}`}>
                    {entry.domain}
                  </a>
                </td>
                <td>{entry.count}</td>
              </tr>
            </>
          ))}
        </table>
      )}
    </>
  )
};

export default Analytics;