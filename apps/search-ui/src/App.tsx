import './App.css'
import {
  BrowserRouter as Router,
  Routes,
  Route,
} from "react-router-dom";
import SearchPage from './pages/Search';
import axios from 'axios';
import CrawledSitesList from './pages/CrawledSitesList';

axios.defaults.baseURL = 'http://localhost:8000';

function App() {
  return (
    <>
      <Router>
        <Routes>
          <Route path='/' element={<SearchPage />}/>
          <Route path='/analytics/crawled_sites' element={<CrawledSitesList />} />
        </Routes>
      </Router>
    </>
  )
}

export default App
