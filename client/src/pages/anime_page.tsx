import { useState, useEffect, useRef, useMemo } from 'react';
import axios from 'axios';
import Anime from '../components/anime';
import '../css/components/animeList.css';
import '../css/components/navbar.css';
import { AnimeType } from '../types';
import { Link } from 'react-router-dom';

export function Anime_page() {
    const [anime, setAnime] = useState<AnimeType[]>([]);
    const [searchQuery, setSearchQuery] = useState('');
    const inputRef = useRef(null);

    const filteredAnimeList = useMemo(() => {
        return anime.filter((a) => {
            return (
                a.title.toLowerCase().includes(searchQuery) ||
                a.description.toLowerCase().includes(searchQuery)
            );
        });
    }, [anime, searchQuery]);

    useEffect(() => {
        axios
            .get('/api/anime')
            .then((res) => res.data)
            .then((data) => setAnime(data));
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
                    className='anime-searchbar'
                />
            </div>
            <div className='anime-list'>
                {filteredAnimeList.map((anime_) => (
                    <Link to={`/anime/${anime_.title}`}>
                        <Anime
                            key={anime_.title}
                            title={anime_.title}
                            cover={anime_.cover}
                        />
                    </Link>
                ))}
            </div>
        </>
    );
}
