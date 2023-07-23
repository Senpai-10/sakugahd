import { useState, useEffect } from 'react';
import axios from 'axios';
import Show from '../show';
import './index.css';
import { ShowType } from "../../types"

function Shows() {
    const [shows, setShows] = useState<ShowType[]>([]);

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
