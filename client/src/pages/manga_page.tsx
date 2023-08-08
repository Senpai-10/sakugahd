import { useState, useEffect, useRef, useMemo } from 'react';
import axios from 'axios';
import '/public/css/pages/manga.css';
import { MangaType } from '../types';
import { Manga } from '../components/manga';
import { Link } from 'react-router-dom';

export function Manga_page() {
    const [manga, setManga] = useState<MangaType[]>([]);
    const [searchQuery, setSearchQuery] = useState('');
    const inputRef = useRef(null);

    const filteredMangaList = useMemo(() => {
        return manga.filter((a) => {
            return (
                a.title.toLowerCase().includes(searchQuery) ||
                a.description.toLowerCase().includes(searchQuery)
            );
        });
    }, [manga, searchQuery]);

    useEffect(() => {
        axios
            .get('/api/manga')
            .then((res) => res.data)
            .then((data) => setManga(data));
    }, []);

    return (
        <>
            <div className='topnav'>
                <input
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    ref={inputRef}
                    type='text'
                    placeholder='Search'
                    className='manga-searchbar'
                />
            </div>
            <div className='manga-list'>
                {filteredMangaList.map((manga_) => (
                    <Link to={`/manga/${manga_.title}`}>
                        <Manga
                            key={manga_.title}
                            title={manga_.title}
                            cover={manga_.cover}
                        />
                    </Link>
                ))}
            </div>
        </>
    );
}
