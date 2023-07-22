import { useParams } from 'react-router-dom';
import { useState, useEffect } from 'react';
import axios from 'axios';
import './index.css';

type ShowFormat = 'TV' | 'OVA' | 'ONA' | 'MOVIE' | 'SPECIAL';
type ShowSeason = 'SPRING' | 'SUMMER' | 'FALL' | 'WINTER';
type ShowStatus = 'FINISHED' | 'ONGOING';

interface Show {
    title: string;
    description: String;
    format?: ShowFormat;
    status?: ShowStatus;
    season?: ShowSeason;
    season_year?: number;
    cover?: number[];
}

interface Episode {
    id: string;
    show_title: string;
    title: string;
    number: number;
    is_filler: boolean;
    file_name: string;
    thumbnail_file_name: string;
}

interface Movie {
    id: string;
    show_title: string;
    title: string;
    number: number;
    watch_after: number;
    file_name: string;
    thumbnail_file_name: string;
}

interface Opening {
    id: string;
    show_title: string;
    title: string;
    number: number;
    file_name: string;
    thumbnail_file_name: string;
}

interface Ending {
    id: string;
    show_title: string;
    title: string;
    number: number;
    file_name: string;
    thumbnail_file_name: string;
}

interface EpProps {
    ep: Episode;
}

function Episode(epprops: EpProps) {
    const ep = epprops.ep;
    const image = encodeURIComponent(ep.thumbnail_file_name);

    return (
        <>
            <img className='thumbnail' src={`/api/thumbnail/${image}`} />
            <p>title: {ep.title}</p>
            <p>show title: {ep.show_title}</p>
            <p>ep number {ep.number}</p>
            <p>is filler {ep.is_filler}</p>
            <p>file name: {ep.file_name}</p>
        </>
    );
}

type Tabs = 'Episodes' | 'Movies' | 'Openings' | 'Endings';

export function Show_page() {
    const { title } = useParams();

    if (title == undefined) {
        return (
            <>
                <h1>title not found!</h1>
            </>
        );
    }

    const encoded_title = encodeURIComponent(title);
    const [currentTab, setCurrentTab] = useState<Tabs>('Episodes');
    const [show, setShow] = useState<Show>();
    const [episodes, setEpisodes] = useState<Episode[]>([]);
    const [movies, setMovies] = useState<Movie[]>([]);
    const [openings, setOpenings] = useState<Opening[]>([]);
    const [endings, setEndings] = useState<Ending[]>([]);

    useEffect(() => {
        axios
            .get(`/api/shows/${encoded_title}`)
            .then((res) => res.data)
            .then((data) => setShow(data));
        axios
            .get(`/api/shows/${encoded_title}/episodes`)
            .then((res) => res.data)
            .then((data) => setEpisodes(data));
        axios
            .get(`/api/shows/${encoded_title}/movies`)
            .then((res) => res.data)
            .then((data) => setMovies(data));
        axios
            .get(`/api/shows/${encoded_title}/openings`)
            .then((res) => res.data)
            .then((data) => setOpenings(data));
        axios
            .get(`/api/shows/${encoded_title}/endings`)
            .then((res) => res.data)
            .then((data) => setEndings(data));
    }, []);

    return show ? (
        <>
            <button onClick={() => setCurrentTab('Episodes')}>Episodes</button>
            <button onClick={() => setCurrentTab('Movies')}>Movies</button>
            <button onClick={() => setCurrentTab('Openings')}>Openings</button>
            <button onClick={() => setCurrentTab('Endings')}>Endings</button>
            show: {show.title}
            <div
                className={
                    currentTab == 'Episodes' ? 'active-tab' : 'inactive-tab'
                }
            >
                <h1>Episodes</h1>
                {episodes.map((episode) => (
                    <Episode key={episode.id} ep={episode} />
                ))}
            </div>
            <div
                className={
                    currentTab == 'Movies' ? 'active-tab' : 'inactive-tab'
                }
            >
                <h1>Movies</h1>
                {movies.map((movie) => (
                    <p>{movie.title}</p>
                ))}
            </div>
            <div
                className={
                    currentTab == 'Openings' ? 'active-tab' : 'inactive-tab'
                }
            >
                <h1>Openings</h1>
                {openings.map((opening) => (
                    <p>{opening.title}</p>
                ))}
            </div>
            <div
                className={
                    currentTab == 'Endings' ? 'active-tab' : 'inactive-tab'
                }
            >
                <h1>Endings</h1>
                {endings.map((ending) => (
                    <p>{ending.title}</p>
                ))}
            </div>
        </>
    ) : (
        <p>Loading...</p>
    );
}
