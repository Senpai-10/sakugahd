import { Link } from "react-router-dom"
import { MovieType } from "../types";

export function Movie(props: { itf: MovieType }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/movies/${itf.number
                }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className='thumbnail'
            >
                <div className='overlay'>
                    <p className='video-title'>
                        {itf.number} - {itf.title}
                    </p>
                </div>
            </div>
        </Link>
    );
}
