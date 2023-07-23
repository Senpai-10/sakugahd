import { useState, useEffect } from 'react';
import axios from 'axios';
import Show from '../show';
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
    cover: string;
}

function Shows() {
    const [shows, setShows] = useState<Show[]>([]);

    useEffect(() => {
        axios
            .get('/api/shows')
            .then((res) => res.data)
            .then((data) => setShows(data));
    }, []);

    return (
        <div className='shows'>
            {shows.map((show) => (
                <Show key={show.title} title={show.title} cover={show.cover} />
            ))}
        </div>
    );
}

export default Shows;
