import './App.css'
import {
  BrowserRouter as Router,
  Routes,
  Route,
} from "react-router-dom";
import SearchPage from './pages/Search';

function App() {
  return (
    <>
      <Router>
        <Routes>
          <Route path='/' element={<SearchPage />}/>
        </Routes>
      </Router>
    </>
  )
}

export default App
