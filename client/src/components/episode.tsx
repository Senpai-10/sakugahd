import { Link } from "react-router-dom"
import { EpisodeType } from "../types";
import classNames from "classnames";

export function Episode(props: { itf: EpisodeType }) {
    const itf = props.itf;
    const image = encodeURIComponent(itf.thumbnail_file_name);

    return (
        <Link
            to={`/anime/${encodeURIComponent(itf.anime_title)}/watch/episodes/${itf.number
                }`}
        >
            <div
                style={{
                    backgroundImage: `url(/api/thumbnail/${image})`,
                }}
                className={classNames({
                    thumbnail: true,
                    'filler-video': itf.is_filler,
                })}
            >
                <div className='overlay'>
                    <p className='video-title'>{itf.title}</p>
                    {
                        itf.is_filler ? <p className="video-filler">Filler</p> : null
                    }
                </div>
            </div>
        </Link>
    );
}
