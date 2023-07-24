import './App.css';
import { Route, Routes } from 'react-router-dom';
import { Home_page } from './pages/home_page';
import { Anime_page } from './pages/anime_page';
import { Watch_page } from './pages/watch_page';

function App() {
    return (
        <Routes>
            <Route path='/' element={<Home_page />} />
            <Route path='/anime/:title' element={<Anime_page />} />
            <Route
                path='/anime/:title/watch/:type/:number'
                element={<Watch_page />}
            />
        </Routes>
    );
}

export default App;
