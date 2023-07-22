import { Link, useParams } from 'react-router-dom';
import { useState, useEffect } from 'react';
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

    const encoded_title = encodeURIComponent(title);

    const VideosList = () => {
        if (type == 'episodes') {
            return episodes.map((video) => (
                <Link
                    reloadDocument
                    to={`/shows/${title}/watch/episodes/${video.number}`}
                >
                    <div
                        className={classNames({
                            'active-video': Number(number) == video.number,
                            filler: video.is_filler,
                            video: true,
                        })}
                    >
                        {video.number}
                    </div>
                </Link>
            ));
        } else if (type == 'movies') {
            return movies.map((video) => (
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
                        {video.number}
                    </div>
                </Link>
            ));
        } else if (type == 'openings') {
            return openings.map((video) => (
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
                        {video.number}
                    </div>
                </Link>
            ));
        } else if (type == 'endings') {
            return endings.map((video) => (
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
                        {video.number}
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

    return (
        <>
            <video width='650' controls>
                <source
                    src={`/api/shows/${title}/${type}/${number}`}
                    type='video/mp4'
                />
            </video>
            <div>
                <VideosList />
            </div>
        </>
    );
}
