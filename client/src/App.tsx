import './App.css';
import { Route, Routes } from 'react-router-dom';
import { Home_page } from './pages/home_page';
import { AnimeView_page } from './pages/anime_view_page';
import { Watch_page } from './pages/watch_page';
import { Studio_page } from './pages/studio_page';
import { Studios_page } from './pages/studios_page';
import { Genres_page } from './pages/genres_page';
import { Genre_page } from './pages/genre_page';
import { Manga_page } from './pages/manga_page';
import { MangaView_page } from './pages/manga_view_page';
import { MangaRead_page } from './pages/manga_read_page';
import { Anime_page } from './pages/anime_page';

function App() {
    return (
        <Routes>
            <Route path='/' element={<Home_page />} />
            <Route path='/studios' element={<Studios_page />} />
            <Route path='/studios/:name' element={<Studio_page />} />
            <Route path='/genres' element={<Genres_page />} />
            <Route path='/genres/:name' element={<Genre_page />} />
            <Route path='/anime' element={<Anime_page />} />
            <Route path='/anime/:title' element={<AnimeView_page />} />
            <Route
                path='/anime/:title/watch/:type/:number'
                element={<Watch_page />}
            />
            <Route path='/manga' element={<Manga_page />} />
            <Route path='/manga/:title' element={<MangaView_page />} />
            <Route path='/manga/:title/read/:id' element={<MangaRead_page />} />
        </Routes>
    );
}

export default App;
