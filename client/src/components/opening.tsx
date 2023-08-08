import { Link } from "react-router-dom"
import { OpeningType } from "../types";

export function Opening(props: { itf: OpeningType }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/openings/${itf.number
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
