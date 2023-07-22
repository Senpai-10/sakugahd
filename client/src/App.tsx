import './App.css';
import { Route, Routes } from 'react-router-dom';
import { Home_page } from './pages/home_page';
import { Show_page } from './pages/show_page';
import { Watch_page } from './pages/watch_page';

function App() {
    return (
        <Routes>
            <Route path='/' element={<Home_page />} />
            <Route path='/shows/:title' element={<Show_page />} />
            <Route path='/shows/:title/watch' element={<Watch_page />} />
        </Routes>
    );
}

export default App;
