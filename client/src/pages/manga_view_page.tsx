import axios from "axios";
import { useEffect, useMemo, useRef, useState } from "react";
import { Link, useParams } from "react-router-dom"
import { ChapterType } from "../types";
import '/public/css/pages/manga_view.css';

export function MangaView_page() {
    const { title } = useParams();

    if (title == undefined) {
        return <h1>No title</h1>
    }

    const [chapters, setChapters] = useState<ChapterType[]>([]);
    const inputRef = useRef(null);
    const [searchQuery, setSearchQuery] = useState("");

    let filtered_list = useMemo(() => {
        return chapters.filter((x) => (
            x.title.toLowerCase().includes(searchQuery) || x.number.includes(searchQuery)
        ))
    }, [chapters, searchQuery])

    useEffect(() => {
        axios.get(`/api/manga/${title}/chapters`)
            .then((r) => r.data)
            .then((data) => setChapters(data))
    }, [])

    return (
        <>
            <input ref={inputRef} onChange={(e) => setSearchQuery(e.target.value)} />
            <div className="chapters-list">
                {
                    filtered_list.map((ch: ChapterType) => {
                        return (
                            <Link key={ch.id} className="ch" to={`/manga/${title}/read/${ch.id}`}>
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
