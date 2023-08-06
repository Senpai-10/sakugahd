import { useEffect, useState } from 'react';
import '/public/css/pages/home.css';
import { AnimeType, MangaType } from '../types';
import axios from 'axios';
import Anime from '../components/anime';
import { Manga } from '../components/manga';
import { Link } from 'react-router-dom';

export function Home_page() {
    const [anime, setAnime] = useState<AnimeType[]>()
    const [manga, setManga] = useState<MangaType[]>()

    useEffect(() => {
        axios.get(`/api/anime?limit=5`)
            .then((res) => res.data)
            .then((data) => setAnime(data))

        axios.get(`/api/manga?limit=5`)
            .then((res) => res.data)
            .then((data) => setManga(data))

    }, [])

    if (anime == undefined || manga == undefined) {
        return <h1>Loading..</h1>
    }

    return (
        <>
            <Link to='/anime'>
                <h1 className="section">Anime</h1>
                <div className="horizontal-list">
                    {
                        anime.map((a) => {
                            return (
                                <Anime title={a.title} cover={a.cover} />
                            )
                        })
                    }
                </div>
            </Link>
            <Link to='/manga'>
                <h1 className="section">Manga</h1>
            </Link>
            <div className="horizontal-list">
                {
                    manga.map((m) => {
                        return (
                            <Manga title={m.title} cover={m.cover} />
                        )
                    })
                }
            </div>
        </>
    )
}
