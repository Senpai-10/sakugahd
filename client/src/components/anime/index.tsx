import { Link } from 'react-router-dom';
import './index.css';

interface Props {
    title: string;
    cover: string;
}

function Anime(props: Props) {
    let imageUrl = () => {
        if (props.cover == undefined) {
            return "/default_cover.svg"
        }

        return `/api/anime/${encodeURI(
                        props.title
                    )}/cover/${encodeURI(props.cover)}`
    }

    return (
        <Link to={`/anime/${props.title}`}>
            <div
                className='anime'
                style={{
                    backgroundImage: `url(${imageUrl()})`,
                }}
                key={props.title}
            >
                <div className='anime-overlay'>
                    <p className='anime-title'>{props.title}</p>
                </div>
            </div>
        </Link>
    );
}

export default Anime;
