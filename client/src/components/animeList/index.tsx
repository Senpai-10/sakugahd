import { useState, useEffect } from 'react';
import axios from 'axios';
import Anime from '../anime';
import './index.css';
import { AnimeType } from "../../types"

function AnimeList() {
    const [anime, setAnime] = useState<AnimeType[]>([]);

    useEffect(() => {
        axios
            .get('/api/anime')
            .then((res) => res.data)
            .then((data) => setAnime(data));
    }, []);

    return (
        <div className='anime-list'>
            {anime.map((anime_) => (
                <Anime key={anime_.title} title={anime_.title} cover={anime_.cover} />
            ))}
        </div>
    );
}

export default AnimeList;
