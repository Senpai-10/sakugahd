import { useParams, Link } from 'react-router-dom';
import { useState, useEffect } from 'react';
import classNames from 'classnames';
import axios from 'axios';
import './index.css';
import { AnimeType, EpisodeType, MovieType, OpeningType, EndingType } from "../../types"

function Episode(props: { itf: EpisodeType }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/episodes/${
                itf.number
            }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className={classNames({
                    thumbnail: true,
                    'filler-video': itf.is_filler,
                })}
            >
                <div className='overlay'>
                    <p className='video-title'>{itf.title}</p>
                </div>
            </div>
        </Link>
    );
}

function Movie(props: { itf: MovieType }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/movies/${
                itf.number
            }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className='thumbnail'
            >
                <div className='overlay'>
                    <p className='video-title'>{itf.number} - {itf.title}</p>
                </div>
            </div>
        </Link>
    );
}

function Opening(props: { itf: OpeningType }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/openings/${
                itf.number
            }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className='thumbnail'
            >
                <div className='overlay'>
                    <p className='video-title'>{itf.number} - {itf.title}</p>
                </div>
            </div>
        </Link>
    );
}

function Ending(props: { itf: EndingType }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/endings/${
                itf.number
            }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className='thumbnail'
            >
                <div className='overlay'>
                    <p className='video-title'>{itf.number} - {itf.title}</p>
                </div>
            </div>
        </Link>
    );
}

type Tabs = 'Episodes' | 'Movies' | 'Openings' | 'Endings';

export function Anime_page() {
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
    const [anime, setAnime] = useState<AnimeType>();
    const [episodes, setEpisodes] = useState<EpisodeType[]>([]);
    const [movies, setMovies] = useState<MovieType[]>([]);
    const [openings, setOpenings] = useState<OpeningType[]>([]);
    const [endings, setEndings] = useState<EndingType[]>([]);

    useEffect(() => {
        axios
            .get(`/api/anime/${encoded_title}`)
            .then((res) => res.data)
            .then((data) => setAnime(data));
        axios
            .get(`/api/anime/${encoded_title}/episodes`)
            .then((res) => res.data)
            .then((data) => setEpisodes(data));
        axios
            .get(`/api/anime/${encoded_title}/movies`)
            .then((res) => res.data)
            .then((data) => setMovies(data));
        axios
            .get(`/api/anime/${encoded_title}/openings`)
            .then((res) => res.data)
            .then((data) => setOpenings(data));
        axios
            .get(`/api/anime/${encoded_title}/endings`)
            .then((res) => res.data)
            .then((data) => setEndings(data));
    }, []);

    return anime ? (
        <>
            anime: {anime.title}
            <button
                className={classNames({
                    tab: true,
                    'active-tab-button': currentTab == 'Episodes',
                })}
                onClick={() => setCurrentTab('Episodes')}
            >
                Episodes
            </button>
            <button
                className={classNames({
                    tab: true,
                    'active-tab-button': currentTab == 'Movies',
                })}
                onClick={() => setCurrentTab('Movies')}
            >
                Movies
            </button>
            <button
                className={classNames({
                    tab: true,
                    'active-tab-button': currentTab == 'Openings',
                })}
                onClick={() => setCurrentTab('Openings')}
            >
                Openings
            </button>
            <button
                className={classNames({
                    tab: true,
                    'active-tab-button': currentTab == 'Endings',
                })}
                onClick={() => setCurrentTab('Endings')}
            >
                Endings
            </button>
            <div
                className={
                    currentTab == 'Episodes' ? 'active-tab' : 'inactive-tab'
                }
            >
                {episodes.length == 0 ? (
                    <p>nothing found</p>
                ) : (
                    <div className='videos'>
                        {episodes.map((episode) => (
                            <Episode key={episode.id} itf={episode} />
                        ))}
                    </div>
                )}
            </div>
            <div
                className={
                    currentTab == 'Movies' ? 'active-tab' : 'inactive-tab'
                }
            >
                {movies.length == 0 ? (
                    <p>nothing found</p>
                ) : (
                    <div className='videos'>
                        {movies.map((movie) => (
                            <Movie itf={movie} />
                        ))}
                    </div>
                )}
            </div>
            <div
                className={
                    currentTab == 'Openings' ? 'active-tab' : 'inactive-tab'
                }
            >
                {openings.length == 0 ? (
                    <p>nothing found</p>
                ) : (
                    <div className='videos'>
                        {openings.map((opening) => (
                            <Opening itf={opening} />
                        ))}
                    </div>
                )}
            </div>
            <div
                className={
                    currentTab == 'Endings' ? 'active-tab' : 'inactive-tab'
                }
            >
                {endings.length == 0 ? (
                    <p>nothing found</p>
                ) : (
                    <div className='videos'>
                        {endings.map((ending) => (
                            <Ending itf={ending} />
                        ))}
                    </div>
                )}
            </div>
        </>
    ) : (
        <p>Loading...</p>
    );
}
