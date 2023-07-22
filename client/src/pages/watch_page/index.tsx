import { Link, useLocation, useParams } from 'react-router-dom';
import { useState, useEffect, useRef, useMemo } from 'react';
import classNames from 'classnames';
import axios from 'axios';
import './index.css';

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

export function Watch_page() {
    const { title, type, number } = useParams();

    if (title == undefined || type == undefined || number == undefined) {
        return <h1>missing params</h1>;
    }

    const [episodes, setEpisodes] = useState<Episode[]>([]);
    const [movies, setMovies] = useState<Movie[]>([]);
    const [openings, setOpenings] = useState<Opening[]>([]);
    const [endings, setEndings] = useState<Ending[]>([]);
    const inputRef = useRef(null);
    const [searchQuery, setSearchQuery] = useState(0);
    const [hideFillers, setHideFillers] = useState(false);

    const filteredEpisodes = useMemo(() => {
        return episodes.filter((video) => {
            if (hideFillers === true && video.is_filler === true) {
                return false;
            }

            if (searchQuery == 0) return true;

            return video.number == searchQuery;
        });
    }, [episodes, searchQuery, hideFillers]);

    const filteredMovies = useMemo(() => {
        return movies.filter((video) => {
            if (searchQuery == 0) return true;

            return video.number == searchQuery;
        });
    }, [movies, searchQuery]);

    const filteredOpenings = useMemo(() => {
        return openings.filter((video) => {
            if (searchQuery == 0) return true;

            return video.number == searchQuery;
        });
    }, [openings, searchQuery]);

    const filteredEndings = useMemo(() => {
        return endings.filter((video) => {
            if (searchQuery == 0) return true;

            return video.number == searchQuery;
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
                    to={`/shows/${title}/watch/episodes/${video.number}`}
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
                    to={`/shows/${title}/watch/movies/${video.number}`}
                >
                    <div
                        className={classNames({
                            'active-video': Number(number) == video.number,
                            video: true,
                        })}
                    >
                        Movie {video.number}
                    </div>
                </Link>
            ));
        } else if (type == 'openings') {
            return filteredOpenings.map((video) => (
                <Link
                    reloadDocument
                    to={`/shows/${title}/watch/openings/${video.number}`}
                >
                    <div
                        className={classNames({
                            'active-video': Number(number) == video.number,
                            video: true,
                        })}
                    >
                        Opening {video.number}
                    </div>
                </Link>
            ));
        } else if (type == 'endings') {
            return filteredEndings.map((video) => (
                <Link
                    reloadDocument
                    to={`/shows/${title}/watch/endings/${video.number}`}
                >
                    <div
                        className={classNames({
                            'active-video': Number(number) == video.number,
                            video: true,
                        })}
                    >
                        Ending {video.number}
                    </div>
                </Link>
            ));
        }
    };

    useEffect(() => {
        if (type == 'episodes') {
            axios
                .get(`/api/shows/${encoded_title}/episodes`)
                .then((res) => res.data)
                .then((data) => setEpisodes(data));
        } else if (type == 'movies') {
            axios
                .get(`/api/shows/${encoded_title}/movies`)
                .then((res) => res.data)
                .then((data) => setMovies(data));
        } else if (type == 'openings') {
            axios
                .get(`/api/shows/${encoded_title}/openings`)
                .then((res) => res.data)
                .then((data) => setOpenings(data));
        } else if (type == 'endings') {
            axios
                .get(`/api/shows/${encoded_title}/endings`)
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

    return (
        <>
            <video width='650' controls>
                <source
                    src={`/api/shows/${title}/${type}/${number}`}
                    type='video/mp4'
                />
            </video>
            <div className='videos-list'>
                <input
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(Number(e.target.value))}
                    ref={inputRef}
                    type='number'
                    placeholder='Search'
                />
                <label>Hide Fillers</label>
                <input
                    type='checkbox'
                    onChange={() => setHideFillers(!hideFillers)}
                    checked={hideFillers}
                />
                <VideosList />
            </div>
        </>
    );
}
