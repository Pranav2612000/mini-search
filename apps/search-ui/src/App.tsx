import './App.css'
import {
  BrowserRouter as Router,
  Routes,
  Route,
} from "react-router-dom";
import SearchPage from './pages/Search';
import axios from 'axios';
import CrawledSitesList from './pages/CrawledSitesList';
import Analytics from './pages/Analytics';

axios.defaults.baseURL = 'https://mini-search-p4mq.shuttle.app';

function App() {
  return (
    <>
      <Router>
        <Routes>
          <Route path='/' element={<SearchPage />}/>
          <Route path='/analytics' element={<Analytics />} />
          <Route path='/analytics/crawled_sites' element={<CrawledSitesList />} />
        </Routes>
      </Router>
    </>
  )
}

export default App
