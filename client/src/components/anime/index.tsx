import { Link } from 'react-router-dom';
import './index.css';

interface Props {
    title: string;
    cover: string;
}

function Anime(props: Props) {
    return (
        <Link to={`/anime/${props.title}`}>
            <div
                className='anime'
                style={{
                    backgroundImage: `url(/api/anime/${encodeURI(
                        props.title
                    )}/cover/${encodeURI(props.cover)})`,
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
