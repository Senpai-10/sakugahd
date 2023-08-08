import { useParams, Link } from 'react-router-dom';
import { useState, useEffect, useMemo } from 'react';
import classNames from 'classnames';
import axios from 'axios';
import '/public/css/pages/anime.css';
import {
    AnimeType,
    EpisodeType,
    MovieType,
    OpeningType,
    EndingType,
} from '../types';

function Episode(props: { itf: EpisodeType }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/episodes/${itf.number
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
                    {
                        itf.is_filler ? <p className="video-filler">Filler</p> : null
                    }
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
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/movies/${itf.number
                }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className='thumbnail'
            >
                <div className='overlay'>
                    <p className='video-title'>
                        {itf.number} - {itf.title}
                    </p>
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
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/openings/${itf.number
                }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className='thumbnail'
            >
                <div className='overlay'>
                    <p className='video-title'>
                        {itf.number} - {itf.title}
                    </p>
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
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/endings/${itf.number
                }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className='thumbnail'
            >
                <div className='overlay'>
                    <p className='video-title'>
                        {itf.number} - {itf.title}
                    </p>
                </div>
            </div>
        </Link>
    );
}

type Tabs = 'Episodes' | 'Movies' | 'Openings' | 'Endings';

export function AnimeView_page() {
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
    const [animeStudios, setAnimeStudios] = useState<string[]>([]);
    const [animeGenres, setAnimeGenres] = useState<string[]>([]);
    const [anime, setAnime] = useState<AnimeType>();
    const [episodes, setEpisodes] = useState<EpisodeType[]>([]);
    const [movies, setMovies] = useState<MovieType[]>([]);
    const [openings, setOpenings] = useState<OpeningType[]>([]);
    const [endings, setEndings] = useState<EndingType[]>([]);
    const [hideFillers, setHideFillers] = useState(false);

    const filteredEpisodes = useMemo(() => {
        return episodes.filter((video) => {
            if (hideFillers === true && video.is_filler === true) {
                return false;
            }

            return true
        });
    }, [episodes, hideFillers]);

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
        axios
            .get(`/api/anime/${encoded_title}/studios`)
            .then((res) => res.data)
            .then((data) => setAnimeStudios(data));
        axios
            .get(`/api/anime/${encoded_title}/genres`)
            .then((res) => res.data)
            .then((data) => setAnimeGenres(data));

    }, []);

    return anime ? (
        <>
            <div className='anime-description'>
                <img
                    src={`/api/anime/${encodeURI(
                        anime.title
                    )}/cover/${encodeURI(anime.cover)}`}
                />

                <div>
                    <h2>{anime.title}</h2>
                    <p>{anime.description}</p>
                    <div className='anime-info'>
                        <div>
                            <h3 className="info-title">Format</h3>
                            <p className="info-value">{anime.format ? anime.format : 'Unknown'}</p>
                        </div>
                        <div>
                            <h3 className="info-title">Status</h3>
                            <p className="info-value">{anime.status ? anime.status : "Unknown"}</p>
                        </div>
                        <div>
                            <h3 className="info-title">Season</h3>
                            <p className="info-value">{anime.season && anime.season_year ? `${anime.season} - ${anime.season_year}` : "Unknown"}</p>
                        </div>
                        <div>
                            <h3 className="info-title">Studios</h3>
                            {animeStudios.map(studio => (
                                <Link to={`/studios/${studio}`} className="info-value info-value-list">{studio}</Link>
                            ))}
                        </div>
                        <div>
                            <h3 className="info-title">Genres</h3>
                            {animeGenres.map(genre => (
                                <Link to={`/genres/${genre}`} className="info-value info-value-list">{genre}</Link>
                            ))}
                        </div>
                    </div>
                </div>
            </div>
            <div className='content'>
                <button
                    className={classNames({
                        tab: true,
                        'active-tab-button': currentTab == 'Episodes',
                    })}
                    onClick={() => setCurrentTab('Episodes')}
                    disabled={filteredEpisodes.length == 0 ? true : false}
                >
                    Episodes {filteredEpisodes.length}
                </button>
                <button
                    className={classNames({
                        tab: true,
                        'active-tab-button': currentTab == 'Movies',
                    })}
                    onClick={() => setCurrentTab('Movies')}
                    disabled={movies.length == 0 ? true : false}
                >
                    Movies {movies.length}
                </button>
                <button
                    className={classNames({
                        tab: true,
                        'active-tab-button': currentTab == 'Openings',
                    })}
                    onClick={() => setCurrentTab('Openings')}
                    disabled={openings.length == 0 ? true : false}
                >
                    Openings {openings.length}
                </button>
                <button
                    className={classNames({
                        tab: true,
                        'active-tab-button': currentTab == 'Endings',
                    })}
                    onClick={() => setCurrentTab('Endings')}
                    disabled={endings.length == 0 ? true : false}
                >
                    Endings {endings.length}
                </button>
                {currentTab == "Episodes" ?
                    <>
                        <label>Hide Fillers</label>
                        <input
                            type='checkbox'
                            onChange={() => setHideFillers(!hideFillers)}
                            checked={hideFillers}
                        />
                    </>
                    : null
                }
                <div
                    className={
                        currentTab == 'Episodes' ? 'active-tab' : 'inactive-tab'
                    }
                >
                    {filteredEpisodes.length == 0 ? (
                        <p>nothing found</p>
                    ) : (
                        <div className='videos'>
                            {filteredEpisodes.map((episode) => (
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
            </div>
        </>
    ) : (
        <p>Loading...</p>
    );
}
