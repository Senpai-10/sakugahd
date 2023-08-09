import axios from "axios";
import { useEffect, useRef, useState } from "react";
import { useLocation, useNavigate, useParams } from "react-router-dom"
import { ChapterType, PageType } from "../types";
import "/public/css/pages/manga_read.css"

interface Data {
    pages: PageType[]
    current_chapter: ChapterType
    prev_chapter?: ChapterType
    next_chapter?: ChapterType
}

export function MangaRead_page() {
    const { title, id } = useParams();

    if (title == undefined || id == undefined) {
        return <h1>No chapter id</h1>
    }

    const navigate = useNavigate();
    const [data, setData] = useState<Data>();
    const [cursor, setCursor] = useState<number>(0);
    const [currentPage, setCurrentPage] = useState<PageType>()
    const imgRef = useRef(null)

    const location = useLocation()

    useEffect(() => {
        axios.get(`/api/manga/${title}/chapters/${id}`)
            .then((r) => r.data)
            .then((data) => {
                setData(data)
                setCursor(0)
                setCurrentPage(data.pages[cursor])
            })
    }, [location])

    useEffect(() => {
        if (data) {
            setCurrentPage(data.pages[cursor])
        }
    }, [cursor])

    const handleKeyPress = (event: any) => {
        if (data) {
            if (event.shiftKey === true && event.key == "ArrowLeft") {
                to_prev_chapter()
            }
            else if (event.shiftKey === true && event.key == "ArrowRight") {
                to_next_chapter()
            }
            else if (event.key === "ArrowLeft") {
                if (cursor == 0) {
                    to_prev_chapter()
                    return
                }

                setCursor(cursor - 1)
            }
            else if (event.key === "ArrowRight") {
                if (cursor == data.pages.length - 1) {
                    to_next_chapter()
                    return
                }

                setCursor(cursor + 1)
            }
        }
    }

    useEffect(() => {
        // attach the event listener
        document.addEventListener('keydown', handleKeyPress);

        // remove the event listener
        return () => {
            document.removeEventListener('keydown', handleKeyPress);
        };
    }, [handleKeyPress]);


    if (data == undefined || currentPage == undefined) {
        return <h1>Loading..</h1>
    }

    const to_prev_chapter = () => {
        if (data.prev_chapter) {
            navigate(`/manga/${title}/read/${data.prev_chapter.id}`)
        }
    }
    const to_next_chapter = () => {
        if (data.next_chapter) {
            navigate(`/manga/${title}/read/${data.next_chapter.id}`)
        }
    }

    return (
        <div>
            <div className="info-navbar">
                <span>Chapter {data.current_chapter.number} - {data.current_chapter.title}</span>
                <span>Page {currentPage.number}/{data.pages.length}</span>

                <div className="btn-container">
                    <button onClick={() => navigate(`/`)}>Home</button>
                    <button onClick={() => navigate(`/manga/${title}`)}>Chapters list</button>
                    <button onClick={to_prev_chapter} disabled={data.prev_chapter == undefined}>Prev chapter</button>
                    <button onClick={to_next_chapter} disabled={data.next_chapter == undefined}>Next chapter</button>
                </div>
            </div>
            <div className="page-img-container">
                <img ref={imgRef} className="page-img" src={`/api/page/${currentPage.id}`} />
            </div>
        </div>
    )
}
