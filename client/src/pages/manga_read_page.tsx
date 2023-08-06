import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom"
import { PageType } from "../types";

export function MangaRead_page() {
    const { title, id } = useParams();

    if (title == undefined || id == undefined) {
        return <h1>No chapter id</h1>
    }

    const [pages, setPages] = useState<PageType[]>();

    useEffect(() => {
        axios.get(`/api/manga/${title}/chapters/${id}`)
            .then((r) => r.data)
            .then((data) => setPages(data))
    }, [])

    if (pages == undefined) {
        return <h1>Loading..</h1>
    }

    return (
        <>
            {
                pages.map((page) => {
                    return (<div>
                        <img src={`/api/page/${page.id}`} />
                    </div>)
                })
            }
        </>
    )
}
