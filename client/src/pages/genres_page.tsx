import { Link } from 'react-router-dom'
import { useState, useEffect } from "react"
import axios from "axios";

export function Genres_page() {
    const [genres, setGenres] = useState<string[]>([])

    useEffect(() => {
        axios.get(`/api/genres`)
            .then((res) => res.data)
            .then((data) => setGenres(data));
    }, [])

    return (
        <>
            {genres.map((genres_name) => (
                <Link to={`/genres/${genres_name}`}><button>{genres_name}</button></Link>
            ))}
        </>
    )
}

