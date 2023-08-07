import axios from "axios";
import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom"
import { ChapterType } from "../types";
import '/public/css/pages/manga_view.css';

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
            <div className="chapters-list">
                {
                    chapters.map((ch: ChapterType) => {
                        return (
                            <Link className="ch" to={`/manga/${title}/read/${ch.id}`}>
                                <p className="ch-number">{ch.number}</p>
                                <p className="ch-title">{ch.title}</p>
                            </Link>
                        )
                    })
                }
            </div>
        </>
    )
}
