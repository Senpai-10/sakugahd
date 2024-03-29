import { Link, useLocation, useParams } from 'react-router-dom';
import { useState, useEffect, useRef, useMemo } from 'react';
import classNames from 'classnames';
import axios from 'axios';
import '../css/pages/watch.css';
import { EpisodeType, MovieType, OpeningType, EndingType } from '../types';

interface VideoProgress {
    [key: string]: {
        [key: string]: { video_number: number; progress: number }[];
    };
}

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
    const [searchQuery, setSearchQuery] = useState('');
    const [hideFillers, setHideFillers] = useState(false);
    const videoRef = useRef<HTMLVideoElement>(null);
    const [videoUrl, setVideoUrl] = useState(`/api/anime/${title}/${type}/${number}`)
    const video_progress_key = "video_progress";

    const filteredEpisodes = useMemo(() => {
        return episodes.filter((video) => {
            if (
                hideFillers === true &&
                video.is_filler === true &&
                video.number != Number(number)
            ) {
                return false;
            }

            return video.title.toLowerCase().includes(searchQuery);
        });
    }, [episodes, searchQuery, hideFillers]);

    const filteredMovies = useMemo(() => {
        return movies.filter((video) => {
            return (
                video.title.toLowerCase().includes(searchQuery) ||
                video.number.toString().includes(searchQuery)
            );
        });
    }, [movies, searchQuery]);

    const filteredOpenings = useMemo(() => {
        return openings.filter((video) => {
            return (
                video.title.toLowerCase().includes(searchQuery) ||
                video.number.toString().includes(searchQuery)
            );
        });
    }, [openings, searchQuery]);

    const filteredEndings = useMemo(() => {
        return endings.filter((video) => {
            return (
                video.title.toLowerCase().includes(searchQuery) ||
                video.number.toString().includes(searchQuery)
            );
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
        setVideoUrl(`/api/anime/${title}/${type}/${number}`)
    }, [location]);

    useEffect(() => {
        if (videoRef != null) {
            videoRef.current?.load();
        }
    }, [videoUrl])

    const saveCurrnetTime = (e: any) => {
        if (type == 'openings' || type == 'endings') return;

        let string_value: string = localStorage.getItem(video_progress_key) || "{}"

        let video_progress: VideoProgress = JSON.parse(string_value)

        if (video_progress[title] == undefined) {
            video_progress[title] = {
                [type]: [{ video_number: Number(number), progress: e.target.currentTime }],
            }
        } else {
            let index = video_progress[title][type].findIndex((x) => x.video_number == Number(number))

            if (index == -1) {
                video_progress[title][type].push({ video_number: Number(number), progress: e.target.currentTime })
            } else {
                video_progress[title][type][index].progress = e.target.currentTime
            }
        }

        localStorage.setItem(video_progress_key, JSON.stringify(video_progress));
    };

    const loadCurrentTime = (): number => {
        if (type == "openings" || type == "endings") return 0

        let value_string = localStorage.getItem(video_progress_key)

        if (value_string == null) {
            return 0
        }

        let video_progress: VideoProgress = JSON.parse(value_string)

        let progress = video_progress[title][type].find((x) => x.video_number == Number(number))?.progress || 0

        return progress
    };

    function saveVolumeLevel(e: any) {
        localStorage.setItem("video_player_volume", e.currentTarget.volume)
    }

    function loadVolumeLevel(): number {
        let volume = localStorage.getItem("video_player_volume")

        return Number(volume) || 0.50
    }

    return (
        <>
            <div className='topnav'>
                <Link to={`/`} className='link'>
                    Home
                </Link>
                <Link to={`/anime/${title}`} className='link'>
                    Anime
                </Link>
                <Link
                    to={`/anime/${title}/watch/episodes/1`}
                    className='link'
                >
                    Episodes
                </Link>
                <Link
                    to={`/anime/${title}/watch/movies/1`}
                    className='link'
                >
                    Movies
                </Link>
                <Link
                    to={`/anime/${title}/watch/openings/1`}
                    className='link'
                >
                    Openings
                </Link>
                <Link
                    to={`/anime/${title}/watch/endings/1`}
                    className='link'
                >
                    Endings
                </Link>
            </div>
            <video
                ref={videoRef}
                onTimeUpdate={saveCurrnetTime}
                onVolumeChange={saveVolumeLevel}
                onLoadedData={(e) => {
                    e.currentTarget.volume = loadVolumeLevel();
                    e.currentTarget.currentTime = loadCurrentTime();
                }}
                width='850'
                preload='metadata'
                controls
            >
                <source src={videoUrl} type="video/mp4" />
            </video>
            <div className='videos-list-container'>
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
