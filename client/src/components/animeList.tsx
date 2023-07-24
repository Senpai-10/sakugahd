import { useState, useEffect, useRef, useMemo } from 'react';
import axios from 'axios';
import Anime from './anime';
import '/public/css/components/animeList.css';
import { AnimeType } from '../types';

function AnimeList() {
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
            <input
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                ref={inputRef}
                type='text'
                placeholder='Search'
            />
            <div className='anime-list'>
                {filteredAnimeList.map((anime_) => (
                    <Anime
                        key={anime_.title}
                        title={anime_.title}
                        cover={anime_.cover}
                    />
                ))}
            </div>
        </>
    );
}

export default AnimeList;
