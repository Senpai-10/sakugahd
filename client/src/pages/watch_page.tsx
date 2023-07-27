import { Link, useLocation, useParams } from 'react-router-dom';
import { useState, useEffect, useRef, useMemo } from 'react';
import classNames from 'classnames';
import axios from 'axios';
import '/public/css/pages/watch.css';
import { EpisodeType, MovieType, OpeningType, EndingType } from "../types"

export function Watch_page() {
    const { title, type, number } = useParams();

    if (title == undefined || type == undefined || number == undefined) {
        return <h1>missing params</h1>;
    }

    const [episodes, setEpisodes] = useState<EpisodeType[]>([]);
    const [movies, setMovies] = useState<MovieType[]>([]);
    const [openings, setOpenings] = useState<OpeningType[]>([]);
    const [endings, setEndings] = useState<EndingType[]>([]);
    const inputRef = useRef(null);
    const [searchQuery, setSearchQuery] = useState("");
    const [hideFillers, setHideFillers] = useState(false);
    const videoProgressKey = `videoProgress_${title}_${type}_${number}`;

    const filteredEpisodes = useMemo(() => {
        return episodes.filter((video) => {
            if (hideFillers === true && video.is_filler === true && video.number != Number(number)) {
                return false;
            }

            return video.title.toLowerCase().includes(searchQuery);
        });
    }, [episodes, searchQuery, hideFillers]);

    const filteredMovies = useMemo(() => {
        return movies.filter((video) => {
            return video.title.toLowerCase().includes(searchQuery) || video.number.toString().includes(searchQuery);
        });
    }, [movies, searchQuery]);

    const filteredOpenings = useMemo(() => {
        return openings.filter((video) => {
            return video.title.toLowerCase().includes(searchQuery) || video.number.toString().includes(searchQuery);
        });
    }, [openings, searchQuery]);

    const filteredEndings = useMemo(() => {
        return endings.filter((video) => {
            return video.title.toLowerCase().includes(searchQuery) || video.number.toString().includes(searchQuery);
        });
    }, [endings, searchQuery]);

    const encoded_title = encodeURIComponent(title);

    const scrollToActiveVideo = () => {
        const section = document.querySelector('.active-video');
        if (section) {
            section.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }
    };

    const VideosList = () => {
        if (type == 'episodes') {
            return filteredEpisodes.map((video) => (
                <Link
                    reloadDocument
                    to={`/anime/${title}/watch/episodes/${video.number}`}
                >
                    <div
                        className={classNames({
                            'active-video': Number(number) == video.number,
                            video: true,
                            filler: video.is_filler,
                        })}
                    >
                        Episode {video.number}{' '}
                        {video.is_filler ? '(Filler)' : ''}
                    </div>
                </Link>
            ));
        } else if (type == 'movies') {
            return filteredMovies.map((video) => (
                <Link
                    reloadDocument
                    to={`/anime/${title}/watch/movies/${video.number}`}
                >
                    <div
                        className={classNames({
                            'active-video': Number(number) == video.number,
                            video: true,
                        })}
                    >
                        Movie {video.number} - {video.title}
                    </div>
                </Link>
            ));
        } else if (type == 'openings') {
            return filteredOpenings.map((video) => (
                <Link
                    reloadDocument
                    to={`/anime/${title}/watch/openings/${video.number}`}
                >
                    <div
                        className={classNames({
                            'active-video': Number(number) == video.number,
                            video: true,
                        })}
                    >
                        Opening {video.number} - {video.title}
                    </div>
                </Link>
            ));
        } else if (type == 'endings') {
            return filteredEndings.map((video) => (
                <Link
                    reloadDocument
                    to={`/anime/${title}/watch/endings/${video.number}`}
                >
                    <div
                        className={classNames({
                            'active-video': Number(number) == video.number,
                            video: true,
                        })}
                    >
                        Ending {video.number} - {video.title}
                    </div>
                </Link>
            ));
        }
    };

    useEffect(() => {
        if (type == 'episodes') {
            axios
                .get(`/api/anime/${encoded_title}/episodes`)
                .then((res) => res.data)
                .then((data) => setEpisodes(data));
        } else if (type == 'movies') {
            axios
                .get(`/api/anime/${encoded_title}/movies`)
                .then((res) => res.data)
                .then((data) => setMovies(data));
        } else if (type == 'openings') {
            axios
                .get(`/api/anime/${encoded_title}/openings`)
                .then((res) => res.data)
                .then((data) => setOpenings(data));
        } else if (type == 'endings') {
            axios
                .get(`/api/anime/${encoded_title}/endings`)
                .then((res) => res.data)
                .then((data) => setEndings(data));
        }
    }, []);

    const location = useLocation();

    useEffect(() => {
        setTimeout(() => {
            scrollToActiveVideo();
        }, 500);
    }, [location]);

    const saveCurrnetTime = (e: any) => {
        if (type == "openings" || type == "endings")
            return

        localStorage.setItem(videoProgressKey, e.target.currentTime)
    }

    const loadCurrentTime = (): number => {
        let time = localStorage.getItem(videoProgressKey)

        if (time != null) {
            return Number(time)
        }

        return 0
    }

    return (
        <>
            <div className='topnav'>
                <Link reloadDocument to={`/`} className="link">Home</Link>
                <Link reloadDocument to={`/anime/${title}`} className="link">Anime</Link>
                <Link reloadDocument to={`/anime/${title}/watch/episodes/1`} className="link">Episodes</Link>
                <Link reloadDocument to={`/anime/${title}/watch/movies/1`} className="link">Movies</Link>
                <Link reloadDocument to={`/anime/${title}/watch/openings/1`} className="link">Openings</Link>
                <Link reloadDocument to={`/anime/${title}/watch/endings/1`} className="link">Endings</Link>
            </div>
            <video onTimeUpdate={saveCurrnetTime} onLoadedData={(e) => {e.currentTarget.volume = 0.25; e.currentTarget.currentTime = loadCurrentTime()}} width='850' preload='metadata' controls>
                <source
                    src={`/api/anime/${title}/${type}/${number}`}
                    type='video/mp4'
                />
            </video>
            <div className="videos-list-container">
                <input
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    ref={inputRef}
                    type='text'
                    placeholder='Search'
                />
                <label>Hide Fillers</label>
                <input
                    type='checkbox'
                    onChange={() => setHideFillers(!hideFillers)}
                    checked={hideFillers}
                />

                <div className='videos-list'>
                    <VideosList />
                </div>
            </div>
        </>
    );
}
