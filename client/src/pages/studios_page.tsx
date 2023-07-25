import {Link} from 'react-router-dom'
import { useState, useEffect } from "react"
import axios from "axios";

export function Studios_page() {
    const [studios, setStudios] = useState<string[]>([])

    useEffect(() => {
        axios.get(`/api/studios`)
            .then((res) => res.data)
            .then((data) => setStudios(data));
    }, [])

    return (
        <>
            {studios.map((studio_name) => (
                <Link to={`/studios/${studio_name}`}><button>{studio_name}</button></Link>
            ))}
        </>
    )
}

