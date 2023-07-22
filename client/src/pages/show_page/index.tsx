import { useParams, Link } from 'react-router-dom';
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
    thumbnail_file_name: string;
    file_name: string;
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

function Episode(props: { itf: Episode }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link to={`/shows/${encodeURIComponent(itf.show_title)}/watch/episodes/${itf.number}`}>
            <img loading="lazy" className='thumbnail' src={`/api/thumbnail/${image}`} />
            <p>title: {itf.title}</p>
            <p>show title: {itf.show_title}</p>
            <p>number: {itf.number}</p>
            <p>is filler: {itf.is_filler}</p>
            <p>file name: {itf.file_name}</p>
        </Link>
    );
}

function Movie(props: { itf: Movie }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link to={`/shows/${encodeURIComponent(itf.show_title)}/watch/movies/${itf.number}`}>
            <img className='thumbnail' src={`/api/thumbnail/${image}`} />
            <p>title: {itf.title}</p>
            <p>show title: {itf.show_title}</p>
            <p>number: {itf.number}</p>
            <p>watch after: {itf.watch_after}</p>
            <p>file name: {itf.file_name}</p>
        </Link>
    );
}

function Opening(props: { itf: Opening }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link to={`/shows/${encodeURIComponent(itf.show_title)}/watch/openings/${itf.number}`}>
            <img className='thumbnail' src={`/api/thumbnail/${image}`} />
            <p>title: {itf.title}</p>
            <p>show title: {itf.show_title}</p>
            <p>number: {itf.number}</p>
            <p>file name: {itf.file_name}</p>
        </Link>
    );
}

function Ending(props: { itf: Ending }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link to={`/shows/${encodeURIComponent(itf.show_title)}/watch/endings/${itf.number}`}>
            <img className='thumbnail' src={`/api/thumbnail/${image}`} />
            <p>title: {itf.title}</p>
            <p>show title: {itf.show_title}</p>
            <p>number: {itf.number}</p>
            <p>file name: {itf.file_name}</p>
        </Link>
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
            show: {show.title}
            <button onClick={() => setCurrentTab('Episodes')}>Episodes</button>
            <button onClick={() => setCurrentTab('Movies')}>Movies</button>
            <button onClick={() => setCurrentTab('Openings')}>Openings</button>
            <button onClick={() => setCurrentTab('Endings')}>Endings</button>
            <div
                className={
                    currentTab == 'Episodes' ? 'active-tab' : 'inactive-tab'
                }
            >
                <h1>Episodes</h1>
                {episodes.map((episode) => (
                    <Episode key={episode.id} itf={episode} />
                ))}
            </div>
            <div
                className={
                    currentTab == 'Movies' ? 'active-tab' : 'inactive-tab'
                }
            >
                <h1>Movies</h1>
                {movies.map((movie) => (
                    <Movie itf={movie} />
                ))}
            </div>
            <div
                className={
                    currentTab == 'Openings' ? 'active-tab' : 'inactive-tab'
                }
            >
                <h1>Openings</h1>
                {openings.map((opening) => (
                    <Opening itf={opening} />
                ))}
            </div>
            <div
                className={
                    currentTab == 'Endings' ? 'active-tab' : 'inactive-tab'
                }
            >
                <h1>Endings</h1>
                {endings.map((ending) => (
                    <Ending itf={ending} />
                ))}
            </div>
        </>
    ) : (
        <p>Loading...</p>
    );
}
