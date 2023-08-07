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

    const getHideTitle = (): boolean => {
        const v = localStorage.getItem("hide_chapter_title")

        if (v === "true") {
            return true
        } else {
            return false
        }
    }


    const [chapters, setChapters] = useState<ChapterType[]>([]);
    const inputRef = useRef(null);
    const [searchQuery, setSearchQuery] = useState("");
    const [hideTitle, setHideTitle] = useState<boolean>(getHideTitle());

    const save_hideTitle_value = (v: boolean) => {
        localStorage.setItem("hide_chapter_title", String(v))
        setHideTitle(v)
    }

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
            <label>Hide titles</label>
            <input checked={hideTitle} type="checkbox" onChange={() => save_hideTitle_value(!hideTitle)} />
            <div className="chapters-list">
                {
                    filtered_list.map((ch: ChapterType) => {
                        return (
                            <Link key={ch.id} className="ch" to={`/manga/${title}/read/${ch.id}`}>
                                <p className="ch-number">{ch.number}</p>
                                <p className="ch-title">{hideTitle ? "x".repeat(ch.title.length) : ch.title}</p>
                            </Link>
                        )
                    })
                }
            </div>
        </>
    )
}
