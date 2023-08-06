import axios from "axios";
import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom"
import { ChapterType } from "../types";

export function MangaView_page() {
    const { title } = useParams();

    if (title == undefined) {
        return <h1>No title</h1>
    }

    const [chapters, setChapters] = useState<ChapterType[]>();

    useEffect(() => {
        axios.get(`/api/manga/${title}/chapters`)
            .then((r) => r.data)
            .then((data) => setChapters(data))
    }, [])

    if (chapters == undefined) {
        return <h1>Loading..</h1>
    }

    return (
        <>
            {
                chapters.map((ch: ChapterType) => {
                    return (<Link to={`/manga/${title}/read/${ch.id}`}><h1>{ch.title}</h1></Link>)
                })
            }
        </>
    )
}
