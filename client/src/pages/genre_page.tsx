import { useParams } from 'react-router-dom';
import { useState, useEffect } from 'react';
import { AnimeType } from '../types';
import Anime from '../components/anime';
import axios from 'axios';
import '/public/css/components/animeList.css';

export function Genre_page() {
    const { name } = useParams();

    if (name == undefined) {
        return <>nothing found</>;
    }

    const encoded_genre_name = encodeURI(name);
    const [anime, setAnime] = useState<AnimeType[]>([]);

    useEffect(() => {
        axios
            .get(`/api/genres/${encoded_genre_name}`)
            .then((res) => res.data)
            .then((data) => setAnime(data));
    }, []);

    return (
        <>
            <div className='anime-list'>
                {anime.map((item) => (
                    <Anime
                        title={item.title}
                        cover={item.cover}
                        key={item.title}
                    />
                ))}
            </div>
        </>
    );
}
