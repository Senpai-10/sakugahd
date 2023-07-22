import { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css';

function bytesToBase64(bytes: number[] | undefined): string {
    if (bytes == undefined) {
        return '';
    }

    const binString = Array.from(bytes, (x) => String.fromCodePoint(x)).join(
        ''
    );
    return btoa(binString);
}

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

function App() {
    const [shows, setShows] = useState<Show[]>([]);

    useEffect(() => {
        axios
            .get('/api/shows')
            .then((res) => res.data)
            .then((data) => setShows(data));
    }, []);

    return (
        <>
            <div className='shows'>
                {shows.map((show) => (
                    <div
                        className='show'
                        style={{
                            backgroundImage: `url(data:image/png;base64,${bytesToBase64(
                                show.cover
                            )})`,
                        }}
                        key={show.title}
                    >
                        <div className='show-overlay'>
                            <p className='show-title'>{show.title}</p>
                        </div>
                    </div>
                ))}
            </div>
        </>
    );
}

export default App;
