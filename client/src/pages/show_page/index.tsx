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
    const [show, setShow] = useState<Show>();
    const [episodes, setEpisodes] = useState<Episode[]>([]);

    useEffect(() => {
        axios
            .get(`/api/shows/${encoded_title}`)
            .then((res) => res.data)
            .then((data) => setShow(data));
        axios
            .get(`/api/shows/${encoded_title}/episodes`)
            .then((res) => res.data)
            .then((data) => setEpisodes(data));
    }, []);

    return show ? (
        <>
            show: {show.title}
            <h1>Episodes</h1>
            {episodes.map((episode) => (
                <Episode key={episode.id} ep={episode} />
            ))}
        </>
    ) : (
        <p>Loading...</p>
    );
}
