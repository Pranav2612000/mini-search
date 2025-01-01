import axios from "axios";
import { AnalyticsEntry } from "../types/AnalyticsEntry";

const fetchAnalytics = async (): Promise<AnalyticsEntry[]> => {
  const response = await axios.get(`api/analytics`);
  const data = await response.data;
  return data;
}

export default fetchAnalytics;