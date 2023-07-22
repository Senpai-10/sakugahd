import './App.css';
import { Route, Routes } from 'react-router-dom';
import { Home_page } from './pages/home_page';
import { Show_page } from './pages/show_page';

function App() {
    return (
        <Routes>
            <Route path='/' element={<Home_page />} />
            <Route path='/shows/:title' element={<Show_page />} />
        </Routes>
    );
}

export default App;
